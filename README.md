[![Crates.io](https://img.shields.io/crates/v/uao-rs.svg)](https://crates.io/crates/uao-rs)
[![License](https://img.shields.io/crates/l/uao-rs)](LICENSE-MIT)
[![Build Status](https://travis-ci.org/zenixls2/uao-rs.svg?branch=master)](https://travis-ci.org/zenixls2/uao-rs)

# uao-rs version - 0.1.0

## UAO-rs

** big5-uao decoder/encoder for rust **

Original UAO table is from:
- https://moztw.org/docs/big5/table/uao250-b2u.txt
- https://moztw.org/docs/big5/table/uao250-b2u.txt

### Examples:
```rust
use uao_rs::{decode, encode};
const UNICODE: &str = "一小段中文測試♥一小段中文测试♥中国の短いテスト♥";
const BIG5: &[u8] = b"\xa4\x40\xa4\x70\xac\x71\xa4\xa4\xa4\xe5\xb4\xfa\xb8\xd5\x9d\xde\xa4\x40\xa4\x70\xac\x71\xa4\xa4\xa4\xe5\x84\xf2\x86\x49\x9d\xde\xa4\xa4\x83\xf6\xc7\x55\xb5\x75\xc6\xea\xc7\xc2\xc7\xb5\xc7\xc4\x9d\xde";
assert_eq!(encode(UNICODE), BIG5);
assert_eq!(decode(BIG5), UNICODE);
```

### License

Licensed under

* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licences/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, shall be licensed as above,
without any additional terms or conditions.
