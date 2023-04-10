mod ast;
mod ast_macros;
mod var_table;
mod nova_interpreter;

use ast::ast_generator;
use nova_interpreter::nova_engine::NovaEngine;


//use ast::token::Token;

use nova_interpreter::nova_modules::NovaModules;

fn main2() {
    let _ = NovaModules::new();
}


fn main() {
    
    let mut nova_engine = NovaEngine::new(
        "Main.nova".to_owned(),
    );
    
    nova_engine.grammar_parser();
    
    //println!("generated tree:");
    //println!("{:#?}", nova_engine._get_tree());
    
    println!("=================OUTPUT=====================");
    nova_engine.resolver().unwrap_or_else(|e| {
        eprintln!("{}", e)
    });
    println!("\n===========================================");

    println!("\nvariable table after runtime:");
    println!("{:?}", nova_engine.get_table());
}
