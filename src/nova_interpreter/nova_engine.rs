use proc_macro2::TokenStream;
use quote::ToTokens;
use std::env::args;
use std::io::Read;
use proc_macro2::TokenTree;

use proc_macro2::TokenTree::Group;
use super::super::var_table::vtable::VarTable;
use super::super::var_table::vtable::Value;

use crate::build_declaration_tree;
use crate::build_stdout_write_tree;

use evalexpr;



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
    pub fn get_tree(&self) -> &Vec<TokenStream>{
        &self.syntax_tree
    }


    pub fn grammar_parser(&mut self) {
        let mut line_number = 1;

        for line in self.get_file_lines() {
            // loading syntax tree for builtin functions
            // TODO: HANDLE ERRORS IN SINTAX
            let builtin_stdout_write = build_stdout_write_tree!(&line);

            if builtin_stdout_write.is_ok() {
                self.syntax_tree.push(builtin_stdout_write.clone().unwrap().into_token_stream());
                line_number += 1;
                continue;
            }
            
            // loading syntax tree for variable declaration
            // TODO: HANDLE ERRORS IN SINTAX
            let declaration_tree = build_declaration_tree!(&line);

            if declaration_tree.is_ok() {
                self.syntax_tree.push(declaration_tree.clone().unwrap().into_token_stream());
                line_number += 1;
                continue;
            }

            eprintln!("{line_number}. Error: Some errors ocurred:\n-{:?}\n-{:?}", declaration_tree.err(), builtin_stdout_write.err());
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
                                    let v = handler_stream.into_token_stream();

                                    let mut id = String::new();
                                    let mut value = Value::Null;

                                    let mut handler_idx = 0;
                                    v.into_iter().for_each(|el| {
                                        match handler_idx {
                                            1 => id = el.to_string(), // getting the name of the variabe
                                            3 => { // getting the value of the variable
                                                // TODO: FIX LITERALS WITHOUT PARENTESIS LIKE: 2+2+2.
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
                                                            
                                                            //parsing single string literal to handle break line
                                                            let parsed_str = String::from(
                                                                e.to_owned().replace("\\n", "\n")
                                                                .trim_matches('"')
                                                            );
                                                            
                                                            value = Value::Str(parsed_str.to_owned());
                                                            return;
                                                        }
                                                        
                                                    },
                                                    
                                                    // TODO: handle TokenTree::Group to parse full expressions
                                                    TokenTree::Group(g) => {
                                                        let group_expr = g.to_string();
                                                        let eval_result = evalexpr::eval(&group_expr);

                                                        if let Err(e) = eval_result {
                                                            eprintln!("{}", e);
                                                            return;
                                                        } else {
                                                            let eval_result = eval_result.unwrap();
                                                            match eval_result {
                                                                evalexpr::Value::Int(i) => value = Value::Integer(i),
                                                                evalexpr::Value::String(s) => value = Value::Str(s),
                                                                evalexpr::Value::Float(f) => value = Value::Float(f),
                                                                evalexpr::Value::Boolean(f) => value = Value::Boolean(f),
                                                                _ => eprintln!("error parsing expression")
                                                            }   
                                                        }
                                                    }
                                                    
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
                                    }
                                },
                                // TODO: CREATE AN EXTERNET FUNCTION TO HANDLE THIS BUILTIN FUNCTION
                                "stdout" => {
                                    let v = handler_stream.into_token_stream().into_iter().last().unwrap().to_string();
                                    
                                    let val = self.get_table().get(v.as_str()).unwrap_or_else(|| {
                                        panic!("undeclared variable {v}")
                                    });

                                    match val {
                                        Value::Integer(e) => print!("{}", e),
                                        Value::Float(f) => print!("{}", f),
                                        Value::Str(s) => print!("{}", s.to_string()),
                                        Value::Boolean(s) => print!("{}", s),
                                        _ => ()
                                    }
                                },
                                _ => ()
                            }
                        },
                        _ => continue
                    }
                }
            }

        } else {
            return Err("syntax tree is empty");
        }

        Ok(())
    }

    // get the lines of the current file buffer loaded
    pub fn get_file_lines(&self) -> Vec<String> {
       std::fs::read_to_string(self.filepath.as_str())
            .expect("Failed opening file")
            .split(";").map(|line| line.trim().to_owned())
            .filter(|line| !line.is_empty().to_owned())
            .collect()
    }
}
