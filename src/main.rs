mod ast;
mod ast_macros;
mod nova_interpreter;
mod var_table;

use ast::ast_generator;
use nova_interpreter::nova_engine::NovaEngine;

//use ast::token::Token;

use nova_interpreter::nova_modules::NovaModules;

fn main2() {
    let enbracked_variables: Vec<String> = "El error se ha cometido en: {err} por culpa de la siguiente funcion {f}"
        .to_owned()
        .split("{")
        .map(|s| s.to_owned().split("}").nth(0).unwrap().trim().to_owned())
        //.filter(|f| f.contains("}"))
        .collect();
    let enbracked_variables = enbracked_variables[1..].to_vec();
    // imprime: ["var1", "var2", "var3", "var4"]
    println!("{:?}", enbracked_variables);
    
}

fn main() {
    let mut nova_engine = NovaEngine::new("Main.nova".to_owned());

    nova_engine.grammar_parser();

    //println!("generated tree:");
    //println!("{:#?}", nova_engine._get_tree());

    println!("=================OUTPUT=====================");
    nova_engine
        .resolver()
        .unwrap_or_else(|e| eprintln!("{}", e));
    println!("\n===========================================");

    println!("\nvariable table after runtime:");
    println!("{:?}", nova_engine.get_table());
}
