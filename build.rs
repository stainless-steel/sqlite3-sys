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
    if version >= [3, 7, 14] {
        println!(r#"cargo:rustc-cfg=feature="sqlite3-close-v2""#);
    }
    if version >= [3, 7, 15] {
        println!(r#"cargo:rustc-cfg=feature="sqlite3-errstr""#);
    }
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
