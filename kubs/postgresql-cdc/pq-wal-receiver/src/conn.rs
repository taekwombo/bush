use super::bindings;

pub mod tuples;

use tuples::Tuples;

unsafe fn print_err(prefix: &str, err: *const i8) {
    eprintln!("{prefix}:");
    unsafe {
        bindings::fprintf(bindings::stderr, c"%s".as_ptr(), err);
    }
}

pub struct Connection {
    info: std::ffi::CString,
    conn: *mut bindings::PGconn,
}

impl Connection {
    pub unsafe fn new(connection_string: String) -> Self {
        use bindings::{PQconnectdb, PQstatus, ConnStatusType};

        let conn_info = std::ffi::CString::new(connection_string).unwrap();

        unsafe {
            let conn = PQconnectdb(conn_info.as_c_str().as_ptr()); 
            let status = PQstatus(conn);

            if status != ConnStatusType::CONNECTION_OK {
                let emsg = format!("Failed to establish connection to {:?}", conn_info);
                print_err(&emsg, bindings::PQerrorMessage(conn));
                std::process::exit(69);
            }

            Self {
                info: conn_info,
                conn,
            }
        }
    }

    pub unsafe fn exec_unchecked<'a, 't>(&'a self, query: &std::ffi::CStr) -> Tuples<'t> {
        unsafe {
            let result = bindings::PQexec(self.conn, query.as_ptr());
            let status = bindings::PQresultStatus(result);

            Tuples::new(status, result)
        }
    }

    pub unsafe fn get_err_owned(&self) -> String {
        format!("Error: {:?}", unsafe { std::ffi::CStr::from_ptr(bindings::PQerrorMessage(self.conn)) })
    }
    
    pub unsafe fn exec<'a, 't>(&'a self, query: &std::ffi::CStr) -> Result<Tuples<'t>, String> {
        use bindings::ExecStatusType;

        let result = unsafe {
            self.exec_unchecked(query)
        };

        if result.status.0 == ExecStatusType::PGRES_TUPLES_OK.0 || result.status.0 == ExecStatusType::PGRES_COMMAND_OK.0 {
            Ok(result)
        } else {
            unsafe {
                println!("Status: {:?} == {:?}", result.status, bindings::ExecStatusType::PGRES_TUPLES_OK);
                Err(self.get_err_owned())
            }
        }
    }

    pub unsafe fn put_copy_msg(&self, data: &[u8]) -> Result<std::ffi::c_int, String> {
        unsafe {
            let result = bindings::PQputCopyData(
                self.conn,
                data.as_ptr() as *const std::ffi::c_char,
                data.len().try_into().unwrap(),
            );

            if result <= 0 {
                bindings::PQflush(self.conn);
                return Err(self.get_err_owned());
            }

            Ok(result)
        }
    }

    pub unsafe fn get_copy_msg(&self) -> Result<(std::ffi::c_int, *mut std::ffi::c_char), String> {
        let mut buf: *mut std::ffi::c_char = std::ptr::null_mut();

        unsafe {
            let rawlen = bindings::PQgetCopyData(
                self.conn,
                &mut buf,
                1,
            );

            if rawlen < -1 {
                return Err(self.get_err_owned());
            }

            if rawlen == -1 {
                unimplemented!();
            }

            return Ok((rawlen, buf));
        }
    }

    pub fn check_notifies(&self) {
        if 0 == unsafe { bindings::PQconsumeInput(self.conn) } {
            eprintln!("check_notifies: Some kind of error");
            return;
        }

        let notifies = unsafe { bindings::PQnotifies(self.conn) };

        if notifies == std::ptr::null_mut() {
            return;
        }

        println!("{:?}", notifies);
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

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            f.debug_struct("Connection")
                .field("host", &std::ffi::CStr::from_ptr(bindings::PQhost(self.conn)))
                .field("port", &std::ffi::CStr::from_ptr(bindings::PQport(self.conn)))
                .field("user", &std::ffi::CStr::from_ptr(bindings::PQuser(self.conn)))
                .field("db", &std::ffi::CStr::from_ptr(bindings::PQdb(self.conn)))
                .field("status", &bindings::PQstatus(self.conn).0)
                .finish()
        }
    }
}

