/// https://www.postgresql.org/docs/current/protocol-replication.html

use crate::conn::tuples::{Tuples, Tuple};

mod cmd;
mod wal_level;
mod msg;

use cmd::Cmd;
use wal_level::WalLevel;
use msg::CopyMessage;

pub mod system_id {
    use super::Tuple;

    pub fn systemid<'a>(tuple: &'a Tuple<'a, '_>) -> &'a std::ffi::CStr {
        tuple.get_attr_value(0)
    }

    pub fn timeline<'a>(tuple: &'a Tuple<'a, '_>) -> &'a std::ffi::CStr {
        tuple.get_attr_value(1)
    }

    pub fn xlogpos<'a>(tuple: &'a Tuple<'a, '_>) -> &'a std::ffi::CStr {
        tuple.get_attr_value(2)
    }
    pub fn dbname<'a>(tuple: &'a Tuple<'a, '_>) -> &'a std::ffi::CStr {
        tuple.get_attr_value(3)
    }
}

#[derive(Debug)]
pub struct Replication {
    connection: crate::conn::Connection,
    slot_name: String,
}

impl Replication {
    pub fn new(connection: crate::conn::Connection, slot_name: String) -> Self {
        Self {
            connection,
            slot_name,
        }
    }

    pub fn get_wal_level(&self) -> Result<WalLevel, String> {
        let result = unsafe {
            match self.connection.exec(c"SHOW wal_level;") {
                Ok(r) => r,
                Err(msg) => return Err(msg),
            }
        };

        assert!(result.num_attrs == 1);
        assert!(result.num_tuples == 1);

        Ok(WalLevel::from_bytes(result.get_attr_value(0, 0).to_bytes()))
    }

    pub fn create_repl_slot<'r, 's: 'r>(&'s self) -> Result<Tuples<'r>, String> {
        let result = unsafe {
            let cmd = format!("CREATE_REPLICATION_SLOT {} TEMPORARY LOGICAL pgoutput;\0", self.slot_name);
            match self.connection.exec(std::ffi::CStr::from_bytes_with_nul(cmd.as_bytes()).unwrap()) {
                Ok(r) => r,
                Err(msg) => return Err(msg),
            }
        };

        assert!(result.num_attrs == 4);
        assert!(result.num_tuples == 1);

        Ok(result)
    }

    pub fn get_repl_slots<'r, 's: 'r>(&'s self) -> Result<Tuples<'r>, String> {
        let result = unsafe {
            let cmd = format!("SELECT * FROM pg_replication_slots WHERE slot_name = '{}'\0", self.slot_name);
            match self.connection.exec(std::ffi::CStr::from_bytes_with_nul(cmd.as_bytes()).unwrap()) {
                Ok(r) => r,
                Err(msg) => return Err(msg),
            }
        };

        assert!(result.num_tuples == 1);

        Ok(result)
    }

    pub fn identify_system<'r, 's: 'r>(&'s self) -> Result<Tuples<'r>, String> {
        let result = unsafe {
            match self.connection.exec(c"IDENTIFY_SYSTEM") {
                Ok(r) => r,
                Err(msg) => return Err(msg),
            }
        };

        assert!(result.num_attrs == 4);
        assert!(result.num_tuples == 1);

        Ok(result)
    }

    pub fn create_publication(&self) -> Result<(), String> {
        let existing_pub = unsafe {
            let cmd = format!("SELECT pubname FROM pg_publication WHERE pubname = '{}_pub';\0", self.slot_name);  
            match self.connection.exec(std::ffi::CStr::from_bytes_with_nul(cmd.as_bytes()).unwrap()) {
                Ok(r) => r,
                Err(msg) => return Err(msg),
            }
        };

        if existing_pub.num_tuples == 1 {
            return Ok(());
        }

        let result = unsafe {
            let cmd = format!("CREATE PUBLICATION {}_pub FOR ALL TABLES;\0", self.slot_name);
            match self.connection.exec(std::ffi::CStr::from_bytes_with_nul(cmd.as_bytes()).unwrap()) {
                Ok(r) => r,
                Err(msg) => return Err(msg),
            }
        };

        Ok(())
    }

    fn start_replication(&self, restart_lsn: &std::ffi::CStr) -> Result<bool, String> {
        let lsn = String::from_utf8(restart_lsn.to_bytes().to_owned()).unwrap();
        let cmd = format!(
            "START_REPLICATION SLOT {0} LOGICAL {1} (proto_version '4', streaming 'false', publication_names '{0}_pub')\0",
            self.slot_name,
            lsn,
        );
        let result = unsafe {
            self.connection.exec_unchecked(std::ffi::CStr::from_bytes_with_nul(cmd.as_bytes()).unwrap())
        };

        match result.status {
            crate::bindings::ExecStatusType::PGRES_COMMAND_OK => Ok(false),
            crate::bindings::ExecStatusType::PGRES_COPY_BOTH => Ok(true),
            _ => Err(unsafe { self.connection.get_err_owned() }),
        }
    }

    fn send_status(&self, request_reply: u8) -> Result<(), String> {
        let get_timestamp = || {
            let start = time::macros::utc_datetime!(2000-01-01 00:00);
            let now = time::UtcDateTime::now();
            (now - start).whole_microseconds() as i64
        };
        let mut cmd = Cmd::new();
        cmd
            .push_byte(b'r')
            .push_i64(0)
            .push_i64(0)
            .push_i64(0)
            .push_i64(get_timestamp())
            .push_byte(request_reply)
            .finish();
        
        let result = unsafe {
            match self.connection.put_copy_msg(cmd.as_bytes()) {
                Ok(r) => r,
                Err(msg) => return Err(msg),
            }
        };

        println!("send_status({request_reply}): {}", result);
        // ../../postgres/src/interfaces/libpq/fe-exec.c:2691
        assert!(result == 1);

        Ok(())
    }

    pub fn start(&self) -> Result<(), String> {
        assert!(WalLevel::Logical == self.get_wal_level().unwrap());

        self.create_repl_slot().unwrap();
        self.connection.check_notifies();
        self.create_publication().unwrap();

        let slots = self.get_repl_slots().unwrap();
        let slot = slots.iter().next().unwrap();
        let restart_lsn = slot.get_attr_value(slot.get_attr_idx(c"restart_lsn"));
        println!("Slot: {:?}", slot);
        println!("Restart LSN: {:?}", restart_lsn);

        loop {
            if self.start_replication(restart_lsn).unwrap() {
                // ../../postgres/src/backend/replication/walreceiver.c:437
                self.send_status(1);
                loop {
                    unsafe {
                        let Ok((len, buf)) = self.connection.get_copy_msg() else {
                            unimplemented!();
                        };
                        if len == 0 {
                            println!(">> No more copy data");
                            break;
                        }
                        let cm = CopyMessage::from_slice(std::slice::from_raw_parts(buf, len as usize));
                        println!(">> {:?}", cm);
                        println!(">> got copy_data: {} - header {:?}", len, (*buf as u8) as char);
                        println!(">> {:?}", std::slice::from_raw_parts(buf, len as usize));
                    }
                }
            } else {
                println!("No new changes to consume");

                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }

        Ok(())
    }
}
