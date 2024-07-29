use syn::{parse_file, File};

pub fn parse_rust_code(source: &str) -> File {
    parse_file(source).expect("failed to parse rust code")
}
