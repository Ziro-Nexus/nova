use std::borrow::BorrowMut;
use std::ops::DerefMut;

use evalexpr::eval_boolean;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::ToTokens;
use proc_macro2::TokenTree;
use super::super::var_table::vtable::VarTable;
use super::super::var_table::vtable::Value;

// Create a matcher to handle the tokens of type Ident
pub fn variable_matcher(e: &Ident, handler_stream: &TokenStream, tree: &TokenTree, vartable: &mut VarTable) {
    match e.to_string().as_str() {
        "nya" => {
            let v = handler_stream.into_token_stream();

            let mut id = String::new();
            let mut value = Value::Null;

            let mut handler_idx = 0;
            v.into_iter().for_each(|el| {
                match handler_idx {
                    1 => id = el.to_string(), // getting the name of the variabe
                    3 => { // getting the value of the variable
                        // TODO: FIX LITERALS WITHOUT PARENTESIS LIKE: 2+2+2.
                        // TODO: FIX PASSING VARIABLES AS LITERALS
                        match el {
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
                                    let parsed_str = String::from(
                                        e.to_owned().replace("\\n", "\n")
                                        .trim_matches('"')
                                    );
                                    
                                    value = Value::Str(parsed_str.to_owned());
                                    return;
                                }
                                
                            },
                            
                            // TODO: handle TokenTree::Group to parse full expressions
                            TokenTree::Group(g) => {
                                // TODO: VERY IMPORTANT, HANDLE THE REMOVE OF WHITESPACES BEFORE EVAL
                                let mut group_expr = g.to_string();

                                // this handle the whitespace when passing variables. But i don't think is the best way to do it
                                if group_expr.contains("var ::") {
                                    group_expr = group_expr.replace(" ", "");
                                }
                                

                                // TODO: HANDLE STRING INTERPOLATION:GROUP                                                        
                                for x in vartable.get_vars() {
                                    
                                    if group_expr.contains(format!("var::{}", x.0).as_str()) {
                                        
                                        match x.1 {
                                            Value::Integer(i) => {
                                                group_expr = group_expr.replace(format!("var::{}", x.0).as_str(), &i.to_string());
                                            },
                                            Value::Str(s) => {
                                                // if is a string, the variable value should be inside double quotes
                                                let s = format!("\"{}\"", s);

                                                group_expr = group_expr.replace(format!("var::{}", x.0).as_str(), &format!("{}", &s.as_str()));
                                            },
                                            // TODO: fix float values unexpected converted to integer values
                                            Value::Float(f) => {
                                                
                                                group_expr = group_expr.replace(format!("var::{}", x.0).as_str(), &format!("{:.2}", f));
                                            
                                            },
                                            Value::Boolean(b) => {
                                                group_expr = group_expr.replace(format!("var::{}", x.0).as_str(), &b.to_string());
                                            
                                            },
                                            _ => panic!("Error variable in expression")
                                        }
                                    }
                                }
                                // DEBUG: GROUP OF EXPRESSIONS
                                //println!("{group_expr}");
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
                                        _ => eprintln!("error parsing expression")
                                    }   
                                }
                            }
                            
                            // TODO: handle var names in variable expressions: set age = <var>;
                            _ => eprintln!("Error: parsing literal")
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
        },
        // TODO: CREATE AN EXTERNET FUNCTION TO HANDLE THIS BUILTIN FUNCTION
        "stdout" => {
            use super::builtin_std::std_write;
            std_write(handler_stream, &vartable.clone());
        },
        _ => ()
    }
}