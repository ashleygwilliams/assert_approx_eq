# assert_approx_eq
> a rust macro for asserting 2 numbers are approximately equal 

[![crates.io badge](https://img.shields.io/crates/v/assert_approx_eq.svg)](https://crates.io/crates/assert_approx_eq)
[![travis badge](https://api.travis-ci.org/ashleygwilliams/assert_approx_eq.svg?branch=master)](https://travis-ci.org/ashleygwilliams/assert_approx_eq)
[![appveyor badge](https://ci.appveyor.com/api/projects/status/j6q5vay6ryne4du7?svg=true)](https://ci.appveyor.com/project/ashleygwilliams/assert-approx-eq)

This crate exports a macro for asserting that two numbers are approximately equal (~1.0e-6, by default) to each other.

On panic, this macro will print the values of the expressions with their
debug representations. You can optionally add an optional diff value. If you
don't supply a diff value as an argument, `1.0e-6` is the default used. 

## Usage

To use this crate, add `assert_approx_eq` as a dependency of your Rust project.

Then, you can use the macro as follows:

```rust
use assert_approx_eq::assert_approx_eq;

let a = 3f64;
let b = 4f64;

assert_approx_eq!(a, b); //panics
assert_approx_eq!(a, b, 2f64); //does not panic
assert_approx_eq!(a, b, 1e-3f64); // panics
```

You can read documentation for how to use this crate on [docs.rs](https://docs.rs/assert_approx_eq).
