use proc_macro2::TokenStream;
use quote::ToTokens;
use std::env::args;
use proc_macro2::TokenTree;

use proc_macro2::TokenTree::Group;
use super::super::var_table::vtable::VarTable;
use super::super::var_table::vtable::Value;

use crate::build_declaration_tree;





pub struct NovaEngine {
    filepath: String,
    var_table: VarTable,
    syntax_tree: Vec<TokenStream>
}

impl NovaEngine {
    pub fn new(filepath: String) -> Self {
        let syntax_tree = Vec::new();
        let var_table = VarTable::new();
        Self {
            filepath,
            var_table,
            syntax_tree
        }
    }

    pub fn get_table(&self) -> &VarTable {
        &self.var_table
    }


    pub fn grammar_parser(&mut self) {
        let mut line_number = 0;

        for line in self.get_file_lines() {
            let checks = [
                build_declaration_tree!(&line),
                //build_expr_tree!(&line)

            ].iter().for_each(|result| {
                if result.is_ok() {
                    self.syntax_tree.push(result.clone().unwrap().into_token_stream());
                } else {
                    eprintln!("{}", &result.clone().err().unwrap())
                }
            });
            line_number += 1;
        }
    }

    // resolve Ident structure like AllocatorGrammar
    pub fn resolve_idents(&mut self) -> Result<(), &'static str>{
        
        if !self.syntax_tree.is_empty() {

            for stream in self.syntax_tree.iter() {

                let handler_stream = &stream.clone();

                for tree in stream.into_token_stream() {
                    match &tree {
                        TokenTree::Ident(e) => {
                            match e.to_string().as_str() {
                                "set" => {
                                    println!("set trigger called: {}", &handler_stream.to_string());
                                    let v = handler_stream.into_token_stream();

                                    let mut id = String::new();
                                    let mut value = Value::Null;

                                    let mut handler_idx = 0;
                                    v.into_iter().for_each(|el| {
                                        match handler_idx {
                                            1 => id = el.to_string(), // getting the name of the variabe
                                            3 => { // getting the value of the variable
                                                match el {
                                                    TokenTree::Literal(lit) => {
                                                        if let Ok(e) = lit.to_string().parse::<i64>() {
                                                             value = Value::Integer(e);
                                                             return; 
                                                        }
                                                        if let Ok(e) = lit.to_string().parse::<f64>() {
                                                            value = Value::Float(e);
                                                            return;
                                                        }
                                                        if let Ok(e) = lit.to_string().parse::<String>() {
                                                            value = Value::Str(e.clone());
                                                            return; 
                                                        }
                                                    }
                                                    // TODO: handle TokenTree::Group to parse full expressions
                                                    // TODO: handle var names in variable expressions: set age = <var>;
                                                    _ => eprintln!("Error: parsing literal")
                                                }
                                            }
                                            _ => (),
                                        };
                                        handler_idx += 1;
                                    });
                                    
                                    if let Value::Null = value {
                                        eprintln!("Error: parsing variable declaration")
                                    } else {
                                        // TODO: CONFIRM IS VAR NAME ALREADY EXIST, IN THAT CASE, PANIC
                                        self.var_table.set(id.clone(), value);
                                        println!("Nueva variable cargada en memoria con id = '{id}'");
                                    }
                                },
                                _ => println!("UNHANDLED IDENT: {} needs to be handled", e.to_string())
                            }
                        },
                        _ => continue
                    }
                }
            }

        } else {
            return Err("syntax tree has errors");
        }

        Ok(())
    }

    // get the lines of the current file buffer loaded
    pub fn get_file_lines(&self) -> Vec<String> {
       std::fs::read_to_string(self.filepath.as_str())
            .expect("Failed opening file")
            .split(";").map(|line| line.trim().to_owned())
            .filter(|line| !line.is_empty())
            .collect()
    }
}
