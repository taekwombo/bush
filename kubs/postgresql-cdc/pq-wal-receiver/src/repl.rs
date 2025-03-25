/// https://www.postgresql.org/docs/current/protocol-replication.html

#[allow(unused)]
pub struct Replication {
    connection: crate::conn::Connection,
}
