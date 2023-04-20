use std::collections::HashMap;

#[derive(Debug)]
pub struct VarTable {
    pub symbols: HashMap<String, Value>,
}

impl VarTable {
    pub fn new() -> VarTable {
        VarTable {
            symbols: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.symbols.get(name)
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.symbols.insert(name, value);
    }

    pub fn get_vars(&self) -> &HashMap<String, Value> {
        &self.symbols
    }

    pub fn parse_string_vars(&self, val: String) -> Result<String, &'static str> {
        let mut group_expr = val.to_owned();
        println!("{group_expr}");

        // this handle the whitespace when passing variables. But i don't think is the best way to do it
        if group_expr.contains("var ::") {
            group_expr = group_expr.replace(" ", "");
        }

        // TODO: HANDLE STRING INTERPOLATION:GROUP
        for x in self.get_vars() {
            if group_expr.contains(format!("var::{}", x.0).as_str()) {
                match x.1 {
                    Value::Integer(i) => {
                        group_expr =
                            group_expr.replace(format!("var::{}", x.0).as_str(), &i.to_string());
                    }
                    Value::Str(s) => {
                        // if is a string, the variable value should be inside double quotes
                        let s = format!("\"{}\"", s);

                        group_expr = group_expr.replace(
                            format!("var::{}", x.0).as_str(),
                            &format!("{}", &s.as_str()),
                        );
                    }
                    // TODO: fix float values unexpected converted to integer values
                    Value::Float(f) => {
                        group_expr = group_expr
                            .replace(format!("var::{}", x.0).as_str(), &format!("{}", f));
                    }
                    Value::Boolean(b) => {
                        group_expr =
                            group_expr.replace(format!("var::{}", x.0).as_str(), &b.to_string());
                    }
                    _ => panic!("Error variable in expression"),
                }
            }
        }
        //println!("{group_expr}");
        Ok(group_expr)
    }

    pub fn parse_group_vars(&self, g: proc_macro2::Group) -> Result<String, &'static str>{
        let mut group_expr = g.to_string();
        //println!("{group_expr}");

        // this handle the whitespace when passing variables. But i don't think is the best way to do it
        if group_expr.contains("var ::") {
            group_expr = group_expr.replace(" ", "");
        }

        // TODO: HANDLE STRING INTERPOLATION:GROUP
        for x in self.get_vars() {
            if group_expr.contains(format!("var::{}", x.0).as_str()) {
                match x.1 {
                    Value::Integer(i) => {
                        group_expr =
                            group_expr.replace(format!("var::{}", x.0).as_str(), &i.to_string());
                    }
                    Value::Str(s) => {
                        // if is a string, the variable value should be inside double quotes
                        let s = format!("\"{}\"", s);

                        group_expr = group_expr.replace(
                            format!("var::{}", x.0).as_str(),
                            &format!("{}", &s.as_str()),
                        );
                    }
                    // TODO: fix float values unexpected converted to integer values
                    Value::Float(f) => {
                        group_expr = group_expr
                            .replace(format!("var::{}", x.0).as_str(), &format!("{:.2}", f));
                    }
                    Value::Boolean(b) => {
                        group_expr =
                            group_expr.replace(format!("var::{}", x.0).as_str(), &b.to_string());
                    }
                    _ => panic!("Error variable in expression"),
                }
            }
        }
        //println!("{group_expr}");
        Ok(group_expr)
    }
}

impl Clone for VarTable {
    fn clone(&self) -> Self {
        let cloned_map = self.symbols.clone();
        VarTable {
            symbols: cloned_map,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    // represent a 64 bytes rs integer
    Integer(i64),
    // represent a 64 bytes rs float
    Float(f64),
    // represent a boolean
    Boolean(bool),
    // represent a heap allocated String
    Str(String),
    // Module represent a third party or builtin Module to include
    // EXAMPLE: MODULE ("stdio", ["print", "read", ...])
    Module((String, Vec<String>)),
    // represent a null value
    // TODO: CREATE A HANDLER FOR NULL
    Null,
}
