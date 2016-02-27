extern crate pkg_config;

fn main() {
    if pkg_config::find_library("sqlite3").is_err() {
        println!("cargo:rustc-link-lib=dylib=sqlite3");
    }
}
