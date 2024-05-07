pub fn ignored_error(err: &str) -> bool {
    let errors = vec![
        "attempt to divide by zero",
        "Comparisons are invalid on Field types.",
        "The number of bits to use for this bitwise operation is ambiguous.",
        //temp
        "Types in a binary operation should match",
        "expected type",
        "Expected type",
        "Unexpected match", // Only for fuzz by fun call
                            // "panicked at",
                            // "attempt to shift left with overflow",
                            // "attempt to shift right with overflow"
    ];

    if !errors.iter().any(|&e| err.contains(e)) {
        return false;
    }

    true
}
