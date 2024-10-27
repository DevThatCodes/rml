use std::fs;

use rml::*;

fn main() {
    println!("try parse file:\n{:#?}", parse_from_string(fs::read_to_string("test.rml").unwrap()))
}
