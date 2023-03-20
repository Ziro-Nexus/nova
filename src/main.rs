mod tokenizer;
mod sym_table;
mod shell;

use tokenizer::token::Token;
//use sym_table::symbols::SymbolTable;
//use sym_table::symbols::Value::*;
fn main() {

    let st = "
        '2+2';
        2 + 2
    ";

    let mut token = Token::new(st, false);
    println!("{:#?}", token);
    token.generate_subtokens();
    println!("{:#?}", token.sub_tokens);

}
