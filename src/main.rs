mod tokenizer;

use tokenizer::token::Token;

fn main() {
    let token = Token::new("main.ZPP");
    println!("{:#?}", token);
    println!("{:#?}", token.generate_subtokens());
}
