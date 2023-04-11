use super::super::var_table::vtable::Value;
use super::super::var_table::vtable::VarTable;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::ToTokens;

// Create a matcher to handle the tokens of type Ident>>
pub fn variable_matcher(
    _e: &Ident,
    handler_stream: &TokenStream,
    _tree: &TokenTree,
    vartable: &mut VarTable,
) {
    let v = handler_stream.into_token_stream();

    let mut id = String::new();
    let mut value = Value::Null;

    let mut handler_idx = 0;
    v.into_iter().for_each(|el| {
        match handler_idx {
            1 => id = el.to_string(), // getting the name of the variabe
            3 => {
                // getting the value of the variable
                // TODO: FIX LITERALS WITHOUT PARENTESIS LIKE: 2+2+2.
                // TODO: FIX PASSING VARIABLES AS LITERALS
                match el {
                    TokenTree::Ident(i) => {
                        println!("new ident detected for declaration {:?} : full stream: {}", i, handler_stream);
                        unimplemented!("parsing declaration values as Idents");
                    },
                    TokenTree::Literal(lit) => {
                        if let Ok(e) = lit.to_string().parse::<i64>() {
                            value = Value::Integer(e);
                            return;
                        }
                        if let Ok(e) = lit.to_string().parse::<f64>() {
                            value = Value::Float(e);
                            return;
                        }
                        if let Ok(e) = lit.to_string().parse::<String>() {
                            //parsing single string literal to handle break line
                            let parsed_str =
                                String::from(e.to_owned().replace("\\n", "\n").trim_matches('"'));

                            value = Value::Str(parsed_str.to_owned());
                            return;
                        }
                    }

                    // TODO: handle TokenTree::Group to parse full expressions
                    TokenTree::Group(g) => {
                        // TODO: VERY IMPORTANT, HANDLE THE REMOVE OF WHITESPACES BEFORE EVAL
                        let group_expr = vartable.parse_group_vars(g).unwrap();
                        // DEBUG: GROUP OF EXPRESSIONS
                        println!("{group_expr}");
                        //println!("{}", group_expr);

                        let eval_result = evalexpr::eval(&group_expr.replace("\\n", "\n"));

                        if let Err(e) = eval_result {
                            eprintln!("{}", e);

                            return;
                        } else {
                            let eval_result = eval_result.unwrap();

                            match eval_result {
                                evalexpr::Value::Int(i) => value = Value::Integer(i),
                                evalexpr::Value::String(s) => value = Value::Str(s),
                                evalexpr::Value::Float(f) => value = Value::Float(f),
                                evalexpr::Value::Boolean(f) => value = Value::Boolean(f),
                                _ => eprintln!("error parsing expression"),
                            }
                        }
                    }

                    // TODO: handle var names in variable expressions: set age = <var>;
                    (e) => eprintln!("Error: parsing literal"),
                }
            }
            _ => (),
        };
        handler_idx += 1;
    });

    if let Value::Null = value {
        eprintln!("Error: parsing variable declaration")
    } else {
        // TODO: CONFIRM IS VAR NAME ALREADY EXIST, IN THAT CASE, PANIC
        vartable.set(id, value);
    }
}
