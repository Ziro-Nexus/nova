mod ast;
mod ast_macros;
use crate::ast_macros::build_macros;
mod var_table;
mod shell;

use ast::ast_generator;
use proc_macro2::TokenStream;
use quote::ToTokens;
//use ast::token::Token;
use var_table::vtable::VarTable;
use std::env::args;
use proc_macro2::TokenTree;
use proc_macro2::TokenTree::Group;
use evalexpr::*;



struct ZiroxEngine<'a> {
    filepath: String,
    var_table: &'a VarTable,
    syntax_tree: Vec<TokenStream>
}

impl<'a> ZiroxEngine<'a> {
    fn new(filepath: String, var_table: &'a mut VarTable) -> Self {
        let mut syntax_tree = Vec::new();
        Self {
            filepath,
            var_table,
            syntax_tree
        }
    }


    fn grammar_parser(&mut self) {
        let mut line_number = 0;

        for line in self.get_file_lines() {
            let checks = [
                build_declaration_tree!(&line),
                build_expr_tree!(&line)

            ].iter().for_each(|result| {
                if result.is_ok() {
                    self.syntax_tree.push(result.clone().unwrap().into_token_stream());
                }
            });
            line_number += 1;
        }
    }

    // get the lines of the current file buffer loaded
    fn get_file_lines(&self) -> Vec<String> {
       std::fs::read_to_string(self.filepath.as_str())
            .expect("Failed opening file")
            .split(";").map(|line| line.trim().to_owned())
            .filter(|line| !line.is_empty())
            .collect()
    }
}


fn main() {
    let mut vartable = VarTable::new();

    let mut z_engine = ZiroxEngine::new(
        "c.zirox".to_owned(),
        &mut vartable
    );
    z_engine.grammar_parser();

    for tstream in z_engine.syntax_tree.iter() {
        println!("{:#?}", &tstream);
    }

  
}


fn main2() {

    // interpreter mode
    let first_arg = args().nth(1).unwrap_or_else(|| {
        panic!("1-nth argument is required")
    });

    // shell mode
    match first_arg.as_str() {
        "zshell" => {

            shell::zirox_shell::CommandLineUtility::new(None, "Zirox>> ".to_string())
                .interactive_shell();

        },
        "comp" => {
            let second_command = args().nth(2).unwrap_or_else(|| {
                panic!("argument for [comp] command is required: path of the source file")
            });
            let codebase = std::fs::read_to_string(&second_command).unwrap_or_else(|e| {
                panic!("System error: {}", e.to_string())
            });

            println!("{}", codebase);

            //- grammar checker

            //- AST generation
            //type resolver
            //expression resolver

        }
        _ => println!("Invalid argument")
    }
}





#[cfg(test)]
mod tests {
    use proc_macro2::TokenTree;

    use super::*;

    #[test]
    fn test_ast_allocator_spans() {

        let expr1 = "local num = 20;".to_owned();
        let ast = ast_generator::AllocatorGrammar::translate(&expr1).unwrap();
        let mut start_lines: Vec<(usize, usize)> = Vec::new();

        for t in ast.into_iter() {
            start_lines.push((t.span().start().column, t.span().end().column));
        }
        
        let item0 = &start_lines[0];

        assert_eq!(item0.0, 0 as usize);
        assert_eq!(item0.1, 5 as usize);

        let item1 = &start_lines[1];
        assert_eq!(item1.0, 6 as usize);
        assert_eq!(item1.1, 9 as usize);

        let item2 = &start_lines[2];
        assert_eq!(item2.0, 10 as usize);
        assert_eq!(item2.1, 11 as usize);

        let item3 = &start_lines[3];
        assert_eq!(item3.0, 12 as usize);
        assert_eq!(item3.1, 14 as usize);

        let item4 = &start_lines[4];
        assert_eq!(item4.0, 14 as usize);
        assert_eq!(item4.1, 15 as usize);
    }
}