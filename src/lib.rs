//! Bindings to [SQlite][1].
//!
//! [1]: https://www.sqlite.org

#![allow(non_camel_case_types, non_snake_case)]
#![no_std]

#[cfg(feature = "linkage")]
extern crate sqlite3_src;

mod bindings;

pub use bindings::*;
