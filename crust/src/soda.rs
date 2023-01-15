// This is part of *-sys package.
mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// This is part of package that uses *-sys package.

use std::mem::MaybeUninit;

#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Sodium;

impl Sodium {

    pub fn new() -> Result<Sodium, ()> {
        if unsafe { ffi::sodium_init() } < 0 {
            return Err(());
        }

        Ok(Self)
    }

    pub fn crypto_generichash<'a>(self, input: &[u8], key: Option<&[u8]>, out: &'a mut [MaybeUninit<u8>]) -> Result<&'a mut [u8], ()> {
        use ffi::{
            crypto_generichash_BYTES_MIN as BYTES_MIN,
            crypto_generichash_BYTES_MAX as BYTES_MAX,
            crypto_generichash_KEYBYTES_MIN as KEY_MIN,
            crypto_generichash_KEYBYTES_MAX as KEY_MAX,
        };

        let out_len = u32::try_from(out.len()).map_err(|_| ())?;
        if out_len < BYTES_MIN || out_len > BYTES_MAX {
            return Err(());
        }

        let (key, keylen) = if let Some(key) = key {
            let key_len = u32::try_from(key.len()).map_err(|_| ())?;
            if key_len < KEY_MIN || key_len > KEY_MAX {
                return Err(());
            }

            (key.as_ptr(), key.len())
        } else {
            (std::ptr::null(), 0)
        };

        let res = unsafe {
            ffi::crypto_generichash(
                MaybeUninit::slice_as_mut_ptr(out),
                out.len(),
                input.as_ptr(),
                input.len().try_into().map_err(|_| ())?,
                key,
                keylen,
            )
        };

        if res < 0 {
            Err(())
        } else {
            Ok(unsafe {MaybeUninit::slice_assume_init_mut(out) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        assert!(Sodium::new().is_ok());
        assert!(Sodium::new().is_ok());
    }

    #[test]
    fn hash() {
        use super::ffi::crypto_generichash_BYTES as LEN;

        fn hash<'a>(bytes: &[u8], out: &'a mut [MaybeUninit<u8>; LEN as usize]) -> &'a [u8] {
            Sodium::new()
                .unwrap()
                .crypto_generichash(bytes, None, out)
                .unwrap()
        }

        let mut v1 = [MaybeUninit::uninit(); LEN as usize];
        let mut v2 = [MaybeUninit::uninit(); LEN as usize];

        let v1 = hash(b"Arbitrary string", &mut v1);
        let v2 = hash(b"Arbitrary string", &mut v2);

        assert_eq!(v1, v2);
    }
}

