use pq_wal_receiver;
use std::ffi::CStr;

fn main() {
    unsafe {
        // println!("{}", pq_wal_receiver::version());
        let url = CStr::from_bytes_with_nul(b"postgresql://user:pass@localhost/db?replication=database\0").unwrap();
        let mut conn = pq_wal_receiver::conn::Connection::new(url.as_ptr());
        println!("{:?}", conn);
        println!("{:?}", conn.get_wal_level());
        conn.identify_system();
        conn.start("playin3");
    }
}
