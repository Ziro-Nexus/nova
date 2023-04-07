use super::super::var_table::vtable::Value;
use super::super::var_table::vtable::VarTable;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::ToTokens;

pub fn std_write(handler_stream: &TokenStream, table: &VarTable) {
    let v = handler_stream
        .clone()
        .into_token_stream()
        .into_iter()
        .last()
        .unwrap()
        .to_string();

    let val = table
        .get(v.as_str())
        .unwrap_or_else(|| panic!("undeclared variable {v}"));

    match val {
        Value::Integer(e) => print!("{}", e),
        Value::Float(f) => print!("{:.2}", f),
        Value::Str(s) => print!("{}", s.to_string()),
        Value::Boolean(s) => print!("{}", s),
        _ => (),
    }
}
