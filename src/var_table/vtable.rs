use std::collections::HashMap;

pub struct VarTable{
    pub symbols: HashMap<String, Value>,
}

impl VarTable{
    pub fn new() -> VarTable{
        VarTable{
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
}

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Str(String),
    Null,
}
