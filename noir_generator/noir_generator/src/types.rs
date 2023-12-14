pub fn types() -> Vec<String> {
    vec![
        "Field".to_string(),
        "u8".to_string(),
        "u16".to_string(),
        "u32".to_string(),
        "u64".to_string(),
        "u127".to_string(),
        "i8".to_string(),
        "i16".to_string(),
        "i32".to_string(),
        "i64".to_string(),
        "i127".to_string(),
        "bool".to_string(),
        "str".to_string(),
    ]
}

pub fn compound_types() -> Vec<String> {
    vec![
        "Array".to_string(),
        "Slice".to_string(),
        "Vector".to_string(),
        "Tuple".to_string(),
        "Struct".to_string(),
    ]
}

pub fn operators() -> Vec<String> {
    vec![
        "+".to_string(),
        "-".to_string(),
        "*".to_string(),
        "/".to_string(),
        "!".to_string(),
    ]
}

pub fn without(types_without: Vec<String>) -> Vec<String> {
    types()
        .iter()
        .filter(|s| !types_without.contains(s))
        .cloned()
        .collect()
}
