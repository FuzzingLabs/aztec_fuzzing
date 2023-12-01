use rand::distributions::{Alphanumeric, Standard};
use rand::{thread_rng, Rng};
use rand::RngCore;
use num_bigint::BigUint;


static POSSIBLE_VARIABLE_TYPES: [&str; 13] = ["Field", "u8", "u16", "u32", "u64", "u127", "i8", "i16", "i32", "i64", "i127", "bool", "str"];
static POSSIBLE_VARIABLE_COMPOUND_TYPES: [&str; 5] = ["Array", "Slice", "Vector", "Tuple", "Struct"];

fn generate_random_field() -> String {
    let mut rng = rand::thread_rng();

    // Crée un nombre aléatoire de 256 bits
    let mut random_bytes = vec![0u8; 32];
    rng.fill_bytes(&mut random_bytes);

    // Convertit les octets en un nombre BigUint
    let random_u256 = BigUint::from_bytes_be(&random_bytes);

    // Convertit le nombre BigUint en chaîne de caractères
    random_u256.to_string()
}

fn generate_random_u8() -> String {
    rand::thread_rng().gen::<u8>().to_string()
}

fn generate_random_u16() -> String {
    rand::thread_rng().gen::<u16>().to_string()
}

fn generate_random_u32() -> String {
    rand::thread_rng().gen::<u32>().to_string()
}

fn generate_random_u64() -> String {
    rand::thread_rng().gen::<u64>().to_string()
}

// generate an int in the range : 0..170141183460469231731687303715884105727
fn generate_random_u127() -> String {
    let mut result = rand::thread_rng().gen::<u128>();
    while result > 170141183460469231731687303715884105727 {
        result = rand::thread_rng().gen::<u128>();
    }
    result.to_string()
}

fn generate_random_i8() -> String {
    rand::thread_rng().gen::<i8>().to_string()
}

fn generate_random_i16() -> String {
    rand::thread_rng().gen::<i16>().to_string()
}

fn generate_random_i32() -> String {
    rand::thread_rng().gen::<i32>().to_string()
}

fn generate_random_i64() -> String {
    rand::thread_rng().gen::<i64>().to_string()
}

fn generate_random_i127() -> String {
    let mut result = rand::thread_rng().gen::<i128>();
    while result > 85070591730234615865843651857942052863 || result < -85070591730234615865843651857942052864 {
        result = rand::thread_rng().gen::<i128>();
    }
    result.to_string()
}

fn generate_random_bool() -> String {
    rand::thread_rng().gen::<bool>().to_string()
}

fn generate_random_str() -> String {
    let mut rng = thread_rng();
    if rand::thread_rng().gen::<bool>() {
        let length: usize = rng.gen_range(1..=100);
        let random_string: String = rng
            .sample_iter(Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        format!("\"{}\"", random_string)
    } else {
        let length: usize = rng.gen_range(1..=100);
        let random_string: String = rng
            .sample_iter::<char, _>(Standard)
            .take(length)
            .collect();

        format!("\"{}\"", random_string)
    }
}

pub fn generate_random_value_from_type(type_: String) -> String {
    match type_.as_str() {
        "Field" => generate_random_field(),
        "u8" => generate_random_u8(),
        "u16" => generate_random_u16(),
        "u32" => generate_random_u32(),
        "u64" => generate_random_u64(),
        "u127" => generate_random_u127(),
        "i8" => generate_random_i8(),
        "i16" => generate_random_i16(),
        "i32" => generate_random_i32(),
        "i64" => generate_random_i64(),
        "i127" => generate_random_i127(),
        "bool" => generate_random_bool(),
        "str" => generate_random_str(),
        _ => panic!("Unknown type {}", type_),
    }
}

pub fn generate_variable_type() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..POSSIBLE_VARIABLE_TYPES.len());
    POSSIBLE_VARIABLE_TYPES[index].to_string()
}

pub fn generate_variable_type_except_types(except_types: Vec<String>) -> String {
    let mut type_: String = generate_variable_type();
    while except_types.contains(&type_) {
        type_ = generate_variable_type();
    }
    return type_;
}

pub fn verify_type_existence(type_: String) -> bool {
    for possible_type in POSSIBLE_VARIABLE_TYPES.iter() {
        if type_ == *possible_type {
            return true;
        }
    }
    return false;
}

pub fn generate_random_str_with_length(length: usize) -> String {
    let mut rng = thread_rng();
    if rand::thread_rng().gen::<bool>() {
        let random_string: String = rng
            .sample_iter(Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        format!("\"{}\"", random_string)
    } else {
        let random_string: String = rng
            .sample_iter::<char, _>(Standard)
            .take(length)
            .collect();

        format!("\"{}\"", random_string)
    }
}
