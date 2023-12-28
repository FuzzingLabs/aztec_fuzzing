use lazy_static::lazy_static;
use std::sync::Mutex;
use rand::Rng;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

lazy_static! {
    static ref RNG: Mutex<RandomGenerator> = Mutex::new(RandomGenerator::new(0));
}

pub fn initialize_rng(seed: Option<u8>) {
    let mut rng = RNG.lock().unwrap();
    match seed {
        Some(s) => *rng = RandomGenerator::new(u64::from(s)),
        None => *rng = RandomGenerator::new(rand::thread_rng().gen()),
    }
}

pub fn generate_random_number(lower_limit: u32, upper_limit: u32) -> u32 {
    let mut rng = RNG.lock().unwrap();
    if upper_limit <= lower_limit {
        panic!("Upper limit must be greater than lower limit");
    }

    rng.rng.gen_range(lower_limit..upper_limit)
}

pub fn generate_random_bool() -> bool {
    let mut rng = RNG.lock().unwrap();
    rng.rng.gen()
}

pub fn select_random_str_from_vec(strings: Vec<&'static str>) -> &'static str {
    let mut rng = RNG.lock().unwrap();
    if strings.is_empty() {
        return "";
    }

    let index = rng.rng.gen_range(0..strings.len());
    strings[index]
}

pub fn choose_random_item_from_vec<T: Clone>(items: &Vec<T>) -> Option<T> {
    let mut rng = RNG.lock().unwrap();
    if items.is_empty() {
        None
    } else {
        let index = rng.rng.gen_range(0..items.len());
        Some(items[index].clone())
    }
} 

pub fn generate_random_value_for_type(type_: &'static str) -> String {
    let mut rng = RNG.lock().unwrap();
    match type_{
        "Field" => rng.rng.gen_range(0..=u128::MAX).to_string(),
        "u8" => rng.rng.gen_range(0..=u8::MAX).to_string(),
        "u16" => rng.rng.gen_range(0..=u16::MAX).to_string(),
        "u32" => rng.rng.gen_range(0..=u32::MAX).to_string(),
        "u64" => rng.rng.gen_range(0..=u64::MAX).to_string(),
        "u127" => {
            let random_u128: u128 = rng.rng.gen();
            let random_u127 = random_u128 >> 1; // Truncate to 127 bits
            random_u127.to_string()
        },
        "i8" => rng.rng.gen_range(i8::MIN..=i8::MAX).to_string(),
        "i16" => rng.rng.gen_range(i16::MIN..=i16::MAX).to_string(),
        "i32" => rng.rng.gen_range(i32::MIN..=i32::MAX).to_string(),
        "i64" => rng.rng.gen_range(i64::MIN..=i64::MAX).to_string(),
        "i127" => {
            let random_i128: i128 = rng.rng.gen();
            let random_i127 = random_i128 >> 1; // Truncate to 127 bits
            random_i127.to_string()
        },
        "bool" => rng.rng.gen::<bool>().to_string(),
        "str" => {
            let length = rng.rng.gen_range(1..100); // Vous pouvez ajuster la plage de longueurs selon vos besoins
            let random_string: String = (0..length)
                .map(|_| (rng.rng.gen_range(b'a'..=b'z') as char))
                .collect();
            let result = format!("\"{}\"", random_string);
            result
        },
        _ => panic!("Unknown type {}", type_),
    }
}

pub struct RandomGenerator {
    rng: XorShiftRng,
}

impl RandomGenerator {
    pub fn new(seed: u64) -> Self {
        let rng = XorShiftRng::seed_from_u64(seed);
        RandomGenerator { rng }
    }
}