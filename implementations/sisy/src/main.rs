// use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "syscript.pest"]
pub struct SyParser;

#[derive(Debug)]
pub enum SyLineType {
    COMMENT,
    COMMAND,
}

#[derive(Debug)]
pub struct SyLine {
    _line_type: SyLineType,  // TODO: Remove underscore
}

fn main() {
    let unparsed_file = fs::read_to_string("./test.sy").expect("Cannot read file");
    
    let file = SyParser::parse(Rule::program, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    let mut lines: Vec<SyLine> = Vec::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::comment => {
                let new_line = SyLine { _line_type: SyLineType::COMMENT };
                lines.push(new_line);
            }
            Rule::command => {
                let new_line = SyLine { _line_type: SyLineType::COMMAND };
                lines.push(new_line);
            }
            _ => {}
        }
    }

    println!("{:#?}", lines);
}
