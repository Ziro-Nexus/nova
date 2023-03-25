

// macros for AllocatorGrammar
#[macro_export]
macro_rules! build_declaration_tree {
    ($x:expr) => {
        crate::ast_generator::AllocatorGrammar::translate($x)
    };
}

#[macro_export]
macro_rules! print_declaration_tree {
    ($lines:expr) => {
        
        let mut line_number = 0;
        use crate::build_declaration_tree;
        for line in $lines.lines() {
            let gen = build_declaration_tree!(line);
    
            if let Err(e) = gen {
                println!("{}: ERROR INFO = {}", line_number, e);
            } else {
                let gen = gen.ok().unwrap();
                println!("LINE {}", line_number);
                println!("{}", gen);
                println!("{:#?}", gen);
                println!("----------------------------------------");
                line_number += 1;
            }
        }
    };
}


// macros for ExprGrammar
#[macro_export]
macro_rules! build_expr_tree {
    ($x:expr) => {
        crate::ast_generator::ExprGrammar::translate($x)
    };
}
