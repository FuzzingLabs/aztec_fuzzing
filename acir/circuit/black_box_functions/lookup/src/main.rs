#[macro_use]
extern crate honggfuzz;
use acir::circuit::black_box_functions::BlackBoxFunc;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(string) = std::str::from_utf8(data) {
                if let Some(res) = BlackBoxFunc::lookup(string) {
                    // panic!("good string: {} and res: {}", string, res);
                }
            }
        });
    }
}
