use std::fmt::Debug;
use std::io::prelude::*;
use std::fs;

#[derive(Debug)]
enum MatchHandler {
    AllocatorVar,
    AllocatorAssign,
    Procedure,
    ProcedureTrigger,
    ProcedureSeparator,
    TypeNum,
    TypeNumDecimal,
    TypeBool,
    TypeString,
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

impl MatchHandler {
    pub fn new(item: &String) -> Self {
        match item.as_str() {
            "alloc" => MatchHandler::AllocatorVar,
            "=" => MatchHandler::AllocatorAssign,
            "proc"=> MatchHandler::Procedure,
            "@" => MatchHandler::ProcedureTrigger,
            "type:n" => MatchHandler::TypeNum,
            "type:f" => MatchHandler::TypeNumDecimal,
            "type:b" => MatchHandler::TypeBool,
            "type:s" => MatchHandler::TypeString,
            ":" => MatchHandler::ProcedureSeparator,
            "+" => MatchHandler::ExprSum,
            "-" => MatchHandler::ExprSubs,
            "*" => MatchHandler::ExprMultiplication,
            "/" => MatchHandler::ExprDivision,
            "if" => MatchHandler::Conditional,
            ">" => MatchHandler::Greater,
            "<" => MatchHandler::Less,
            "eq" => MatchHandler::Equal,
            "not" => MatchHandler::ISNT,


            &_ => MatchHandler::Unknown,
        }
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
    pub fn new(codebase_path: &str) -> Self {
        let codebase = fs::read_to_string(codebase_path).unwrap_or_else(|err| {
            panic!("{}", err.to_string())
        });

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
            // sb = subtokens
            let sb: Vec<String> = item.split(" ")
                .map(|x| String::from(x))
                .filter(|x| x.len() > 0)
                .collect();
            for (id, sub_item) in sb.iter().enumerate() {
                sub_tokens.push(SubToken::new(id, sub_item.clone(), item_id));
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