use crate::var_table::vtable::Value;
use std::env;
use std::io::Stdin;
use std::process::{Command, Stdio};

pub fn std_print(args: Vec<Value>) -> Result<Value, &'static str>{
    for arg in args.iter() {
        match arg {
            Value::Integer(e) => print!("{}", e),
            Value::Float(f) => print!("{:.2}", f),
            Value::Str(s) => print!("{}", s.to_string()),
            Value::Boolean(s) => print!("{}", s),
            _ => eprintln!("Cannot parse value: {:?}", arg),
        }
    }
    Ok(Value::Null)
}

pub fn math_sum(args: Vec<Value>) -> Result<Value, &'static str>{
    
    let arg0 = &args[0];

    let posy = &args[1];

    let val1: i64;
    let val2: i64;


    match arg0 {
        Value::Integer(e) => val1 = *e,
        Value::Float(_) => panic!("float is not valid for this function"),
        Value::Str(_) => panic!("string is not valid for this function"),
        Value::Boolean(_) => panic!("bool is not valid for this function"),
        _ => panic!("invalid value"),
    }

    match posy {
        Value::Integer(e) => val2 = *e,
        Value::Float(_) => panic!("float is not valid for this function"),
        Value::Str(_) => panic!("string is not valid for this function"),
        Value::Boolean(_) => panic!("bool is not valid for this function"),
        _ => panic!("invalid value"),
    }

   // println!("args of sum: {args:?}");
    Ok(Value::Integer(val1.wrapping_add(val2)))
}

pub fn math_is_positive(args: Vec<Value>) -> Result<Value, &'static str>{
    
    let arg0 = &args[0];

    let val1: i64;

    match arg0 {
        Value::Integer(e) => val1 = *e,
        Value::Float(_) => panic!("float is not valid for this function"),
        Value::Str(_) => panic!("string is not valid for this function"),
        Value::Boolean(_) => panic!("bool is not valid for this function"),
        _ => panic!("invalid value"),
    }

   // println!("args of sum: {args:?}");
    Ok(Value::Boolean(val1.is_positive()))
}

pub fn os_args(args: Vec<Value>) -> Result<Value, &'static str> {
    let arg0 = &args[0];

    let val1: i64;

    match arg0 {
        Value::Integer(e) => val1 = *e,
        Value::Float(_) => panic!("float is not valid for this function"),
        Value::Str(_) => panic!("string is not valid for this function"),
        Value::Boolean(_) => panic!("bool is not valid for this function"),
        _ => panic!("invalid value"),
    }

    let argument = env::args().nth(val1 as usize).unwrap_or_else(|| {
        panic!("Argument in position: {} cannot be found", val1)
    });
    
    Ok(Value::Str(argument))
}

pub fn os_run(args: Vec<Value>) -> Result<Value, &'static str> {


    let arg0 = &args[0];
    

    let mut val1: String;

    match arg0 {
        Value::Integer(_) => panic!("integer is not valid for this function"),
        Value::Float(_) => panic!("float is not valid for this function"),
        Value::Str(s) => val1 = s.to_owned(),
        Value::Boolean(_) => panic!("bool is not valid for this function"),
        _ => panic!("invalid value"),
    }

    if val1.starts_with('"') {
        val1 = format!("\"{}\"", val1);
    } else {
        val1 = format!("{}", val1);
    }
    
    let output = Command::new("bash")
        .arg("-c")
        .arg(&val1)
        .stdout(Stdio::piped())
        .output();

   // println!("cmd: {v}", v = output.is_ok());
    
    if output.is_err() {
        return Ok(Value::Null);
    }

    let res = String::from_utf8_lossy(&output.unwrap().stdout).to_string();

    if res.is_empty() {
        return Ok(Value::Null);
    }

    Ok(Value::Str(res.trim().to_owned()))
}

