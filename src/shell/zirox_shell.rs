use std::io::prelude::*;
use std::io::stdin;
use std::io::stdout;
use proc_macro2::TokenTree;

use super::super::sym_table::symbols::SymbolTable;
use super::super::ast_generator::AllocatorGrammar;


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
                "AST" => {

                    if args.len() < 2 {
                        println!("Arguments are required");
                        continue;
                    }

                    let arg = &args[1];
                    
                    let codebase = std::fs::read_to_string(arg.as_str()).unwrap_or_else(|err| {
                        panic!("{}", err.to_string())
                    });
                    
                    for line in codebase.clone().lines() {
                        println!("line {}", line);
                        let res = AllocatorGrammar::translate(line);
                        if res.is_ok() {
                            let res = res.unwrap();

                            for t in res.into_iter() {
                                match t {
                                   TokenTree::Literal(lit)  => println!("literal value: {:#?}", lit),
                                   TokenTree::Group(g) => println!("Group value: {:#?}", g),
                                   TokenTree::Ident(id) => println!("Ident value: {:#?}", id),
                                   TokenTree::Punct(pct) => println!("Punct value: {:#?}", pct),
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
