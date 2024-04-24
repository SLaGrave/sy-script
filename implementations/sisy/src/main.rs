use std::collections::HashMap;
use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "syscript.pest"]
pub struct SyParser;

fn main() {
    let unparsed_file = fs::read_to_string("test.sy").expect("Cannot read file");
}
