mod ast;
mod ast_macros;
mod var_table;
mod shell;
mod nova_interpreter;

use ast::ast_generator;
use nova_interpreter::nova_engine::NovaEngine;
use std::env::args;

//use ast::token::Token;


fn main() {
    let mut nova_engine = NovaEngine::new(
        "c.zirox".to_owned(),
    );
    nova_engine.grammar_parser();
    nova_engine.resolve_idents().unwrap_or_else(|e| {
        eprintln!("{}", e)
    });
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