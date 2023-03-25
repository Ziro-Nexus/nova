
use std::io::prelude::*;
use std::io::stdin;
use std::io::stdout;

// macros
use crate::print_declaration_tree;
use crate::build_expr_tree;

use super::super::var_table::vtable::VarTable;

use evalexpr::*;


pub struct CommandLineUtility<'a> {
    pub sym_table: Option<&'a mut VarTable>,
    pub prompt: String,
}

impl<'a> CommandLineUtility<'a> {
    
    pub fn new(sym_table: Option<&'a mut VarTable>, prompt: String) -> Self {
        Self {
            sym_table,
            prompt
        }
    }

    fn match_flags(args: &Vec<String>) -> bool {
        let possible_flag = &args[1];
        let flag_handler: bool = match possible_flag.as_str() {
            "-f" => {
                if args.len() >= 3 {
                    let path = &args[2];
                    let codebase = std::fs::read_to_string(path).unwrap();
                    print_declaration_tree!(codebase);
                    true
                } else {
                    println!("-f needs a file path as argument");
                    true
                }
            },
            _ => false
        };

        flag_handler
    }
    
    pub fn interactive_shell(&self) -> bool {
        
        let mut exit = false;
        
        
        while !exit {
            let mut buffer = String::new();
        
            stdout().write(self.prompt.as_bytes()).expect("Failed writing prompt");
            stdout().flush().expect("Failed flushing stdout");
            stdin().read_line(&mut buffer)
                .expect("Failed reading input line");
            
            let buffer = buffer.trim().to_string();
            if buffer.len() == 0 {
                continue;
            }
            
            let args: Vec<String> = buffer.split(" ").map(|st| String::from(st)).collect();
            //println!("{:?}", args);
            match args.get(0).unwrap().as_str() {
                "exit" => break,
                "var.grammar" => {

                    if args.len() < 2 {
                        println!("Arguments are required");
                        continue;
                    }
                    
                    if !CommandLineUtility::match_flags(&args) {
                        let mut tmp_str = String::new();
                        for val in &args[1..] {
                            tmp_str.push_str(val);
                            tmp_str.push_str(" ");
                        }
                        println!("{}", tmp_str);
                        print_declaration_tree!(tmp_str);
                    }
                },
                "math.solve" => {

                    if args.len() < 2 {
                        println!("Arguments are required");
                        continue;
                    }

                    let expr = &args[1];
                    let tree = build_expr_tree!(format!("({})", expr).as_str());
                    if let Err(e) = tree {
                        println!("error: {}", e);
                    } else {
                        let tree = tree.unwrap();
                        for exp in tree.into_iter() {
                            let res = eval(&exp.to_string());
                            if let Err(e) = res {
                                println!("error: {}", e.to_string());
                            } else {
                                let res = res.unwrap();
                                match res {
                                    Value::Int(val) => println!("{}", val),
                                    Value::Float(val) => println!("{}", val),
                                    _ => println!("Cannot calculate in this context")
                                }
                            }
                        }
                    }

                }
                _ => ()
            }
        }
        
        return exit;

    }
}
