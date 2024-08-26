use functions::list_functions::ListFunctions;
use rand::{rngs::StdRng, Rng, SeedableRng};
use tools::{constants::CONFIG, random::Random};
use variables::{
    bloc_data::BlocData, list_structs::ListStructs, value::random_value,
    var_type::random_type_without_reference, variable::Variable,
};

pub mod functions;
pub mod imported_libraries;
pub mod instructions;
pub mod statements;
pub mod tools;
pub mod variables;

/// Returns a string containing all the generated code.
pub fn generate_code(data: &[u8]) -> String {
    // Creates a source of randomness based on the data generated by honggfuzz
    let mut random = Random::new(data);

    let mut code_generated: String = String::new();

    // Generate each part of the code (global variables, structs, and functions including main)
    let mut list_global = BlocData::new();
    let mut list_structs = ListStructs::new();
    let mut list_functions = ListFunctions::new();

    if CONFIG.use_of_std_lib == 1 {
        code_generated = format!(
            "{}{}\n",
            code_generated,
            imported_libraries::std_lib::import_statement()
        );
        imported_libraries::std_lib::add_structures_and_functions(
            &mut list_structs,
            &mut list_functions,
        );
    }

    for i in 0..random.gen_range(0, CONFIG.max_global_variables) {
        let var_type = random_type_without_reference(&mut random, &list_structs);
        let var = Variable::new(format!("global{}", i + 1), false, &var_type);
        code_generated = format!(
            "{}{} = {};\n",
            code_generated,
            var.initialize_as_global(),
            random_value(&mut random, &var_type)
        );
        list_global.add_variable(var);
    }

    for _ in 0..random.gen_range(0, CONFIG.max_struct) {
        code_generated = format!(
            "{}{}",
            code_generated,
            list_structs.add_random_struct(&mut random, &list_global, &list_functions)
        );
    }

    for _ in 0..random.gen_range(0, CONFIG.max_function) {
        code_generated = format!(
            "{}{}",
            code_generated,
            list_functions.add_random_function(&mut random, &list_global, &list_structs, false)
        );
    }
    format!(
        "{}{}",
        code_generated,
        list_functions.add_random_function(&mut random, &list_global, &list_structs, true)
    )
}

/// Generates a code string based on the given seed, ensuring reproducible results for the same seed.
pub fn generate_code_with_seed(seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);
    let size = rng.gen_range(10000..=1000000);
    let vec: Vec<u8> = (0..size).map(|_| rng.gen::<u8>()).collect();
    generate_code(&vec)
}
