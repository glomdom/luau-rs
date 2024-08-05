//! Main test runner for luau-rs.
//! 
//! Contains the function responsible for tests.

#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use roblox_rs::transformer;
use syn::{parse_file, File};
use pretty_assertions::assert_eq;

#[datatest::files("tests/data", {
    input in r"^(.*).in",
    output = r"${1}.out",
})]
fn main_tests(input: &str, output: &str) {
    let code = parse_rust_code(input);
    let ast = transformer::transform_file_to_luau(&code);

    assert_eq!(output, format!("{:#?}", ast));
}


pub fn parse_rust_code(source: &str) -> File {
    parse_file(source).expect("failed to parse rust code")
}
