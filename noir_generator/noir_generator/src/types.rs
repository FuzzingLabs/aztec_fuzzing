pub fn supported_operations_for_type(var_type: &str) -> Vec<&'static str> {
    match var_type {
        "Field"
        | "u8"
        | "u16"
        | "u32"
        | "u64"
        | "u127"
        | "i8"
        | "i16"
        | "i32"
        | "i64"
        | "i127" => vec!["+","-","*","/"],
        "bool" => vec!["!"],
        "str" => vec!["+"],
        _ => vec![], // Handle unknown types
    }
}

pub fn compatible_types_for_operation(operation: &str) -> Vec<&'static str> {
    match operation {
        "+" | "-" | "*" | "/" => vec!["Field", "u8", "u16", "u32", "u64", "u127", "i8", "i16", "i32", "i64", "i127"],
        "!" => vec!["bool"],
        _ => vec![], // Handle unknown operations
    }
}

pub fn types() -> Vec<&'static str> {
    vec![
        "Field",
        "u8",
        "u16",
        "u32",
        "u64",
        "u127",
        "i8",
        "i16",
        "i32",
        "i64",
        "i127",
        "bool",
        "str",
    ]
}

pub fn operations() -> Vec<&'static str> {
    vec!["+","-","*","/","!"]
}