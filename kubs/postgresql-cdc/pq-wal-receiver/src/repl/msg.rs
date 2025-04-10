fn i64_from_slice(slice: &[u8], at: usize) -> i64 {
    assert!(at + 8 <= slice.len(), "Be careful: {at} + 8 <= {}?", slice.len());
    
    i64::from_be_bytes(unsafe {
        *std::mem::transmute::<&u8, &[u8; 8]>(&slice[at])
    })
}

#[derive(Debug)]
pub enum CopyMessage {
    Keepalive {
        end_of_wal: i64,
        server_time: i64,
        should_reply: u8,
    }
}

impl CopyMessage {
    pub fn from_slice(s: &[std::ffi::c_char]) -> Self {
        let slice: &[u8] = unsafe { std::mem::transmute(s) };

        match slice[0] {
            b'k' => {
                assert!(slice.len() == 18);
                let end_of_wal = i64_from_slice(slice, 1);
                let server_time = i64_from_slice(slice, 9);
                let should_reply = slice[17];

                Self::Keepalive {
                    end_of_wal,
                    server_time,
                    should_reply,
                }
            },
            _ => unimplemented!(),
        }
    }
}
