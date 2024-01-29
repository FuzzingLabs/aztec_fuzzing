use rand::{SeedableRng, Rng, seq::SliceRandom};
use rand_xorshift::XorShiftRng;

static RNG: std::sync::Once = std::sync::Once::new();

lazy_static::lazy_static! {
    static ref RNG_INSTANCE: std::sync::Mutex<Option<XorShiftRng>> = std::sync::Mutex::new(None);
}

pub fn initialize_rng(data: Option<&[u8]>) {
    let seed_value = match data {
        Some(s) 
            =>  ((s[0] as u64) << 56) |
                ((s[1] as u64) << 48) |
                ((s[2] as u64) << 40) |
                ((s[3] as u64) << 32) |
                ((s[4] as u64) << 24) |
                ((s[5] as u64) << 16) |
                ((s[6] as u64) << 8) |
                (s[7] as u64),
        None => rand::thread_rng().gen(),
    };

    RNG.call_once(|| {
        let rng = XorShiftRng::seed_from_u64(seed_value);
        let _ = RNG_INSTANCE.lock().map(|mut inner| *inner = Some(rng));
    });
}

pub fn gen_range<T>(min: T, max: T) -> T
where
    T: PartialOrd + rand::distributions::uniform::SampleUniform,
{
    if min >= max{
        return min;
    }
    
    let mut binding = RNG_INSTANCE.lock().expect("RNG is not initialized");
    let rng = binding.as_mut().unwrap();
    rng.gen_range(min..max)
}

pub fn choose_random_item_from_vec<T: Clone>(items: &Vec<T>) -> T {
    let mut binding = RNG_INSTANCE.lock().expect("RNG is not initialized");
    let rng = binding.as_mut().unwrap();

    if items.is_empty() {
        panic!("Cannot choose from an empty vector");
    }

    // Utiliser la méthode choose de rand pour choisir un élément aléatoire
    items.choose(rng).expect("Failed to choose a random item").clone()
}

pub fn gen_name() -> String {
    let mut binding = RNG_INSTANCE.lock().expect("RNG is not initialized");
    let rng = binding.as_mut().unwrap();
    let name_length = rng.gen_range(5..15);
    let mut name = String::with_capacity(name_length);

    for _ in 0..name_length {
        let random_char = rng.gen_range(b'a'..=b'z') as char;
        name.push(random_char);
    }

    name
}

pub fn gen_field() -> u128 {
    let mut binding = RNG_INSTANCE.lock().expect("RNG is not initialized");
    let rng = binding.as_mut().unwrap();
    rng.gen()
}

pub fn gen_random_uint(bit_size: usize) -> u128{
    assert!(bit_size > 0 && bit_size <= 128,
        "Invalid bit size for u128");

    let mut binding = RNG_INSTANCE.lock().expect("RNG is not initialized");
    let rng = binding.as_mut().unwrap();

    let random_u128: u128 = rng.gen();
    random_u128 & ((1u128 << bit_size) - 1) as u128
}

pub fn gen_random_int(bit_size: usize) -> i128{
    assert!(bit_size > 0 && bit_size <= 128,
        "Invalid bit size for i128");

    let mut binding = RNG_INSTANCE.lock().expect("RNG is not initialized");
    let rng = binding.as_mut().unwrap();

    let random_i128: i128 = rng.gen();
    random_i128 & ((1i128 << (bit_size - 1)) - 1) as i128
}

pub fn gen_bool() -> bool {
    let mut binding = RNG_INSTANCE.lock().expect("RNG is not initialized");
    let rng = binding.as_mut().unwrap();
    rng.gen()
}

pub fn gen_str(size: usize) -> String {
    let mut binding = RNG_INSTANCE.lock().expect("RNG is not initialized");
    let rng = binding.as_mut().unwrap();
    let mut name = String::with_capacity(size);

    for _ in 0..size {
        let random_char = rng.gen_range(b'a'..=b'z') as char;
        name.push(random_char);
    }

    name
}
