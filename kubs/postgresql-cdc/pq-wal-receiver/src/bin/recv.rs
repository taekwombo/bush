use pq_wal_receiver;

fn main() {
    let conn_info = String::from("postgresql://user:pass@localhost/db?replication=database");
    unsafe {
        let conn = pq_wal_receiver::conn::Connection::new(conn_info);
        let mut repl = pq_wal_receiver::Replication::new(conn, "the_replihno".to_owned());
        let id = repl.identify_system().unwrap();
        let id2 = repl.identify_system().unwrap();
        let id3 = repl.identify_system().unwrap();
        println!("{:?}", repl);
        println!("{:?}", repl.identify_system().unwrap());
        repl.start();
    }
}
