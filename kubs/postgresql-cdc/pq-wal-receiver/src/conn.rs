use super::bindings;

#[derive(Debug)]
pub enum WalLevel {
    Minimal,
    Replica,
    Logical,
}

impl WalLevel {
    fn from_str_bytes(b: &[u8]) -> Self {
        match b {
            b"minimal" => Self::Minimal,
            b"replica" => Self::Replica,
            b"logical" => Self::Logical,
            _ => unimplemented!(),
        }
    }
}

unsafe fn print_err(prefix: &str, err: *const i8) {
    eprintln!("{prefix}:");
    unsafe {
        bindings::fprintf(bindings::stderr, c"%s".as_ptr(), err);
    }
}

#[derive(Debug)]
pub struct Connection {
    // TODO: remove unused
    #[allow(unused)]
    info: *const std::ffi::c_char,
    conn: *mut bindings::PGconn,
}

impl Connection {
    pub unsafe fn new(/* TODO: make str/string */ connection_string: *const std::ffi::c_char) -> Self {
        use bindings::{PQconnectdb, PQstatus, ConnStatusType};

        unsafe {
            let conn = PQconnectdb(connection_string); 
            let status = PQstatus(conn);

            if status != ConnStatusType::CONNECTION_OK {
                print_err("Failed to establish connection to {}", /* TODO: print connection_string */ bindings::PQerrorMessage(conn));
                std::process::exit(69);
            }

            Self {
                info: connection_string,
                conn,
            }
        }
    }
    
    unsafe fn exec<'a>(&'a mut self, query: &std::ffi::CStr) -> ExecRes<'a> {
        use bindings::{PQexec, PQerrorMessage, PQresultStatus, ExecStatusType};

        unsafe {
            let result = PQexec(self.conn, query.as_ptr());

            if PQresultStatus(result) != ExecStatusType::PGRES_TUPLES_OK {
                eprintln!("Status = {:?}", PQresultStatus(result).0);
                return ExecRes::Err(PQerrorMessage(self.conn));
            }

            ExecRes::Ok(QueryResult::new(result))
        }
    }

    pub unsafe fn get_wal_level(&mut self) -> Result<WalLevel, ()> {
        unsafe {
            let result = match self.exec(c"SHOW wal_level;") {
                ExecRes::Ok(r) => r,
                ExecRes::Err(msg) => return Err(print_err("Failed to obtain wal_level config value", msg)),
            };

            debug_assert!(result.columns == 1);
            debug_assert!(result.tuples == 1);

            Ok(WalLevel::from_str_bytes(result.get_value(0, 0).to_bytes()))
        }
    }

    pub unsafe fn identify_system(&mut self) {
        unsafe {
            let result = match self.exec(c"IDENTIFY_SYSTEM") {
                ExecRes::Ok(r) => r,
                ExecRes::Err(msg) => return print_err("Failed IDENTIFY_SYSTEM command", msg),
            };

            debug_assert!(result.columns == 4);
            debug_assert!(result.tuples == 1);

            for i in 0..4 {
                println!("{:?} = {:?}", result.get_col(i), result.get_value(0, i));
            }
        }
    }

    pub unsafe fn create_temp_slot(&mut self, name: &str) -> bool {
        unsafe {
            let query = format!("CREATE_REPLICATION_SLOT {name} TEMPORARY LOGICAL pgoutput;\0");
            let queryc = std::ffi::CStr::from_bytes_with_nul(query.as_bytes()).unwrap();
            let result = match self.exec(queryc) {
                ExecRes::Ok(r) => r,
                ExecRes::Err(msg) => return {
                    print_err("Failed CREATE_REPLICATION_SLOT command", msg);
                    false
                },
            };

            for i in 0..result.columns {
                println!("{:?} = {:?}", result.get_col(i), result.get_value(0, i));
            }

            true
        }
    }

    unsafe fn get_restart_lsn(&mut self, name: &str) -> Result<String, ()> {
        unsafe {
            let query = format!("SELECT restart_lsn FROM pg_replication_slots WHERE slot_name = '{name}';\0");
            let queryc = std::ffi::CStr::from_bytes_with_nul(query.as_bytes()).unwrap();
            let result = match self.exec(queryc) {
                ExecRes::Ok(r) => r,
                ExecRes::Err(msg) => return Err(print_err("Failed to obtain wal_level config value", msg)),
            };

            assert!(result.columns == 1);
            assert!(result.tuples == 1);

            result.get_value(0, 0).to_str().map(String::from).map_err(|_| ())
        }
    }

    unsafe fn create_publication(&mut self, name: &str) -> String {
        let pubname = format!("pub_{name}");

        unsafe {
            let query = format!("CREATE PUBLICATION {pubname} FOR ALL TABLES;\0");
            let queryc = std::ffi::CStr::from_bytes_with_nul(query.as_bytes()).unwrap();
            let result = match self.exec(queryc) {
                ExecRes::Ok(r) => r,
                ExecRes::Err(msg) => {
                    print_err("Failed CREATE PUBLICATION command", msg);
                    return pubname;
                },
            };

            for i in 0..result.columns {
                println!("{:?} = {:?}", result.get_col(i), result.get_value(0, i));
            }
        }

        pubname
    }

    pub unsafe fn start(&mut self, name: &str) {
        // postgres/src/backend/replication/walreceiver.c:337
        unsafe {
            assert!(self.create_temp_slot(name));
            let lsn = self.get_restart_lsn(name).unwrap();
            let pubname = self.create_publication(name);

            println!("lsn = {lsn}");

            // ../../postgres/src/backend/replication/walreceiver.c:437
            // OK - No more data to consume.
            // COPY_BOTH - Some data waiting for replication.
            let query = format!("START_REPLICATION SLOT {name} LOGICAL {lsn} (proto_version '4', streaming 'false', publication_names '{pubname}') ;\0");
            let queryc = std::ffi::CStr::from_bytes_with_nul(query.as_bytes()).unwrap();
            let result = match self.exec(queryc) {
                ExecRes::Ok(r) => r,
                ExecRes::Err(msg) => return print_err("Failed START_REPLICATION command", msg),
            };

            println!("{:?}", result.get_column_names());
        }
    }
}

