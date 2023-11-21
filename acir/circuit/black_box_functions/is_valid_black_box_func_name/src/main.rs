#[macro_use]
extern crate honggfuzz;
use acir::circuit::black_box_functions::BlackBoxFunc;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(string) = std::str::from_utf8(data) {
                if BlackBoxFunc::is_valid_black_box_func_name(string) {
                    // panic!("good string: {}", string);
                }
            }
        });
    }
}
