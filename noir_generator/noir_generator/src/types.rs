pub fn supported_operations_for_type(var_type: &str) -> Vec<&'static str> {
    match var_type {
        "Field" | "u8" | "u16" | "u32" | "u64" | "u127" | "i8" | "i16" | "i32" | "i64" | "i127" 
            => vec!["+","-","*","/","^","&","|","<<",">>"],
        "bool" => vec!["==","!=","|","&"],
        "str" => vec!["+"],
        _ => vec![], // Handle unknown types
    }
}

pub fn supported_operations_for_assertion(var_type: &str) -> Vec<&'static str> {
    match var_type {
        "Field" | "u8" | "u16" | "u32" | "u64" | "u127" | "i8" | "i16" | "i32" | "i64" | "i127" 
            => vec!["==", "!=", "<", ">", "<=", ">="],
        "bool" => vec!["==", "!=", "|", "&"],
        "str" => vec!["==", "!="],
        _ => vec![], // Handle unknown types
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