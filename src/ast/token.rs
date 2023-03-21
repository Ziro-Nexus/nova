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

        let mut _mtchandler = MatchHandler::Unknown;

        _mtchandler = match item.as_str() {
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
        _mtchandler = match item.parse::<u32>().is_ok() {
            true => MatchHandler::LiteralNum,
            false => _mtchandler
        };

        // if the item is already a number, no needs more matchings
        if let MatchHandler::LiteralNum = _mtchandler {
            return _mtchandler;
        }

        // check for type "type:f"
        _mtchandler = match item.parse::<f32>().is_ok() {
            true => MatchHandler::LiteralDecimal,
            false => _mtchandler
        };

        // if the item is already a decimal, no needs more matchings
        if let MatchHandler::LiteralDecimal = _mtchandler {
            return _mtchandler;
        }

        // check for type type:s
        // literal string should start with ' and ends with '
        _mtchandler = match item.starts_with("'") {
            true => match item.ends_with("'") {
                true => MatchHandler::LiteralString,
                false => _mtchandler
            },
            false => _mtchandler
        };


        // check for type type:b
        _mtchandler = match item.as_str() {
            "true" => MatchHandler::LiteralBoolTrue,
            "false" => MatchHandler::LiteralBoolFalse,

            _ => _mtchandler
        };

        _mtchandler
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
    pub buffer_path: String,
    // a list of items separated by ";"
    pub items: Vec<String>,
    // a list of SubTokens
    pub sub_tokens: Option<Vec<SubToken>>
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

    pub fn generate_subtokens(&mut self) {

        let mut sub_tokens: Vec<SubToken> = Vec::new();

        for (item_id, item) in self.items.iter().enumerate() {
            // detect if the line contains a literal string or some
            
            if !item.starts_with("'") && !item.ends_with("'") {
                // if the line does not contains quotes it means no strings needs to be parsed
                let sb: Vec<String> = item.split(" ")
                    .map(|x| String::from(x))
                    .filter(|x| x.len() > 0)
                    .collect();

                for (id, sub_item) in sb.iter().enumerate() {
                    sub_tokens.push(SubToken::new(id, sub_item.clone(), item_id));
                }
            } else {
                // literal string detection in the current line
                // a string in the ziroxtranslator is detected inside '' for example 'hello world'
                // string handler will help us to parse a string subtoken without breaking the whitespace in strings
                let mut string_handler = String::new();
                
                // we need to divide the line
                for i in item.split_ascii_whitespace().enumerate() {   
                    
                    // if the current item in the line start with ' it means is a string and should be pushed to handler
                    if i.1.to_string().starts_with("'") {
                        string_handler.push_str(i.1);
                    }
                    // if the item in the line ends with ' and not start with ' it means is a closing item for a string
                    if i.1.to_string().ends_with("'") && !i.1.to_string().starts_with("'") {
                        // pushing a whitespace to not break the string
                        string_handler.push(0x20 as char);
                        string_handler.push_str(i.1);
      
                    }
                    // this means the string handler is already a valid string and should be pushed as token
                    if string_handler.starts_with("'") && string_handler.ends_with("'") {
                        sub_tokens.push(SubToken::new(i.0 - 1, string_handler.clone(), item_id));
                        string_handler.clear();
                        continue;
                    }
                    // this means we need to push an intermediate string between the opening and closing qoute
                    if !i.1.contains("'") && string_handler.starts_with("'") && !string_handler.ends_with("'"){
                        string_handler.push(0x20 as char);
                        string_handler.push_str(i.1);
                        continue;
                    }
                    
                    if !i.1.contains("'") {
                        sub_tokens.push(SubToken::new(i.0, i.1.to_string().clone(), item_id));
                    }
                }
            }
        }
        self.sub_tokens = Some(sub_tokens);
        println!("Sub tokens for {} has been generated", self.buffer_path.as_str());
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
