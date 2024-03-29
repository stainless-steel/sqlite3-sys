//! Bindings to [SQlite][1].
//!
//! [1]: https://www.sqlite.org

#![allow(improper_ctypes, non_camel_case_types)]
#![no_std]

#[cfg(feature = "linkage")]
extern crate sqlite3_src;

#[rustfmt::skip]
mod constants;
mod functions;
mod types;

pub use constants::*;
pub use functions::*;
pub use types::*;
