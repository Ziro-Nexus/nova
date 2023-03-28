mod ast;
mod ast_macros;
mod var_table;
mod nova_interpreter;

use ast::ast_generator;
use nova_interpreter::nova_engine::NovaEngine;


//use ast::token::Token;


fn main() {
    let mut nova_engine = NovaEngine::new(
        "Main.nova".to_owned(),
    );
    nova_engine.grammar_parser();
    nova_engine.resolve_idents().unwrap_or_else(|e| {
        eprintln!("{}", e)
    });
}
