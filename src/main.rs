
mod ast;
mod sym_table;
mod shell;


use ast::ast_generator;
use ast::token::Token;
use sym_table::symbols::SymbolTable;






fn main() {

    let lines = std::fs::read_to_string("c.zirox").unwrap();
    print_tree!(&lines);


    //shell::zirox_shell::CommandLineUtility::new(None, "Zirox>> ".to_string())
       // .interactive_shell();
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