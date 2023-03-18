mod tokenizer;

use tokenizer::token::Token;

fn main() {

    let st = "
        alloc type:s Greeting = 'hola' + 'mundo';
        if 5 > 2 :
            @print 2 + 3 * 20 - 10 / 2
        ;
    ";

    let token = Token::new(st, false);
    println!("{:#?}", token);
    println!("{:#?}", token.generate_subtokens());
}
