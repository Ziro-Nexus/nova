use std::fmt::format;

use crate::nova_interpreter::nova_modules::NovaModules;

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
                    // TODO: handle TokenTree::Group to parse full expressions
                    TokenTree::Group(g) => {
                        // TODO: VERY IMPORTANT, HANDLE THE REMOVE OF WHITESPACES BEFORE EVAL
                        let mut group_expr = g.to_string().to_owned();

                        for var in vartable.get_vars() {
                            //println!("URGENT DEBUG. FIX BAD VARIABLE RESOLVING: {group_expr} : {}", var.0);
                            
                            if g.to_string().contains(format!("[{}]", var.0.as_str()).as_str()) {
                                
                                group_expr = match var.1 {
                                    Value::Integer(i) => {
                                        println!("{group_expr}"); // group_expr is a string = "({ a } == { false })"
                                        group_expr = group_expr.replace(format!("[{}]", var.0).as_str(), &i.to_string());
                                        group_expr = group_expr.replace("{", "");
                                        group_expr.replace("}", "")
                                    }
                                    Value::Float(f) => {
                                        group_expr = group_expr.replace(format!("[{}]", var.0).as_str(), &f.to_string());
                                        group_expr = group_expr.replace("{", "");
                                        group_expr.replace("}", "")
                                    }
                                    Value::Boolean(b) => {
                                        group_expr = group_expr.replace(format!("[{}]", var.0).as_str(), &b.to_string());
                                        group_expr = group_expr.replace("{", "");
                                        group_expr.replace("}", "")
                                    }
                                    Value::Str(s) => {
                                        group_expr = group_expr.replace(format!("[{}]", var.0).as_str(), &s.to_string());
                                        group_expr = group_expr.replace("{", "");
                                        group_expr.replace("}", "")
                                    }
                                    _ => unimplemented!(),
                                };
                            }
                        }

                        // DEBUG: GROUP OF EXPRESSIONS
                        //println!("{group_expr}");
                        println!("DEBUG WARNING: {}", group_expr);
                        group_expr = group_expr.replace("{", "");
                        group_expr = group_expr.replace("}", "");
                        println!("DEBUG WARNING: {}", group_expr);

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
                    _ => {
                        let cp_stream = handler_stream.clone();
                        let mut token_list: Vec<TokenTree> = Vec::new();
                        let mut resolved_tokens: Vec<Value> = Vec::new();

                        for tok in cp_stream {
                            let match_copy = tok.clone();
                            
                            match match_copy {
                                TokenTree::Group(_) => (),
                                TokenTree::Ident(i) => {
                                    if !i.to_string().eq("init") {
                                        token_list.push(tok)
                                    }
                                }
                                TokenTree::Punct(_) => token_list.push(tok),
                                TokenTree::Literal(_) => token_list.push(tok),
                            }
                        }
                        
                        id = token_list[0].to_string();

                        for tok in token_list.iter() {
                            let tok_copy = tok.clone();

                            println!("tok: {}", tok_copy);

                            // FIX "=="" HANDLER
                            if tok.to_string().eq("=") {
                                continue;
                            };

                            // DEBUG:
                            // resolving idents as var names
                            match tok {
                                TokenTree::Group(_) => todo!(),
                                TokenTree::Ident(i) => {
                                    for var in vartable.get_vars() {
                                       
                                        if var.0.eq(&i.to_string()) {
                                            resolved_tokens.push(var.1.clone());
                                        }
                                    }
                                }
                                TokenTree::Punct(p) => {
                                    resolved_tokens.push(Value::Str(p.to_string()))
                                }
                                TokenTree::Literal(_) => resolved_tokens
                                    .push(NovaModules::novautil_literal_to_values(tok).unwrap()),
                            }
                        }

                        // Now with the variables names resolved, we can evaluate the expression
                        let mut str_expr = "(".to_owned();
                        

                        for values in resolved_tokens.iter() {
                            match values {
                                Value::Integer(i) => {
                                    str_expr.push_str(i.to_string().replace("\"", "").as_str())
                                }
                                Value::Float(f) => str_expr.push_str(f.to_string().as_str()),
                                Value::Boolean(b) => str_expr.push_str(b.to_string().as_str()),
                                Value::Str(s) => {
                                    if !s.starts_with("\"") {
                                        let parsed_s = format!("\"{}", s);
                                        str_expr.push_str(&parsed_s);
                                    } else {
                                        str_expr.push_str(&s);
                                    }
                                }
                                Value::Module(_) => (),
                                Value::Null => (),
                            }
                        }

                        
                        let idx = str_expr.char_indices().nth(1).unwrap();

                        // parsing string literals
                        if !str_expr.ends_with("\"") && idx.1.eq(&'"') {
                            //println!("wow{}", str_expr);
                            str_expr = format!("{}\"", str_expr);
                           // println!("{}", str_expr);
                            
                        } else {
                            str_expr = str_expr.replace('"', "").to_owned();
                            //println!("this: {}", str_expr);
                        }
                        str_expr.push(')');
                        
                        
                        let evaluated = evalexpr::eval(&str_expr)
                            .unwrap_or_else(|e| panic!("{}", e.to_string()));

                        value = match evaluated {
                            evalexpr::Value::String(s) => Value::Str(s),
                            evalexpr::Value::Float(f) => Value::Float(f),
                            evalexpr::Value::Int(i) => Value::Integer(i),
                            evalexpr::Value::Boolean(b) => Value::Boolean(b),
                            _ => Value::Null,
                        };

                        return;
                    }
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
