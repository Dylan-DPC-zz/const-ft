# const-ft

[![Build Status](https://travis-ci.com/Dylan-DPC/const-ft.svg?branch=master)](https://travis-ci.com/Dylan-DPC/const-ft)
[![Latest version](https://img.shields.io/crates/v/const-ft.svg)](https://crates.io/crates/const-ft)
[![Documentation](https://docs.rs/const-ft/badge.svg)](https://docs.rs/const-ft)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.22+-yellow.svg)
![License](https://img.shields.io/crates/l/const-ft.svg)

A macro for easy generation and wrapping of a const function under a const_fn feature gate.

### Installation: 
 
```toml
 const-ft = { version =  "0.1" }
 ```

You can enable the feature by having a feature in the project  enable the feature from the crate:

```toml
[features]
const = ["const-ft/const_fn"]
```
### Usage:
```rust
const_ft! {
      pub fn some_function() -> usize {
         1usize
}
```

### Requirement: 

By using the macro, your code changes from 
```rust
#![cfg_attr(feature = "const_fn", feature(const_fn))]

     #[cfg(feature = "const_fn")]
     pub const fn some_function() -> usize {
         1usize
     }
 
     #[cfg(not(feature = "const_fn"))]
     pub fn some_function() -> usize {
        1usize
     }
```

to

### 
```rust
#![cfg_attr(feature = "const_fn", feature(const_fn))]

 #[macro_use]
 extern crate const_ft;

 const_ft! {
      pub fn some_function() -> usize {
         1usize
      }
 }
``` 


