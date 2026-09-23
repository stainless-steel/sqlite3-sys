//! Bindings to [SQlite].
//!
//! The following Cargo features are supported:
//!
//! * `linkage` to create a dependency on `sqlite3-src`, which links to a suitable
//!   SQLite library;
//! * `bundled` to activate the linkage and compile SQLite from the source code that
//!   comes with the package, ignoring any SQLite libraries that might be installed
//!   in the system;
//! * `system` to activate the linkage and use the SQLite library that is already
//!   installed in the system, ignoring the possibility of compiling it from the
//!   source code; and
//! * `encryption` to enable bindings to the [SQLite Encryption Extension], which is
//!   closed source and hence requires purchasing a license and installing SQLite
//!   manually.
//!
//! [SQLite]: https://www.sqlite.org
//! [SQLite Encryption Extension]: https://www.sqlite.org/see/doc/release/www/index.wiki

#![allow(non_camel_case_types, non_snake_case)]
#![no_std]

#[cfg(feature = "linkage")]
extern crate sqlite3_src;

mod base;
#[cfg(feature = "encryption")]
mod encryption;

pub use base::*;
#[cfg(feature = "encryption")]
pub use encryption::*;
