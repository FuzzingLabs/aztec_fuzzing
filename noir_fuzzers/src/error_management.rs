/// All errors with a message containing one of these strings will be ignored
/// Present to avoid triggering on frequently appearing errors
pub fn list_ignored_errors() -> Vec<&'static str> {
    let errors = vec![
        "attempt to divide by zero",
        "Comparisons are invalid on Field types.",
        "The number of bits to use for this bitwise operation is ambiguous.",
        "The bit count in a bit-shift",
        "No matching impl",
        "Types in a binary operation should match",
        "Expected an expression but found",
        "Expected a binary operator but found",
        "Expected type",
        "expected type",
    ];
    errors
}

pub fn ignored_error_cmd(err: &str) -> bool {
    for line in err.lines() {
        if line.contains("error:") || line.contains("Message:") {
            if !list_ignored_errors().iter().any(|&e| line.contains(e)) {
                return false;
            }
        }
    }
    true
}

pub fn ignored_error(err: &str) -> bool {
    if !list_ignored_errors().iter().any(|&e| err.contains(e)) {
        return false;
    }
    true
}

/// This function removes ANSI escape codes from an error message.
pub fn clean_ansi_escape_codes(input: &str) -> String {
    let regex = regex::Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]").unwrap();
    regex.replace_all(input, "").into_owned()
}