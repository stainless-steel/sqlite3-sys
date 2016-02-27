extern crate pkg_config;

fn main() {
    let library = match pkg_config::find_library("sqlite3") {
        Ok(library) => library,
        _ => {
            println!("cargo:rustc-link-lib=dylib=sqlite3");
            return;
        },
    };
    let version = match parse(&library.version) {
        Some(version) => version,
        _ => return,
    };
    println!("cargo:version={}.{}.{}", version[0], version[1], version[2]);
}

fn parse(line: &str) -> Option<[u32; 3]> {
    let mut version = [0, 0, 0];
    for (i, part) in line.split('.').take(3).enumerate() {
        version[i] = match part.parse::<u32>() {
            Ok(part) => part,
            _ => return None,
        }
    }
    Some(version)
}
