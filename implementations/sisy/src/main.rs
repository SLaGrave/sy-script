use std::fs;
use std::vec::Vec;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "syscript.pest"]
pub struct SyParser;

#[derive(Debug, Clone)]
pub struct SyOperand {
    _noop: bool,
    _int: Option<i32>,
    ident: Option<String>,
}

#[derive(Debug)]
pub struct SyCommandSy {
    idx: u32,
    arg0: SyOperand,
    arg1: SyOperand,
    arg2: SyOperand,
    arg3: SyOperand,
}

#[derive(Debug)]
pub struct SyCommandLeaf {
    idx: u32,
    ident: String,
}

#[derive(Debug)]
pub struct SyComment {
    span: String,
}

fn main() {
    let unparsed_file = fs::read_to_string("./test.sy").expect("Cannot read file");
    
    let file = SyParser::parse(Rule::program, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    let mut comments: Vec<SyComment> = Vec::new();
    let mut sys: Vec<SyCommandSy> = Vec::new();
    let mut leafs: Vec<SyCommandLeaf> = Vec::new();

    let mut idx = 0;
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::comment => {
                comments.push( SyComment{ span: line.as_str().to_string() } );
            }
            Rule::command_sy => {
                let inner = line.into_inner();
                let mut args: Vec<SyOperand> = Vec::new();
                for arg in inner {
                    match arg.as_rule() {
                        Rule::operand_noop => {
                            args.push(SyOperand { _noop: true, _int: None, ident: None, });
                        }
                        Rule::operand_integer => {
                            let s = arg.as_str().to_string();
                            let i = s.parse::<i32>().unwrap();
                            args.push(SyOperand { _noop: false, _int: Some(i), ident: None, });
                        }
                        Rule::operand_identifier => {
                            let s = arg.as_str().to_string();
                            args.push(SyOperand { _noop: false, _int: None, ident: Some(s), });
                        }
                        _ => {}
                    }
                }
                sys.push( SyCommandSy{
                    idx,
                    arg0: args.get(0).unwrap().clone(),
                    arg1: args.get(1).unwrap().clone(),
                    arg2: args.get(2).unwrap().clone(),
                    arg3: args.get(3).unwrap().clone(),
                } );
                idx += 1;
            }
            Rule::command_leaf => {
                let ident = line.into_inner().next().unwrap().as_str().to_string();
                leafs.push( SyCommandLeaf{ idx, ident } );
            }
            Rule::EOI => {
                println!("EOI is idx = {}.", idx);
            }
            _ => {}
        }
    }

    println!("{:#?}", comments);
    println!("{:#?}", sys);
    println!("{:#?}", leafs);
}