impl Drop for Connection {
    // May get away with just simply calling PQfinish, under the assumption that libpq handles all
    // edge cases.
    fn drop (&mut self) {
        unsafe {
            bindings::PQfinish(self.conn);
        }
    }
}

pub enum ExecRes<'a> {
    Ok(QueryResult<'a>),
    Err(*const i8),
}

pub struct QueryResult<'a> {
    _lifetime: std::marker::PhantomData<&'a ()>,
    result: *mut bindings::PGresult,
    columns: std::ffi::c_int,
    tuples: std::ffi::c_int,
}

impl QueryResult<'_> {
    unsafe fn new(result: *mut bindings::PGresult) -> Self {
        unsafe {
            Self {
                _lifetime: std::marker::PhantomData,
                tuples: bindings::PQntuples(result),
                columns: bindings::PQnfields(result),
                result,
            }
        }
    }

    pub fn get_column_names(&self) -> Vec<String> {
        debug_assert!(self.columns > 0);

        let cols: usize = self.columns.try_into().expect("nfields not >= 0");
        let mut result = Vec::with_capacity(cols);
        for i in 0..cols {
            unsafe {
                let cname = bindings::PQfname(self.result, i as i32);
                // TODO: Are PG column names utf8 encoded?
                result.push(String::from_utf8(std::ffi::CStr::from_ptr(cname).to_bytes().to_owned()).unwrap());
            }
        }
        result
    }

    fn get_col(&self, col_idx: i32) -> &std::ffi::CStr {
        unsafe {
            std::ffi::CStr::from_ptr(bindings::PQfname(self.result, col_idx))
        }
    }

    fn get_value(&self, row_idx: i32, col_idx: i32) -> &std::ffi::CStr {
        unsafe {
            std::ffi::CStr::from_ptr(bindings::PQgetvalue(self.result, row_idx, col_idx))
        }
    }
}

impl Drop for QueryResult<'_> {
    fn drop(&mut self) {
        unsafe {
            bindings::PQclear(self.result);
        }
    }
}
