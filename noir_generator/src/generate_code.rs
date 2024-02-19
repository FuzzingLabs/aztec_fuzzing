use crate::{constants, functions::list_functions::ListFunctions, random::{self, Random}, variables::list_structs::ListStructs};

pub fn generate_code(data: &[u8]) -> String {
    let mut random = Random::new(data);
    let mut code_generated: String = String::new();

    let mut list_functions = ListFunctions::new();
    let mut list_structs = ListStructs::new();

    // TODO: NEED AT LEAST 1 BUT IT MIGHT NOT BE THE CASE
    for _ in 0..random.gen_range(0, constants::NB_MAX_STRUCT) {
        list_structs.add_struct(&mut random);
    }
    code_generated = format!("{}{}", code_generated, list_structs.generate_structs_code());

    for _ in 0..random.gen_range(0, constants::NB_MAX_FUNCTION) {
        list_functions.add_function(&mut random, &list_structs, false);
    }
    list_functions.add_function(&mut random, &list_structs, true);
    format!("{}{}", code_generated, list_functions.generate_functions_code(&mut random, &list_structs))
}