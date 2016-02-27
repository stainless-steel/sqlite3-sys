extern crate pkg_config;

fn main() {
    match pkg_config::find_library("sqlite3") {
        Ok(library) => println!("cargo:version={}", library.version),
        _ => println!("cargo:rustc-link-lib=dylib=sqlite3"),
    }
}
