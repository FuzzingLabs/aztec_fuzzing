// When more than half of the bits are used, this will result in a crash.
fn main() {
    let mut var: u8 = 16;

    // (var * var) overflows u8, but both left and right sides are equal to 0 and still result in a crash
    var = ((var * var) ^ (var * var)) - ((var * var) - (var * var));

    // No crash if the middle operand is an addition.
    var = ((var * var) ^ (var * var)) + ((var * var) - (var * var));

    // No crash if one part does not cause overflow.
    var = ((var * var) ^ (var * var)) - (var - var);
    var = (var ^ var) - ((var * var) - (var * var));
}
