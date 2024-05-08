use std::collections::HashMap;
use std::fs;
use std::vec::Vec;
use std::io;
use std::io::Write;

use pest::Parser;
use pest_derive::Parser;

fn read_stdin() -> i32 {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line.");
    input_line.trim().parse::<i32>().expect("Input not an integer")
}

#[derive(Parser)]
#[grammar = "syscript.pest"]
pub struct SyParser;

#[derive(Debug, Clone)]
pub struct SyOperand {
    noop: bool,
    int: Option<i32>,
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
        .next()
        .unwrap();

    //===================================================
    // Needed Vars
    //===================================================
    let mut sy_comments: Vec<SyComment> = Vec::new(); // Not really used
    let mut sy_sys: Vec<SyCommandSy> = Vec::new();
    let mut sy_leafs: Vec<SyCommandLeaf> = Vec::new();
    let mut sy_eoi: u32 = 0;
    let mut sy_vars: HashMap<String, i32> = HashMap::new();

    //===================================================
    // Get it going?
    //===================================================
    let mut idx = 0;
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::comment => {
                sy_comments.push(SyComment {
                    span: line.as_str().to_string(),
                });
            }
            Rule::command_sy => {
                let inner = line.into_inner();
                let mut args: Vec<SyOperand> = Vec::new();
                for arg in inner {
                    match arg.as_rule() {
                        Rule::operand_noop => {
                            args.push(SyOperand {
                                noop: true,
                                int: None,
                                ident: None,
                            });
                        }
                        Rule::operand_integer => {
                            let s = arg.as_str().to_string();
                            let i = s.parse::<i32>().unwrap();
                            args.push(SyOperand {
                                noop: false,
                                int: Some(i),
                                ident: None,
                            });
                        }
                        Rule::operand_identifier => {
                            let s = arg.as_str().to_string();
                            args.push(SyOperand {
                                noop: false,
                                int: None,
                                ident: Some(s),
                            });
                        }
                        _ => {}
                    }
                }
                sy_sys.push(SyCommandSy {
                    idx,
                    arg0: args.get(0).unwrap().clone(),
                    arg1: args.get(1).unwrap().clone(),
                    arg2: args.get(2).unwrap().clone(),
                    arg3: args.get(3).unwrap().clone(),
                });
                idx += 1;
            }
            Rule::command_leaf => {
                let ident = line.into_inner().next().unwrap().as_str().to_string();
                sy_leafs.push(SyCommandLeaf { idx, ident });
            }
            Rule::EOI => {
                sy_eoi = idx;
            }
            _ => {}
        }
    }

    //===================================================
    // Checks
    //===================================================
    if sy_sys.len() > sy_eoi as usize {
        panic!("Something don't add up 1!");
    }
    for i in 0..sy_sys.len() {
        let sy_at_i = sy_sys.get(i).unwrap();
        if sy_at_i.idx != i as u32 {
            panic!("Something don't add up 2!");
        }
    }

    //===================================================
    // Actual run
    //===================================================
    let mut PC: u32 = 0;
    while PC != sy_eoi {
        // Perform sy command at 0
        let command = sy_sys.get(PC as usize).unwrap();
        // Get Value 0
        if command.arg0.noop {
            panic!("Arg0 can't be an '_'.")
        }
        let value0: i32 = match command.arg0.int {
            Some(x) => x,
            None => {
                if &command.arg0.ident.clone().unwrap() == "stdin" {
                    read_stdin()
                } else {
                    *sy_vars.get(&command.arg0.ident.clone().unwrap()).unwrap()
                }
            },
        };
        // Get Value 1
        if command.arg1.noop {
            panic!("Arg1 can't be an '_'.")
        }
        let value1: i32 = match command.arg1.int {
            Some(x) => x,
            None => {
                if &command.arg1.ident.clone().unwrap() == "stdin" {
                    read_stdin()
                } else {
                    *sy_vars.get(&command.arg1.ident.clone().unwrap()).unwrap()
                }
            },
        };
        // Calc result
        let result = value0 - value1;
        // Find and Store result in value 2
        if !command.arg2.noop {
            match &command.arg2.ident {
                Some(x) => {
                    if x == "stdout" {
                        print!("{}", char::from_u32(result as u32).unwrap());
                    }
                    else {
                        sy_vars.insert(x.to_string(), result);
                    }
                }
                None => {
                    panic!("Arg2 must be '_' or an identifier.")
                }
            }
        }
        // jump to right location (arg3)
        if result > 0 || command.arg3.noop {
            PC += 1;
        } else {
            // get arg 3
            match &command.arg3.ident {
                Some(x) => {
                    // Get leaf value
                    let mut found = false;
                    for leaf in &sy_leafs {
                        if &leaf.ident == x {
                            PC = leaf.idx;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        panic!("Arg3 must be either '_' or a known identifier.")
                    }
                }
                None => {
                    panic!("Arg3 must be '_' or an identifier.")
                }
            }
        }
    }
}
