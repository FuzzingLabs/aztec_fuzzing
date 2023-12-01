use rand::Rng;

pub fn generate_affectation_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..1); // Not (0..11) because not yet implemented on Noir, only == works
    match index {
        0 => "=".to_string(),
        1 => "+=".to_string(),
        2 => "-=".to_string(),
        3 => "*=".to_string(),
        4 => "/=".to_string(),
        5 => "%=".to_string(),
        6 => "&=".to_string(),
        7 => "|=".to_string(),
        8 => "^=".to_string(),
        9 => "<<=".to_string(),
        10 => ">>=".to_string(),
        _ => panic!("Unknown index {}", index),
    }
}

pub fn generate_arithmetic_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..5);
    match index {
        0 => "+".to_string(),
        1 => "-".to_string(),
        2 => "*".to_string(),
        3 => "/".to_string(),
        4 => "%".to_string(),
        _ => panic!("Unknown index {}", index),
    }
}

pub fn generate_bitwise_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..5);
    match index {
        0 => "&".to_string(),
        1 => "|".to_string(),
        2 => "^".to_string(),
        3 => "<<".to_string(),
        4 => ">>".to_string(),
        _ => panic!("Unknown index {}", index),
    }
}

pub fn generate_comparison_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..6);
    match index {
        0 => "==".to_string(),
        1 => "!=".to_string(),
        2 => ">=".to_string(),
        3 => "<=".to_string(),
        4 => ">".to_string(),
        5 => "<".to_string(),
        _ => panic!("Unknown index {}", index),
    }
}

pub fn generate_basic_comparison_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..3);
    match index {
        0 => "==".to_string(),
        1 => "!=".to_string(),
        _ => panic!("Unknown index {}", index),
    }
}

pub fn generate_logical_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..3);
    match index {
        0 => "&".to_string(),
        1 => "|".to_string(),
        _ => panic!("Unknown index {}", index),
    }
}

pub fn generate_unary_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(5..8);
    match index {
        // 0 | 1 | 2 | 3 | 4 | 5 => "".to_string(),
        // 0..=5 => "".to_string(),
        5 => "".to_string(),
        6 => "!".to_string(),
        7 => "-".to_string(),
        // 8 => "*".to_string(), // TODO: In development, don't forget to change the range when you uncomment this line
        // 9 => "&".to_string(), // TODO: In development, don't forget to change the range when you uncomment this line
        _ => panic!("Unknown index {}", index),
    }
}
