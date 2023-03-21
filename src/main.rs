
mod ast;
mod sym_table;
use ast::ast_generator;
mod shell;

use sym_table::symbols::SymbolTable;

fn main() {

    

    shell::zirox_shell::CommandLineUtility::new(None, "Zirox>> ".to_string())
        .interactive_shell();
}