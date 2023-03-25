

// macros
#[macro_export]
macro_rules! ast_tree {
    ($x:expr) => {
        super::super::ast_generator::AllocatorGrammar::translate($x)
    };
}

#[macro_export]
macro_rules! print_declaration_tree {
    ($lines:expr) => {
        use crate::ast_tree;
        let mut line_number = 0;
        for line in $lines.lines() {
            let gen = ast_tree!(line);
    
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
