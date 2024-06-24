// This program loads values from a config.toml file
// These values set limits for the generator on various parameters
lazy_static::lazy_static! {
    pub static ref CONFIG: LimitsConfig = load_config();
}

pub fn load_config() -> LimitsConfig {
    let mut file = std::fs::File::open("config.toml").expect("Failed to open config file");
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content).expect("Failed to read config file");
    let config: toml::Value = toml::from_str(&content).expect("Failed to parse config file");

    LimitsConfig {
        max_function: config["limits"]["MAX_FUNCTION"].as_integer().unwrap() as usize,
        max_method_by_struct: config["limits"]["MAX_METHOD_BY_STRUCT"].as_integer().unwrap() as usize,
        max_global_variables: config["limits"]["MAX_GLOBAL_VARIABLES"].as_integer().unwrap() as usize,
        max_operation_depth: config["limits"]["MAX_OPERATION_DEPTH"].as_integer().unwrap() as usize,
        max_composite_depth: config["limits"]["MAX_COMPOSITE_DEPTH"].as_integer().unwrap() as usize,
        max_composite_size: config["limits"]["MAX_COMPOSITE_SIZE"].as_integer().unwrap() as usize,
        max_instruction_depth: config["limits"]["MAX_INSTRUCTION_DEPTH"].as_integer().unwrap() as usize,
        max_instruction_by_function: config["limits"]["MAX_INSTRUCTION_BY_FUNCTION"].as_integer().unwrap() as usize,
        max_instruction_by_lambda: config["limits"]["MAX_INSTRUCTION_BY_LAMBDA"].as_integer().unwrap() as usize,
        max_instruction_by_method: config["limits"]["MAX_INSTRUCTION_BY_METHOD"].as_integer().unwrap() as usize,
        max_function_arguments: config["limits"]["MAX_FUNCTION_ARGUMENTS"].as_integer().unwrap() as usize,
        max_lambda_arguments: config["limits"]["MAX_LAMBDA_ARGUMENTS"].as_integer().unwrap() as usize,
        max_method_arguments: config["limits"]["MAX_METHOD_ARGUMENTS"].as_integer().unwrap() as usize,
        max_struct: config["limits"]["MAX_STRUCT"].as_integer().unwrap() as usize,
        max_loop_in_for: config["limits"]["MAX_LOOP_IN_FOR"].as_integer().unwrap() as usize,
        min_data_length: config["limits"]["MIN_DATA_LENGTH"].as_integer().unwrap() as usize,
        max_data_length: config["limits"]["MAX_DATA_LENGTH"].as_integer().unwrap() as usize,
        use_of_slice: config["limits"]["USE_OF_SLICE"].as_integer().unwrap() as usize,
    }
}

pub struct LimitsConfig {
    pub max_function: usize,
    pub max_method_by_struct: usize,
    pub max_global_variables: usize,
    pub max_operation_depth: usize,
    pub max_composite_depth: usize,
    pub max_composite_size: usize,
    pub max_instruction_depth: usize,
    pub max_instruction_by_function: usize,
    pub max_instruction_by_lambda: usize,
    pub max_instruction_by_method: usize,
    pub max_function_arguments: usize,
    pub max_lambda_arguments: usize,
    pub max_method_arguments: usize,
    pub max_struct: usize,
    pub max_loop_in_for: usize,
    pub min_data_length: usize,
    pub max_data_length: usize,
    pub use_of_slice: usize,
}
