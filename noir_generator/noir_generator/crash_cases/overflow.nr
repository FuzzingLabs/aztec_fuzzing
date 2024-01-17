fn main() {
    // max u127 = 170141183460469231731687303715884105727

    // Right shifting by more than max u127 will result in a crash (the left part doesn't matter)
    let var: u127 = 10 >> (170141183460469231731687303715884105727 + 1);

    // No crash by using the reverse operation if the shift exceeds the maximum u127 value
    let var: u127 = (170141183460469231731687303715884105727 + 1) << 10;
}
