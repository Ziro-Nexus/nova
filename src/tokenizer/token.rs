use std::fmt::Debug;
use std::io::prelude::*;
use std::fs;

#[derive(Debug)]
enum MatchHandler {
    AllocatorVar,
    AllocatorAssign,
    Procedure,
    ProcedureTrigger,
    Then,
    TypeNum,
    TypeNumDecimal,
    TypeBool,
    TypeString,
    LiteralNum,
    LiteralDecimal,
    LiteralString,
    LiteralBoolTrue,
    LiteralBoolFalse,
    ExprSum,
    ExprSubs,
    ExprMultiplication,
    ExprDivision,
    Conditional,
    Less,
    Greater,
    Equal,
    ISNT,
    
    Unknown
}

// It is used to define the behavior of a subtoken
impl MatchHandler {
    pub fn new(item: &String) -> Self {

        let mut mtchandler = MatchHandler::Unknown;

        mtchandler = match item.as_str() {
            // Matching handler for general keys
            "alloc" => MatchHandler::AllocatorVar,
            "=" => MatchHandler::AllocatorAssign,
            "proc"=> MatchHandler::Procedure,
            "type:n" => MatchHandler::TypeNum,
            "type:f" => MatchHandler::TypeNumDecimal,
            "type:b" => MatchHandler::TypeBool,
            "type:s" => MatchHandler::TypeString,
            ":" => MatchHandler::Then,
            "+" => MatchHandler::ExprSum,
            "-" => MatchHandler::ExprSubs,
            "*" => MatchHandler::ExprMultiplication,
            "/" => MatchHandler::ExprDivision,
            "if" => MatchHandler::Conditional,
            ">" => MatchHandler::Greater,
            "<" => MatchHandler::Less,
            "eq" => MatchHandler::Equal,
            "not" => MatchHandler::ISNT,
            // the las match of the general keyboards is the procedure trigger (function call)
            &_ => match item.starts_with("@") {
                true => MatchHandler::ProcedureTrigger,
                false => MatchHandler::Unknown
            }
        };

        // check for type "type:n"
        mtchandler = match item.parse::<u32>().is_ok() {
            true => MatchHandler::LiteralNum,
            false => mtchandler
        };

        // if the item is already a number, no needs more matchings
        if let MatchHandler::LiteralNum = mtchandler {
            return mtchandler;
        }

        // check for type "type:f"
        mtchandler = match item.parse::<f32>().is_ok() {
            true => MatchHandler::LiteralDecimal,
            false => mtchandler
        };

        // if the item is already a decimal, no needs more matchings
        if let MatchHandler::LiteralDecimal = mtchandler {
            return mtchandler;
        }

        // check for type type:s
        // literal string should start with ' and ends with '
        mtchandler = match item.starts_with("'") {
            true => match item.ends_with("'") {
                true => MatchHandler::LiteralString,
                false => mtchandler
            },
            false => mtchandler
        };


        // check for type type:b
        mtchandler = match item.as_str() {
            "true" => MatchHandler::LiteralBoolTrue,
            "false" => MatchHandler::LiteralBoolFalse,

            _ => mtchandler
        };

        mtchandler
    } 
}


// A SubToken represent a structure of a single item(word) in the textfile
#[derive(Debug)]
pub struct SubToken {
    // represent the position of the token in the item
    id: usize,
    // the item in string format
    item: String,
    // the position of the item in the main Token
    item_id: usize,
    // represent how this SubToken should be handled
    handler: MatchHandler
}

// creating the subtokens and generating the matchhandlers
impl SubToken {
    pub fn new(id: usize, item: String, item_id: usize) -> Self {
        let handler = MatchHandler::new(&item);
        SubToken { id, item, item_id, handler}
    }
}

// A Token represent a structure to the file that should be readed by the interpreter
pub struct Token {
    // represent the entire string readed in the file
    buffer_path: String,
    // a list of items separated by ";"
    items: Vec<String>,
    // a list of SubTokens
    sub_tokens: Option<Vec<SubToken>>
}


// Create a new Token
impl Token {
    pub fn new(codebase_path: &str, file_path: bool) -> Self {

        let mut codebase: String = String::from(codebase_path);

        if file_path {
            codebase = fs::read_to_string(codebase_path).unwrap_or_else(|err| {
                panic!("{}", err.to_string())
            });
        }

        println!("{:?}", codebase.as_bytes());

        //Delete "\n" in the entire buffer
        let codebase: String = codebase.chars()
                            .filter(|c| *c as u8 != 10)
                            .collect();

        Token {
            buffer_path: codebase_path.to_string(),
            sub_tokens: None,
            items: codebase.split(";").map(|raw_str| 
                raw_str.to_string()
                    .trim()
                    .to_string()
            ).filter(|parsed_string| parsed_string.len() > 0)
             .collect(),
              
        }
    }

    pub fn generate_subtokens(&self) -> Vec<SubToken> {

        let mut sub_tokens: Vec<SubToken> = Vec::new();

        for (item_id, item) in self.items.iter().enumerate() {
            // detect if the line contains a literal string or some
            if !item.starts_with("'") && !item.ends_with("'") {
                let sb: Vec<String> = item.split(" ")
                    .map(|x| String::from(x))
                    .filter(|x| x.len() > 0)
                    .collect();
                for (id, sub_item) in sb.iter().enumerate() {
                    sub_tokens.push(SubToken::new(id, sub_item.clone(), item_id));
                }
            } else {
                // literal string detection in the current line!
                for i in item.split_ascii_whitespace().enumerate() {
                    sub_tokens.push(SubToken::new(i.0, i.1.to_string(), item_id));
                }
            }
            

        }

        sub_tokens

    }
}


// Create a new Sub Token that handles Token.items



impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("buffer", &self.buffer_path)
            .field("items", &self.items)
            .finish()
    }
}