//! Bindings to [SQlite].
//!
//! The following Cargo features are supported:
//!
//! * `linkage` creates a dependency on `sqlite3-src`, which links to a suitable
//!   SQLite library;
//! * `bundled` compiles SQLite from the source code, ignoring any libraries that
//!   might already be installed; and
//! * `encryption` enables bindings to the [SQLite Encryption Extension], which is
//!   closed source and hence requires purchasing a license and installing SQLite
//!   manually.
//!
//! [SQLite]: https://www.sqlite.org
//! [SQLite Encryption Extension]: https://www.sqlite.org/see/doc/release/www/index.wiki

#![allow(non_camel_case_types, non_snake_case)]
#![no_std]

#[cfg(feature = "linkage")]
extern crate sqlite3_src;

mod bindings;
#[cfg(feature = "encryption")]
mod encryption;

pub use bindings::*;
#[cfg(feature = "encryption")]
pub use encryption::*;
