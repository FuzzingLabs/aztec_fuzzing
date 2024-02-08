use crate::{constants, functions::list_functions::ListFunctions, random};

pub fn generate_code() -> String {

    let mut list_functions = ListFunctions::new();

    for _ in 0..random::gen_range(0, constants::NB_MAX_FUNCTION) {
        list_functions.add_function(false);
    }
    list_functions.add_function(true);
    list_functions.generate_all_functions_core()
}