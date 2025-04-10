pub struct Cmd {
    buf: Vec<u8>,
}

impl Cmd {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn push_byte(&mut self, byte: u8) -> &mut Self {
        self.buf.push(byte);
        self
    }

    pub fn push_i64(&mut self, v: i64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_be_bytes());
        self
    }

    pub fn finish(&mut self) -> &mut Self {
        self.buf.push(0);
        self
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.buf.as_slice()
    }
}
