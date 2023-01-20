// https://doc.rust-lang.org/reference/inline-assembly.html

mod ch {
    #![allow(dead_code)]

    use std::arch::asm;

    pub fn to_lower(char: u8) -> u8 {
        // 010X_XXXX - pattern of upppercase ASCII character
        // 011X_XXXX - pattern of lowercase ASCII character
        #[allow(unused_mut)]
        let mut res = char;

        unsafe {
            asm! {
                "cmp {val}, 65d",
                "JL 0f",
                "cmp {val}, 90d",
                "JG 0f",
                "OR {val}, 00100000b",
                "0:",
                val = inout(reg_byte) res,
                options(pure, nomem, nostack),
            }
        }

        res
    }

    pub fn to_upper(char: u8) -> u8 {
        #[allow(unused_mut)]
        let mut res = char;

        unsafe {
            asm! {
                "cmp {val}, 97d",
                "JL 0f",
                "cmp {val}, 122d",
                "JG 0f",
                "AND {val}, 11011111b",
                "0:",
                val = inout(reg_byte) res,
                options(pure, nomem, nostack),
            }
        }

        res
    }
}

mod bigint {
    #![allow(dead_code)]

    use std::arch::asm;
    use std::fmt::Display;

    #[derive(Debug, PartialEq)]
    #[repr(transparent)]
    pub struct BigInt(Vec<u8>);

    impl BigInt {
        // Could store u32s like here: https://github.com/Dogzik/bigint/blob/master/big_integer.cpp
        pub fn new(value: &str) -> Result<BigInt, ()> {
            let mut start_idx = 0;
            let mut v: Vec<u8> = Vec::with_capacity(value.len());

            for (i, c) in value.chars().enumerate() {
                if c != '0' && c != '_' {
                    start_idx = i;
                    break;
                }
            }

            for c in value[start_idx..].chars().rev() {
                if c == '_' {
                    continue;
                }

                if !c.is_ascii_digit() && c != '_' {
                    return Err(());
                }

                v.push((c as u8) - 48);
            }

            if v.is_empty() {
                return Err(());
            }

            Ok(BigInt(v))
        }

        #[inline]
        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn add(left: &BigInt, right: &BigInt) -> BigInt {
            use std::cmp::{max, min};

            let left_len = left.len();
            let right_len = right.len();
            let min_len = min(left_len, right_len);
            let max_len = max(left_len, right_len);
            let longer = if left_len > right_len { left } else { right };
            let mut result = Vec::with_capacity(max_len + 1);
            let mut carry = 0_u8;

            for i in 0..min_len {
                let sum = left.0[i] + right.0[i] + carry;
                carry = sum / 10;
                result.push(sum % 10);
            }

            for i in min_len..max_len {
                let sum = longer.0[i] + carry;
                carry = sum / 10;
                result.push(sum % 10);
            }

            if carry > 0 {
                result.push(carry);
            }

            BigInt(result)
        }

        pub fn add_asm(left: &BigInt, right: &BigInt) -> BigInt {
            let left_len = left.0.len();
            let right_len = right.0.len();
            let (longer, shorter, min, max) = if left_len > right_len {
                (left, right, right_len, left_len)
            } else {
                (right, left, left_len, right_len)
            };

            let mut result: Vec<u8> = Vec::with_capacity(max + 1);
            let mut index: usize = 0;

            unsafe {
                asm! {
                    "MOV RAX, 0",
                    "MOV R8, 10d",
                    "MOV R10, 0", // Carry

                    "2:", // Loop 0..min
                    "MOV AL, [r15]",
                    "ADD AL, [r14]",
                    "ADD AL, R10b",
                    "CDQ",
                    "DIV R8",
                    "MOV [R13], DL",
                    "MOV R10b, AL",
                    // Increment pointers.
                    "ADD R15, 1",
                    "ADD R14, 1",
                    "ADD R13, 1",
                    // Increment index.
                    "ADD R9, 1",
                    // End loop?
                    "CMP R9, R11",
                    "JL 2b",
                    // Is min == max
                    "CMP R12, R11",
                    "JE 8f",

                    // Loop min..max
                    "4:",
                    "MOV AL, [r15]",
                    "ADD AL, R10b",
                    "CDQ",
                    "DIV R8",
                    "MOV [R13], DL",
                    "MOV R10b, AL",
                    // Increment pointers.
                    "ADD R15, 1",
                    "ADD R14, 1",
                    "ADD R13, 1",
                    // Increment index.
                    "ADD R9, 1",
                    "CMP R9, R12",
                    "JL 4b",

                    // Add remaining carry
                    "8:",
                    "CMP R10, 0",
                    "JE 9f",
                    "MOV [R13], R10b",
                    "ADD R9, 1",
                    "9:",

                    in("r15") longer.0.as_ptr(),
                    in("r14") shorter.0.as_ptr(),
                    in("r13") result.as_mut_ptr(),
                    in("r12") max,
                    in("r11") min,
                    inout("r9") index,
                    options(nostack),
                }
            }

            unsafe {
                result.set_len(index);
            }

            BigInt(result)
        }
    }

