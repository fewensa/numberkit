numberkit
===

[![Build Status](https://drone.0u0.me/api/badges/fewensa/numberkit/status.svg)](https://drone.0u0.me/fewensa/numberkit)


Number kit for Rust.

fast string to number operation.

# Usage

```toml
[dependencies]
numberkit = "0.1"
```


# Check string is a number

```rust
assert_eq!(true, numberkit::is_number("0"));
assert_eq!(false, numberkit::is_number("-1u32"));
assert_eq!(true, numberkit::is_number("2usize"));
assert_eq!(true, numberkit::is_number("3.5f32"));
assert_eq!(false, numberkit::is_number("0.2.1f32"));
```

support `isize` `usize` `float` string check.

In addition, support digit check.

```rust
assert_eq!(true, numberkit::is_digit("2", false));
assert_eq!(false, numberkit::is_digit("-2", false));
assert_eq!(false, numberkit::is_digit("0.2", false));
assert_eq!(true, numberkit::is_idigit("-2"));
assert_eq!(true, numberkit::is_udigit("2"));
```

# Covert string to number

```rust
assert_eq!(1 as isize, numberkit::as_isize("1").unwrap());
assert_eq!(-1 as isize, numberkit::as_isized("a", -1 as isize));
assert_eq!(1 as usize, numberkit::as_usize("1").unwrap());
assert_eq!(52 as i128, numberkit::as_i128("52").unwrap());
assert_eq!(0.5 as f64, numberkit::as_f64("0.5").unwrap());
assert_eq!(0.5 as f64, numberkit::as_f64d("0.5", 0.5 as f64));
```

`as_*` function will covert string to number, but return value this Result, because errors may occur.

`as_*d` function  checks for errors and needs to provide a default value, using the default value when the send conversion fails.
