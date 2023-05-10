mod ast;
mod ast_macros;
mod nova_interpreter;
mod var_table;

use std::env;
use ast::ast_generator;
use nova_interpreter::nova_engine::NovaEngine;

//use ast::token::Token;

use nova_interpreter::nova_modules::NovaModules;


fn main() {
    let mut nova_engine = NovaEngine::new(env::args().nth(1).unwrap().to_owned());

    nova_engine.grammar_parser();

    //println!("generated tree:");
    //println!("{:#?}", nova_engine._get_tree());

    nova_engine
        .resolver()
        .unwrap_or_else(|e| eprintln!("{}", e));


    //println!("\nvariable table after runtime:");
    //println!("{:?}", nova_engine.get_table());
}
