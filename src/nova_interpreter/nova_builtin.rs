use crate::var_table::{self, vtable::Value};

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
    
    let posx = &args[0];

    let posy = &args[1];

    let val1: i64;
    let val2: i64;


    match posx {
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
    
    let posx = &args[0];

    let val1: i64;

    match posx {
        Value::Integer(e) => val1 = *e,
        Value::Float(_) => panic!("float is not valid for this function"),
        Value::Str(s) => panic!("string is not valid for this function"),
        Value::Boolean(_) => panic!("bool is not valid for this function"),
        _ => panic!("invalid value"),
    }

   // println!("args of sum: {args:?}");
    Ok(Value::Boolean(val1.is_positive()))
}

