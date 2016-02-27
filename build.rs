extern crate pkg_config;

fn main() {
    let name = "sqlite3";
    if pkg_config::find_library(name).is_err() {
        println!("cargo:rustc-link-lib=dylib={}", name);
    }
}