    impl Display for BigInt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use std::fmt::Write;

            let mut left = self.0.len();

            for v in self.0.iter().rev() {
                f.write_char((v + 48) as char)?;
                left -= 1;
                if left % 4 == 0 && left > 0 {
                    f.write_char('_')?;
                }
            }

            f.write_str("n")
        }
    }

    impl PartialEq<Vec<u8>> for BigInt {
        fn eq(&self, other: &Vec<u8>) -> bool {
            &self.0 == other
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_lower_test() {
        use super::ch::to_lower;

        assert_eq!(to_lower(10), 10);

        for c in 65..=90 {
            assert_eq!(to_lower(c), c | 0b0010_0000);
        }
    }

    #[test]
    fn to_upper_test() {
        use super::ch::to_upper;

        assert_eq!(to_upper(50), 50);

        for c in 97..=122 {
            assert_eq!(to_upper(c), c & 0b1101_1111);
        }
    }

    #[test]
    fn new_bigint_trim_prefix_zeros() {
        use super::bigint::BigInt;

        assert_eq!(BigInt::new("0001").unwrap(), vec![1]);
        assert_eq!(BigInt::new("0000_0001").unwrap(), vec![1]);
        assert_eq!(BigInt::new("129").unwrap(), vec![9, 2, 1]);
        assert_eq!(BigInt::new("100_000").unwrap(), vec![0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn new_bigint_invalid_err() {
        use super::bigint::BigInt;

        assert!(BigInt::new("0xF").is_err());
        assert!(BigInt::new("a").is_err());
        assert!(BigInt::new("__").is_err());
        assert!(BigInt::new("_0").is_ok());
    }

    #[test]
    fn bigint_add() {
        use super::bigint::BigInt;

        fn n(s: &str) -> BigInt {
            BigInt::new(s).unwrap()
        }

        assert_eq!(
            "3000_0000n",
            BigInt::add(&n("1000_0000"), &n("2000_0000")).to_string()
        );
        assert_eq!(
            "6000_0000n",
            BigInt::add(&n("3000_0000"), &n("3000_0000")).to_string()
        );
        assert_eq!(
            "6_6666n",
            BigInt::add(&n("3_3333"), &n("3_3333")).to_string()
        );
        assert_eq!("10n", BigInt::add(&n("3"), &n("7")).to_string());
        assert_eq!("18n", BigInt::add(&n("9"), &n("9")).to_string());
        assert_eq!("1000n", BigInt::add(&n("1"), &n("999")).to_string());
        assert_eq!("1000n", BigInt::add(&n("999"), &n("1")).to_string());
    }

    #[test]
    fn bigint_add_asm() {
        use super::bigint::BigInt;

        fn n(s: &str) -> BigInt {
            BigInt::new(s).unwrap()
        }

        macro_rules! add_nines {
            ($count:expr) => {
                assert_eq!(
                    format!("1{}_9998n", "_9999".repeat($count)),
                    BigInt::add(
                        &BigInt::new(&format!("9999{}", "9999".repeat($count))).unwrap(),
                        &BigInt::new(&format!("9999{}", "9999".repeat($count))).unwrap(),
                    )
                    .to_string(),
                );
            };
        }

        add_nines!(1);
        add_nines!(2);
        add_nines!(3);
        add_nines!(4);
        add_nines!(5);
        add_nines!(6);
        add_nines!(7);
        add_nines!(8);
        add_nines!(9);
        add_nines!(10);

        assert_eq!(
            "3000_0000n",
            BigInt::add(&n("1000_0000"), &n("2000_0000")).to_string()
        );
        assert_eq!(
            "6000_0000n",
            BigInt::add(&n("3000_0000"), &n("3000_0000")).to_string()
        );
        assert_eq!(
            "6_6666n",
            BigInt::add(&n("3_3333"), &n("3_3333")).to_string()
        );
        assert_eq!("10n", BigInt::add(&n("3"), &n("7")).to_string());
        assert_eq!("18n", BigInt::add(&n("9"), &n("9")).to_string());
        assert_eq!("1000n", BigInt::add(&n("1"), &n("999")).to_string());
        assert_eq!("1000n", BigInt::add(&n("999"), &n("1")).to_string());
    }
}
