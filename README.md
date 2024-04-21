# sqlite3-sys [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides bindings to [SQLite].

The following Cargo features are supported:

* `linkage` creates a dependency on `sqlite3-src`, which links to a suitable
  SQLite library;
* `bundled` compiles SQLite from the source code, ignoring any libraries that
  might already be installed; and
* `encryption` enables bindings to the [SQLite Encryption Extension], which is
  closed source and hence requires purchasing a license and installing SQLite
  manually.

## Development

```shell
cargo install bindgen-cli
git clone https://github.com/stainless-steel/sqlite3-src.git --recursive
bindgen --use-core sqlite3-src/source/sqlite3.h \
  | sed -E "s/^pub const ([0-9A-Z_]+): u32/pub const \1: ::core::ffi::c_int/" \
  > src/bindings.rs
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[SQLite]: https://www.sqlite.org
[SQLite Encryption Extension]: https://www.sqlite.org/see/doc/release/www/index.wiki

[build-img]: https://github.com/stainless-steel/sqlite3-sys/workflows/build/badge.svg
[build-url]: https://github.com/stainless-steel/sqlite3-sys/actions/workflows/build.yml
[documentation-img]: https://docs.rs/sqlite3-sys/badge.svg
[documentation-url]: https://docs.rs/sqlite3-sys
[package-img]: https://img.shields.io/crates/v/sqlite3-sys.svg
[package-url]: https://crates.io/crates/sqlite3-sys
