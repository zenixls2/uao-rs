//! # UAO-rs
//!
//! ** big5-uao decoder/encoder for rust **
//!
//! Original UAO table is from:
//! - https://moztw.org/docs/big5/table/uao250-b2u.txt
//! - https://moztw.org/docs/big5/table/uao250-b2u.txt
//!
//! ## Examples:
//! ```rust
//! use uao_rs::{decode, encode};
//! const UNICODE: &str = "一小段中文測試♥一小段中文测试♥中国の短いテスト♥";
//! const BIG5: &[u8] = b"\xa4\x40\xa4\x70\xac\x71\xa4\xa4\xa4\xe5\xb4\xfa\xb8\xd5\x9d\xde\xa4\x40\xa4\x70\xac\x71\xa4\xa4\xa4\xe5\x84\xf2\x86\x49\x9d\xde\xa4\xa4\x83\xf6\xc7\x55\xb5\x75\xc6\xea\xc7\xc2\xc7\xb5\xc7\xc4\x9d\xde";
//! assert_eq!(encode(UNICODE), BIG5);
//! assert_eq!(decode(BIG5), UNICODE);
//! ```

use lazy_static::lazy_static;
use std::convert::TryInto;

use std::collections::HashMap;
lazy_static! {
    static ref U2B: HashMap<char, [u8; 2]> = {
        let mut result = HashMap::new();
        let u2b = include_str!("u2b.txt");
        let mut iter = u2b.split_ascii_whitespace();
        while let Some(v) = iter.next() {
            let k = iter.next().unwrap();
            let value = u16::from_str_radix(&v[2..], 16).unwrap();
            // skip replacement character
            if value != 65533 {
                let key = u32::from_str_radix(&k[2..], 16).unwrap();
                let unicode: char = std::char::from_u32(key).unwrap();
                let big5: [u8; 2] = [(value >> 8) as u8, (value &0b11111111) as u8];
                result.insert(unicode, big5);
            }
        }
        result
    };
    static ref B2U: HashMap<[u8; 2], char> = {
        let mut result = HashMap::new();
        let b2u = include_str!("b2u.txt");
        let mut iter = b2u.split_ascii_whitespace();
        while let Some(k) = iter.next() {
            let v = iter.next().unwrap();
            let key = u16::from_str_radix(&k[2..], 16).unwrap();
            let value = u32::from_str_radix(&v[2..], 16).unwrap();
            let big5: [u8; 2] = [(key >> 8) as u8, (key &0b11111111) as u8];
            let unicode: char = std::char::from_u32(value).unwrap();
            result.insert(big5, unicode);
        }
        result
    };
}

#[inline]
pub fn encode(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for i in s.chars() {
        if let Some(o) = U2B.get(&i) {
            result.push(o[0]);
            result.push(o[1]);
        } else if let Ok(b) = TryInto::<u32>::try_into(i) {
            if b <= 0x80_u32 {
                result.push(b as u8);
            }
        } else {
            panic!("Cannot encode character {} from {}", i, s);
        }
    }
    result
}

#[inline]
pub fn decode<'a, T>(b: &'a T) -> String
where
    &'a T: IntoIterator<Item = &'a u8>,
    T: ?Sized,
{
    let mut result = String::new();
    let mut iter = b.into_iter();
    while let Some(high) = iter.next() {
        let low = iter.next().unwrap();
        let big5: [u8; 2] = [*high, *low];
        if let Some(c) = B2U.get(&big5) {
            result.push(*c);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    const UNICODE: &str = "一小段中文測試♥一小段中文测试♥中国の短いテスト♥";
    const BIG5: &[u8] = b"\xa4\x40\xa4\x70\xac\x71\xa4\xa4\xa4\xe5\xb4\xfa\xb8\xd5\x9d\xde\xa4\x40\xa4\x70\xac\x71\xa4\xa4\xa4\xe5\x84\xf2\x86\x49\x9d\xde\xa4\xa4\x83\xf6\xc7\x55\xb5\x75\xc6\xea\xc7\xc2\xc7\xb5\xc7\xc4\x9d\xde";
    #[test]
    fn test_encode() {
        assert_eq!(encode(UNICODE), BIG5);
    }
    #[test]
    fn test_decode() {
        assert_eq!(decode(BIG5), UNICODE.to_string());
        assert_eq!(decode(&BIG5.to_vec()), UNICODE.to_string());
    }
}
