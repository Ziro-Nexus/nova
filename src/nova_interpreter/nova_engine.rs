use proc_macro2::TokenStream;
use quote::ToTokens;
use proc_macro2::TokenTree;

use super::super::var_table::vtable::VarTable;
use super::declaration_matcher::variable_matcher;

use crate::build_declaration_tree;
use crate::build_function_call_tree;
use crate::build_integration_tree;
use crate::build_stdout_write_tree;




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

    pub fn _get_tree(&self) -> &Vec<TokenStream>{
        &self.syntax_tree
    }


    pub fn grammar_parser(&mut self) {
        let mut line_number = 1;

        for line in self.get_file_lines() {


            let integration_tree = build_integration_tree!(&line);
            if integration_tree.is_ok() {
                self.syntax_tree.push(integration_tree.clone().unwrap().into_token_stream());
                line_number += 1;
                continue;
            }

            let call_tree = build_function_call_tree!(&line);
            if call_tree.is_ok() {
                self.syntax_tree.push(call_tree.clone().unwrap().into_token_stream());
                line_number += 1;
                continue;
            }

            
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
            
            eprintln!("{line_number}. Error: Some errors ocurred:\n-{:?}\n-{:?}\n{:?}\n{:?}", 
                declaration_tree.err().unwrap(), 
                builtin_stdout_write.err().unwrap(),
                integration_tree.err().unwrap(),
                call_tree.err().unwrap()
            );
            
            line_number += 1;
        }
    }


    // private method used to resolver to handle the matching of ident symbols
    fn resolve_tree(&mut self, handler_stream: &TokenStream, tree: &TokenTree) {
        match &tree {
            TokenTree::Ident(e) => {
                
                // DEBUG
                //println!("{:#?}", e);
                let ident_str = e.to_string();
                if ident_str.eq("nya") {
                    println!("ident variable: {ident_str}");
                
                    // creating a temportal vartable
                    let mut temporal_vartable = self.get_table().clone();
                    
                    variable_matcher(e, handler_stream, tree, &mut temporal_vartable); //cannot borrow data in a `&` reference as mutable
                    
                    // replacing the current var_table to the new var table
                    self.var_table = temporal_vartable.to_owned();
                }
                if ident_str.eq("stdout") {
                    use super::builtin_std::std_write;
                    std_write(handler_stream, self.get_table());
                }
            },
            _ => ()
        }
    }

    // resolve Ident structures
    pub fn resolver(&mut self) -> Result<(), &'static str>{
        
        if !self.syntax_tree.is_empty() {

            let tree_clone = self.syntax_tree.clone();

            for stream in tree_clone.iter() {

                let handler_stream = &stream.clone();

                for tree in stream.into_token_stream() {
                    self.resolve_tree(handler_stream, &tree);
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
