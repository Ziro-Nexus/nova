use std::env::args;
use std::io::prelude::*;
use std::io::stdin;
use std::io::stdout;
use proc_macro2::TokenTree;
use crate::print_declaration_tree;

use super::super::sym_table::symbols::SymbolTable;

use super::super::ast_macros::build_macros;


pub struct CommandLineUtility<'a> {
    pub sym_table: Option<&'a mut SymbolTable>,
    pub prompt: String,
}

impl<'a> CommandLineUtility<'a> {
    
    pub fn new(sym_table: Option<&'a mut SymbolTable>, prompt: String) -> Self {
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
                "AllocGrammar=>" => {

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
                }
                _ => ()
            }
        }
        
        return exit;

    }
}
