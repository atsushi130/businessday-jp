# businessday-jp
[![MIT / Apache2.0 dual licensed](https://img.shields.io/badge/dual%20license-MIT%20/%20Apache%202.0-blue.svg)](./license-mit.md)
[![crates.io](https://img.shields.io/crates/v/businessday_jp.svg)](https://crates.io/crates/businessday_jp)
[![Document](https://img.shields.io/badge/businessday_jp-Document-3B5998.svg)](https://docs.rs/businessday_jp/0.1.0/businessday_jp/)

## dependencies
```toml
[dependencies]
businessday_jp = "0.1.0"
```

## Usage
```rust
extern crate businessday_jp;

use businessday_jp::BusinessdayService;

fn main() {
    if HolidayService.is_businessday(2017, 1, 5, 2) {
        println!("businessday");
    }
}
```

**BusinessdayService API**
```rust
fn is_businessday(&self, year: i32, month: u32, day: u32, at_businseeday: u32) -> bool;
fn get_businessday(&self, year: i32, month: u32, at_businessday: u32) -> DateTime<Local>;
```

## License
**This project is dual-licensed under MIT and Apache 2.0.**

