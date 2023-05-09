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
                    

                    // TODO: handle var names in variable expressions: set age = <var>;
                    (generic_val) => {
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
                        let modules = NovaModules::new();
                        let mut equal_symbol_counter = 0;

                        for tok in token_list.iter() {
                            let tok_copy = tok.clone();

                            // FIX "=="" HANDLER
                            if tok.to_string().eq("=") {
                                equal_symbol_counter += 1;
                            }

                            if tok.to_string().eq("=") && equal_symbol_counter <= 1{
                                continue;
                            };

                            // resolving idents as var names
                            match tok {
                                TokenTree::Group(_) => todo!(),
                                TokenTree::Ident(i) => {
                                    
                                    if let Ok(_mod_result) = modules.handle_module_calls(
                                        i.to_string(),
                                        &vartable,
                                        handler_stream.clone(),
                                    ) {
                                        //handle function return
                                        //sprintln!("Result: {_mod_result:?}");
                                        resolved_tokens.push(_mod_result);
                                        continue;
                                    }
                                    for var in vartable.get_vars() {
                                        if var.0.eq(&i.to_string()) {
                                            resolved_tokens.push(var.1.clone());
                                        }
                                    }
                                }
                                TokenTree::Punct(p) => {
                                    resolved_tokens.push(Value::Str(p.to_string()))
                                }
                                TokenTree::Literal(_) => resolved_tokens.push(
                                    NovaModules::novautil_literal_to_values(tok, &mut Vec::new(), &vartable)
                                        .unwrap(),
                                ),
                            }
                        }

                        // Now with the variables names resolved, we can evaluate the expression
                        let mut str_expr = "(".to_owned();
                        //println!("para resolver: {resolved_tokens:?}. SE DEBE SOLUCIONA LOS NUMEROS NEGATIVOS");
                        

                        for values in resolved_tokens.iter() {
                            match values {
                                Value::Integer(i) => {
                                    str_expr.push_str(i.to_string().replace("\"", "").as_str())
                                }
                                Value::Float(f) => str_expr.push_str(f.to_string().as_str()),
                                Value::Boolean(b) => str_expr.push_str(b.to_string().as_str()),
                                Value::Str(s) => {
                                    if !s.starts_with("\"") && !s.eq("-") && !s.eq("!") && !s.eq("&"){
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

                        let idx = str_expr.char_indices().nth(1).unwrap_or_else(|| {
                            panic!("EEROR PARSING IDX");
                        });


                        // parsing string literals
                        if !str_expr.ends_with("\"") && idx.1.eq(&'"') {
                           
                            // fixing quotes of "=" symbols
                            str_expr = format!("{}\"", str_expr);
                            if str_expr.contains("\"=\""){
                                str_expr = str_expr.replace("\"=\"", "\"=").to_owned();
                            }
                            // fixing quotes of "&&" symbol
                            if str_expr.contains("&&"){
                                str_expr = str_expr.replace("&&", "\"&&").to_owned();
                            }
                            // fixing quotes of != symbol
                            if str_expr.contains("!\"="){
                                str_expr = str_expr.replace("!\"=", "!=").to_owned();
                                str_expr.pop();
                            }
                            
                             println!("wow{}", str_expr);
                            
                            
                        } else {
                            str_expr = str_expr.replace('"', "").to_owned();
                            str_expr = str_expr.replace("\"=\"=", "==");
                            
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
                        //println!("value: {value:?}");
                        
                        return;
                    }
                }
            }
            _ => (),
        };
        //println!("value: {value:?}");
        handler_idx += 1;
    });

    if let Value::Null = value {
        eprintln!("Error: parsing variable declaration")
    } else {
        // TODO: CONFIRM IS VAR NAME ALREADY EXIST, IN THAT CASE, PANIC    
        if vartable.get(id.as_str()).is_some() {
            panic!("Variable {} already exist", id);
        }
            
        vartable.set(id, value);
    }
}
