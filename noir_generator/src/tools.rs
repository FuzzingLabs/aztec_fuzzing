pub fn ignored_error_cmd(err: &str) -> bool {
    let errors = vec![
        "attempt to divide by zero",
        "Comparisons are invalid on Field types.",
        "The number of bits to use for this bitwise operation is ambiguous.",

        //temp
        "Types in a binary operation should match",
        "expected type",
        "Expected type",

        // Only for fuzz by fun call
        // "panicked at",
        // "attempt to shift left with overflow",
        // "attempt to shift right with overflow"
    ];

    for line in err.lines() {
        if line.contains("error:") || line.contains("Message:") {
            if !errors.iter().any(|&e| line.contains(e)) {
                return false;
            }
        }
    }

    true
}

pub fn ignored_error(err: &str) -> bool {
    let errors = vec![
        "attempt to divide by zero",
        "Comparisons are invalid on Field types.",
        "The number of bits to use for this bitwise operation is ambiguous.",

        //temp
        "Types in a binary operation should match",
        "expected type",
        "Expected type",

        // Only for fuzz by fun call
        // "panicked at",
        // "attempt to shift left with overflow",
        // "attempt to shift right with overflow"
    ];

    if !errors.iter().any(|&e| err.contains(e)) {
        return false;
    }

    true
}

pub fn clean_ansi_escape_codes(input: &str) -> String {
    let regex = regex::Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]").unwrap();
    regex.replace_all(input, "").into_owned()
}