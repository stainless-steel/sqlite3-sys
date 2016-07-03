//! Bindings to [SQlite][1].
//!
//! [1]: https://www.sqlite.org

#![allow(improper_ctypes, non_camel_case_types)]

extern crate libc;

#[cfg(feature = "linkage")]
extern crate sqlite3_src;

mod constants;
mod functions;
mod types;

pub use constants::*;
pub use functions::*;
pub use types::*;
