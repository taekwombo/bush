#[derive(Debug, PartialEq)]
pub enum WalLevel {
    Minimal,
    Replica,
    Logical,
}

impl WalLevel {
    pub fn from_bytes(b: &[u8]) -> Self {
        match b {
            b"minimal" => Self::Minimal,
            b"replica" => Self::Replica,
            b"logical" => Self::Logical,
            _ => unimplemented!(),
        }
    }
}

