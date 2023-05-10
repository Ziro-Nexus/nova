use proc_macro2::{Group, TokenStream, TokenTree};
use std::{fmt::Display, error::Error, io::ErrorKind};

use crate::var_table::{self, vtable::Value};

// importing nova builtin
use super::nova_builtin::std_print;
use super::nova_builtin::math_sum;
use super::nova_builtin::math_is_positive;

use super::nova_builtin::os_args;
use super::nova_builtin::os_run;


pub struct NovaModules {
    modules: Vec<(String, fn(Vec<Value>) -> Result<Value, &'static str>)>,
}

impl NovaModules {
    pub fn new() -> Self {
        // modules attr of NovaModules receives a vector of (function name, function handler pointer)
        let modules = vec![
            ("MOD<std_print>".to_owned(), std_print as fn(Vec<_>) -> Result<Value, &'static str>),
            ("MOD<math_sum>".to_owned(), math_sum as fn(Vec<_>) -> Result<Value, &'static str>),
            ("MOD<math_is_positive>".to_owned(), math_is_positive as fn(Vec<_>) -> Result<Value, &'static str>),
            ("MOD<os_args>".to_owned(), os_args as fn(Vec<_>) -> Result<Value, &'static str>),
            ("MOD<os_run>".to_owned(), os_run as fn(Vec<_>) -> Result<Value, &'static str>),
        ];

        Self { modules }
    }

    // TODO: finish this to complete "print" implementation
    pub fn novautil_idents_to_values(el: &TokenTree) -> Result<Value, &'static str> {
        unimplemented!()
    }

    pub fn novautil_literal_to_values(el: &TokenTree, v: &mut Vec<Value>, vartable: &var_table::vtable::VarTable) -> Result<Value, &'static str> {
        let mut value = Value::Null;
        // TODO: IMPLEMENT PASSING VARIABLE NAMES AS ARGUMENTS OF FUNCTIONS
        match el {
            TokenTree::Literal(lit) => {
                if let Ok(e) = lit.to_string().parse::<i64>() {
                    value = Value::Integer(e);
                    return Ok(value);
                }
                if let Ok(e) = lit.to_string().parse::<f64>() {
                    value = Value::Float(e);
                    return Ok(value);
                }
                if let Ok(e) = lit.to_string().parse::<String>() {
                    //parsing single string literal to handle break line
                    let parsed_str =
                        String::from(e.to_owned().replace("\\n", "\n").trim_matches('"'));

                    value = Value::Str(parsed_str.to_owned());
                    return Ok(value);
                }
                Ok(value)
            }
            TokenTree::Group(g) => {
               //println!("handling argument of functions as groups: {:?}", g.to_string());

                let items: Vec<String> = g.to_string().replace("(", "")
                    .replace(")", "")
                    .replace("(", "")
                    .split(",")
                    .map(|s| s.trim().to_owned())
                    .collect();

                //println!("parsed items: {items:?}");
                
                'main_mp: for item in items {
                    
                    for var in vartable.get_vars() {
                        if item.eq(var.0) {
                            v.push(var.1.clone());
                            continue 'main_mp;
                        }
                    }

                    let try_num = item.parse::<i64>();
                    let try_float = item.parse::<f64>();
                    let try_bool = item.parse::<bool>();

                    if try_num.is_ok() {v.push(Value::Integer(try_num.clone().unwrap()))}
                    else {
                        if try_float.is_ok() {v.push(Value::Float(try_float.clone().unwrap()))}
                        else {
                            if try_bool.is_ok() {v.push(Value::Boolean(try_bool.clone().unwrap()))}
                        }
                    }

                    if try_num.is_err() && try_bool.is_err() && try_float.is_err() {
                        v.push(Value::Str(item));
                    }
                }
                //println!("final parsing: {:?}", v);
                
                return Ok(Value::Null)
            },
            _ => Err("Error parsing literal"),
        }
    }

    pub fn get_modules(&self) -> &Vec<(String, fn(Vec<Value>) -> Result<Value, &'static str>)> {
        &self.modules
    }

    // handle module calls check if there is a match between "ident_str" and some Module saved in the vartable
    pub fn handle_module_calls(
        &self,
        ident_str: String,
        vartable: &var_table::vtable::VarTable,
        stream: TokenStream,
    ) -> Result<Value, &'static str> {
        let mut value_ret = Err("Failed parsing Function call");
        'main_loop: for module in self.get_modules() {
            // check if some MOD<> is integrated into the vartable
            let table_option = vartable.get(&module.0);
            
            if table_option.is_some() {
                let table = table_option.unwrap();
                
                match table {
                    Value::Module(m) => {
                        
                        for included_function in m.1.iter() {
                            if included_function.eq(&ident_str) {
                                let mut parsed_args: Vec<Value> = Vec::new();
                                for v in stream.clone().into_iter() {
                                    
                                    /*match v.clone() {
                                        TokenTree::Group(g) => println!("group to handle: {}", g.to_string()),
                                        TokenTree::Ident(i) => println!("ident to handle: {}", i.to_string()),
                                        _ => ()
                                    }*/
                                    let value = NovaModules::novautil_literal_to_values(&v.clone(), &mut parsed_args, vartable);
                                   
                                    
                                    if value.is_ok() {
                                           
                                        let mut value = value.unwrap();
                                        let value_copy = value.clone();
                                        
                                        // parse variable interpolation
                                        match value_copy {
                                            Value::Str(e) => {
                                                //println!("string argument: {e}");
                                                
                                                if e.contains('[') && e.contains(']') {
                                                    //println!("module value : {e}");
                                                    let _g = syn::parse_str::<Group>(&format!(
                                                        "({})",
                                                        e
                                                    ))
                                                    .expect("error parsing var");
                                                    
                                                    let mut resolved_value = vartable
                                                        .parse_string_vars(e.to_owned())
                                                        .expect("error parsing var");

                                                    // removing parentesis after grouping and resolving interpolation
                                                    //resolved_value.remove(resolved_value.len() - 1);
                                                    //resolved_value.remove(0);
                                                    // removing quotes from String
                                                    resolved_value =
                                                        resolved_value.replace("\"", "");

                                                        

                                                    parsed_args.push(Value::Str(resolved_value));
                                                }
                                            }
                                            Value::Float(f) => parsed_args.push(Value::Float(f)),
                                            _ => (),
                                        }

                                        //println!("New argument of {} with value: {:?} is being parsed", included_function, value);
                                        //parsed_args.push(value);
                                    }
                                }
                                value_ret = module.1(parsed_args);
                                break 'main_loop;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        return value_ret;
    }
}
