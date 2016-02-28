extern crate pkg_config;

#[cfg(not(feature = "sqlcipher"))]
const LIBNAME: &'static str = "sqlite3";

#[cfg(feature = "sqlcipher")]
const LIBNAME: &'static str = "sqlcipher";


fn main() {
    if pkg_config::find_library(LIBNAME).is_err() {
        println!("cargo:rustc-link-lib=dylib={}", LIBNAME);
    }
}
