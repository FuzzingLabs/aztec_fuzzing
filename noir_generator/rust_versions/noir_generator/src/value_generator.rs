use rand::Rng;
use rand::RngCore;
use num_bigint::BigUint;
use std::str::FromStr;

pub fn generate_random_field() -> String {
    let mut rng = rand::thread_rng();

    // Crée un nombre aléatoire de 256 bits
    let mut random_bytes = vec![0u8; 32];
    rng.fill_bytes(&mut random_bytes);

    // Convertit les octets en un nombre BigUint
    let random_u256 = BigUint::from_bytes_be(&random_bytes);

    // Convertit le nombre BigUint en chaîne de caractères
    random_u256.to_string()
}

pub fn generate_random_u8() -> String {
    rand::thread_rng().gen::<u8>().to_string()
}

pub fn generate_random_u16() -> String {
    rand::thread_rng().gen::<u16>().to_string()
}

pub fn generate_random_u32() -> String {
    rand::thread_rng().gen::<u32>().to_string()
}

pub fn generate_random_u64() -> String {
    rand::thread_rng().gen::<u64>().to_string()
}

// generate an int in the range : 0..170141183460469231731687303715884105727
pub fn generate_random_u127() -> String {
    let mut result = rand::thread_rng().gen::<u128>();
    while result > 170141183460469231731687303715884105727 {
        result = rand::thread_rng().gen::<u128>();
    }
    result.to_string()
}

pub fn generate_random_i8() -> String {
    rand::thread_rng().gen::<i8>().to_string()
}

pub fn generate_random_i16() -> String {
    rand::thread_rng().gen::<i16>().to_string()
}

pub fn generate_random_i32() -> String {
    rand::thread_rng().gen::<i32>().to_string()
}

pub fn generate_random_i64() -> String {
    rand::thread_rng().gen::<i64>().to_string()
}

pub fn generate_random_i127() -> String {
    let mut result = rand::thread_rng().gen::<i128>();
    while result > 85070591730234615865843651857942052863 || result < -85070591730234615865843651857942052864 {
        result = rand::thread_rng().gen::<i128>();
    }
    result.to_string()
}

pub fn generate_random_bool() -> String {
    rand::thread_rng().gen::<bool>().to_string()
}

pub fn generate_random_str() -> String {
    // Implementation for "str" goes here
    "\"RandomString\"".to_string()
}
