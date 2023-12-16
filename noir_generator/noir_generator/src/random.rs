use rand::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref RNG: Mutex<RandomGenerator> = Mutex::new(RandomGenerator::new(0));
}

    pub fn initialize_rng(seed: Option<u64>) {
        let mut rng = RNG.lock().unwrap();
        match seed {
            Some(s) => *rng = RandomGenerator::new(s),
            None => *rng = RandomGenerator::new(rand::thread_rng().gen()),
        }
    }

pub fn generate_random_number(lower_limit: u32, upper_limit: u32) -> u32 {
    let mut rng = RNG.lock().unwrap();
    rng.generate_random_number(lower_limit, upper_limit)
}

pub fn generate_random_bool() -> bool {
    let mut rng = RNG.lock().unwrap();
    rng.generate_random_bool()
}

pub fn select_random_str_from_vec(strings: Vec<&'static str>) -> &'static str {
    let mut rng = RNG.lock().unwrap();
    rng.select_random_str_from_vec(strings)
}

pub fn choose_random_item_from_vec<T: Clone>(items: &Vec<T>) -> Option<T> {
    let mut rng = RNG.lock().unwrap();
    rng.choose_random_item_from_vec(items)
} 

pub fn generate_random_value_for_type(type_: &'static str) -> String {
    let mut rng = RNG.lock().unwrap();
    rng.generate_random_value_for_type(type_)
}

pub struct RandomGenerator {
    rng: StdRng,
}

impl RandomGenerator {
    pub fn new(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        RandomGenerator { rng }
    }

    pub fn generate_random_number(&mut self, lower_limit: u32, upper_limit: u32) -> u32 {
        if upper_limit <= lower_limit {
            panic!("Upper limit must be greater than lower limit");
        }
    
        self.rng.gen_range(lower_limit..upper_limit)
    }

    pub fn generate_random_bool(&mut self) -> bool {
        self.rng.gen()
    }


    // TODO: DELETE
    pub fn select_random_str_from_vec(&mut self, strings: Vec<&'static str>) -> &'static str {
        if strings.is_empty() {
            return "";
        }

        let index = self.rng.gen_range(0..strings.len());
        strings[index].clone()

        // if type_ == "str" {
        //     type_ = format!("str<{}>", value.len()-2); // -2 because of the ""
        // }
    }

    pub fn choose_random_item_from_vec<T: Clone>(&mut self, items: &Vec<T>) -> Option<T> {
        if items.is_empty() {
            None
        } else {
            let index = self.rng.gen_range(0..items.len());
            Some(items[index].clone())
        }
    }

    pub fn generate_random_value_for_type(&mut self, type_: &'static str) -> String {
        match type_{
            "Field" => {
                let upper_part: u128 = self.rng.gen();
                let lower_part: u128 = self.rng.gen();
                format!("{:032x}{:032x}", upper_part, lower_part)
            },
            "u8" => self.rng.gen_range(0..=u8::MAX).to_string(),
            "u16" => self.rng.gen_range(0..=u16::MAX).to_string(),
            "u32" => self.rng.gen_range(0..=u32::MAX).to_string(),
            "u64" => self.rng.gen_range(0..=u64::MAX).to_string(),
            "u127" => {
                let random_u128: u128 = self.rng.gen();
                let random_u127 = random_u128 >> 1; // Truncate to 127 bits
                random_u127.to_string()
            },
            "i8" => self.rng.gen_range(i8::MIN..=i8::MAX).to_string(),
            "i16" => self.rng.gen_range(i16::MIN..=i16::MAX).to_string(),
            "i32" => self.rng.gen_range(i32::MIN..=i32::MAX).to_string(),
            "i64" => self.rng.gen_range(i64::MIN..=i64::MAX).to_string(),
            "i127" => {
                let random_i128: i128 = self.rng.gen();
                let random_i127 = random_i128 >> 1; // Truncate to 127 bits
                random_i127.to_string()
            },
            "bool" => self.rng.gen::<bool>().to_string(),
            "str" => {
                let length = self.rng.gen_range(0..100); // Vous pouvez ajuster la plage de longueurs selon vos besoins
                let random_string: String = (0..length)
                    .map(|_| (self.rng.gen_range(b'a'..=b'z') as char))
                    .collect();
                random_string
            },
            _ => panic!("Unknown type {}", type_),
        }
    }
}