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
    let mut features = vec![];
    if version >= [3, 7, 14] {
        features.push("sqlite3-close-v2");
    }
    if version >= [3, 7, 15] {
        features.push("sqlite3-errstr");
    }
    for feature in &features {
        println!(r#"cargo:rustc-cfg=feature="{}""#, feature);
    }
    println!("cargo:features={}", join(&features));
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

fn join(chunks: &[&str]) -> String {
    let mut buffer = String::new();
    for (i, chunk) in chunks.iter().enumerate() {
        if i > 0 {
            buffer.push(' ');
        }
        buffer.push_str(chunk);
    }
    buffer
}
