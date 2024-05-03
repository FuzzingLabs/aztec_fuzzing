use crate::{constants::CONFIG, functions::list_functions::ListFunctions, random::Random, variables::{bloc_data::BlocData, list_structs::ListStructs, value::random_value, var_type::random_type, variable::Variable}};

pub fn generate_code(data: &[u8]) -> String {
    let mut random = Random::new(data);
    let mut code_generated: String = String::new();

    let mut list_global = BlocData::new();
    let mut list_functions = ListFunctions::new();
    let mut list_structs = ListStructs::new();

    for i in 0..random.gen_range(0, CONFIG.max_global_variables) {
        let var_type = random_type(&mut random, &list_structs);
        let var = Variable::new(format!("global{}", i+1), false, &var_type);
        code_generated = format!("{}{} = {};\n", code_generated, var.initialize_as_global(), random_value(&mut random, &var_type));
        list_global.add_variable(var);
    }

    for _ in 0..random.gen_range(0, CONFIG.max_struct) {
        code_generated = format!("{}{}", code_generated, list_structs.add_struct(&mut random, &list_global, &list_functions));
    }

    for _ in 0..random.gen_range(0, CONFIG.max_function) {
        code_generated = format!("{}{}", code_generated, list_functions.add_function(&mut random, &list_global, &list_structs, false));
    }
    format!("{}{}", code_generated, list_functions.add_function(&mut random, &list_global, &list_structs, true))
}