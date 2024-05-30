use crate::{constants::CONFIG, random::Random};
use super::{list_structs::ListStructs, operator::Operator, struct_type::StructType};

#[derive(Clone, PartialEq)]
pub enum Trait {
    Eq,
    Ord,
}

impl Trait {
    pub fn iterator() -> impl Iterator<Item = Trait> {
        [Trait::Eq, Trait::Ord].iter().cloned()
    }
}

impl std::fmt::Display for Trait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trait::Eq => write!(f, "Eq"),
            Trait::Ord => write!(f, "Ord"),
        }
    }
}

#[derive(Clone)]
pub enum VarType {
    field,
    uint(usize),
    int(usize),
    bool,
    str(usize),
    array(Box<VarType>, usize),
    slice(Box<VarType>, usize),
    // Vec(Box<VarType>),
    tup(Vec<Box<VarType>>),
    strct(StructType),
    reference(Box<VarType>),
    generic(Vec<Trait>),
}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            
            VarType::field => write!(f, "Field"),
            VarType::bool => write!(f, "bool"),
            VarType::uint(size) => write!(f, "u{}", size),
            VarType::int(size) => write!(f, "i{}", size),
            VarType::str(size) => write!(f, "str<{}>", size),
            VarType::array(type_param, size) => write!(f, "[{}; {}]", type_param, size),
            VarType::slice(type_param, _) => write!(f, "[{}]", type_param),
            VarType::tup(vec_type_param) => {
                write!(f, "(")?;
                for (i, type_param) in vec_type_param.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", type_param)?;
                }
                write!(f, ")")
            },
            VarType::strct(strct) => write!(f, "{}", strct.name()),
            VarType::reference(type_param) => write!(f, "&mut {}", type_param),
            VarType::generic(_) => write!(f, "T"),
        }
    }
}

pub fn basic_types() -> Vec<&'static VarType> {
    vec![
        &VarType::field,
        &VarType::bool,
        &VarType::uint(1),
        &VarType::uint(8),
        &VarType::uint(32),
        &VarType::uint(64),
        &VarType::int(1),
        &VarType::int(8),
        &VarType::int(32),
        &VarType::int(64),
    ]

}

fn random_bit_size(random: &mut Random) -> usize {
    match random.gen_range(0, 4) {
        0 => 1,
        1 => 8,
        2 => 32,
        3 => 64,
        _ => panic!("Impossible case"),
    }
}

pub fn random_int_type(random: &mut Random) -> VarType {
    match random.gen_range(0, 3) {
        0 => VarType::field,
        1 => VarType::uint(random_bit_size(random)),
        2 => VarType::int(random_bit_size(random)),
        _ => VarType::field,
    }
}

pub fn random_basic_type(random: &mut Random) -> VarType {
    match random.gen_range(0, 5) {
        0 => VarType::field,
        1 => VarType::uint(random_bit_size(random)),
        2 => VarType::int(random_bit_size(random)),
        3 => VarType::bool,
        4 => VarType::str(random.gen_range(0, CONFIG.max_composite_size)),
        _ => VarType::field,
    }
}

pub fn random_type(random: &mut Random, list_structs: &ListStructs) -> VarType {
    match random.gen_range(0, 9 + CONFIG.use_of_slice) {
        0 => VarType::field,
        1 => VarType::uint(random_bit_size(random)),
        2 => VarType::int(random_bit_size(random)),
        3 => VarType::bool,
        4 => VarType::str(random.gen_range(0, CONFIG.max_composite_size)),
        5 => VarType::array(Box::new(random_type_with_depth(random, list_structs, CONFIG.max_composite_depth)), random.gen_range(0, CONFIG.max_composite_size)),
        6 => {
            let size = random.gen_range(2, CONFIG.max_composite_size);
            let mut vec_tup = Vec::with_capacity(size);
            for _ in 0..size {
                vec_tup.push(Box::new(random_type_with_depth(random, list_structs, CONFIG.max_composite_depth)));
            }
            VarType::tup(vec_tup)
        },
        7 => {
            if list_structs.is_empty(){
                random_type_with_depth(random, list_structs, CONFIG.max_composite_depth)
            } else {
                VarType::strct(list_structs.get_random(random))
            }
        },
        8 => VarType::reference(Box::new(random_basic_type(random))),
        9 => VarType::slice(Box::new(random_type_with_depth(random, list_structs, CONFIG.max_composite_depth)), random.gen_range(0, CONFIG.max_composite_size)),
        _ => VarType::field,
    }
}

pub fn random_type_with_depth(random: &mut Random, list_structs: &ListStructs, depth: usize) -> VarType {
    if depth == 0 {
        match random.gen_range(0, 5) {
            0 => VarType::field,
            1 => VarType::uint(random_bit_size(random)),
            2 => VarType::int(random_bit_size(random)),
            3 => VarType::bool,
            4 => VarType::str(random.gen_range(0, CONFIG.max_composite_size)),
            _ => VarType::field,
        }
    } else {
        match random.gen_range(0, 9) {
            0 => VarType::field,
            1 => VarType::uint(random_bit_size(random)),
            2 => VarType::int(random_bit_size(random)),
            3 => VarType::bool,
            4 => VarType::str(random.gen_range(0, CONFIG.max_composite_size)),
            5 => VarType::array(Box::new(random_type_with_depth(random, list_structs, depth -1)), random.gen_range(0, CONFIG.max_composite_size)),
            6 => {
                let size = random.gen_range(2, CONFIG.max_composite_size);
                let mut vec_tup = Vec::with_capacity(size);
                for _ in 0..size {
                    vec_tup.push(Box::new(random_type_with_depth(random, list_structs, depth -1)));
                }
                VarType::tup(vec_tup)
            },
            7 => {
                if list_structs.is_empty() || depth != CONFIG.max_composite_depth {
                    random_type_with_depth(random, list_structs, depth -1)
                } else {
                    VarType::strct(list_structs.get_random(random))
                }
            },
            8 => VarType::reference(Box::new(random_basic_type(random))),
            _ => VarType::field,
        }
    }
}

pub fn supported_arithmetic_operator_by_type(var_type: &VarType) -> Vec<Operator> {
    match var_type {
        VarType::field => vec![Operator::Add, Operator::Subtract, Operator::Multiply, Operator::Divide],
        VarType::int(size) => if *size == 127 { 
            vec![Operator::Add, Operator::Subtract, Operator::Divide]
        } else {
            vec![Operator::Add, Operator::Subtract, Operator::Multiply, Operator::Divide]
        },
        VarType::uint(_) => vec![Operator::Add, Operator::Subtract, Operator::Multiply, Operator::Divide, Operator::Xor, Operator::And, Operator::Or, Operator::Lshift, Operator::Rshift],
        VarType::bool => vec![Operator::Equal, Operator::NotEqual, Operator::Or, Operator::And],
        _ => vec![], // Handle unknown types
    }
}

pub fn supported_comparator_operator_by_type(var_type: &VarType) -> Vec<Operator> {
    match var_type {
        VarType::uint(_) | VarType::int(_) => vec![Operator::Equal, Operator::NotEqual, Operator::Lesser, Operator::Greater, Operator::LesserOrEqual, Operator::GreaterOrEqual],
        VarType::bool => vec![Operator::Equal, Operator::NotEqual, Operator::Or, Operator::And],
        VarType::field | VarType::str(_) | VarType::slice(_,_) | VarType::array(_,_) => vec![Operator::Equal, Operator::NotEqual],
        VarType::generic(vec_trait) => {
            let mut ret = Vec::new();
            if vec_trait.contains(&Trait::Eq) {
                ret.push(Operator::Equal);
                ret.push(Operator::NotEqual);
            }
            if vec_trait.contains(&Trait::Ord) {
                ret.push(Operator::Lesser);
                ret.push(Operator::Greater);
                ret.push(Operator::LesserOrEqual);
                ret.push(Operator::GreaterOrEqual);
            }
            ret
        }
        _ => vec![], // Handle unknown types
    }
}

pub fn is_same_type(first_type: &VarType, second_type: &VarType) -> bool {
    match (first_type, second_type) {
        (VarType::field, VarType::field) => true,
        (VarType::uint(s1), VarType::uint(s2)) => s1 == s2,
        (VarType::int(s1), VarType::int(s2)) => s1 == s2,
        (VarType::bool, VarType::bool) => true,
        (VarType::str(s1), VarType::str(s2)) => s1 == s2,
        (VarType::array(var_type1, s1), VarType::array(var_type2, s2)) 
            => s1 == s2 && is_same_type(var_type1, var_type2),
        (VarType::slice(var_type1, s1), VarType::slice(var_type2, s2)) 
            => s1 == s2 && is_same_type(var_type1, var_type2),
        (VarType::tup(vec_type1), VarType::tup(vec_type2)) 
            => if vec_type1.len() != vec_type2.len(){
                false
            } else {
                for i in 0..vec_type1.len() {
                    if !is_same_type(vec_type1.get(i).expect("None in tup"), vec_type2.get(i).expect("None in tup")){
                        return false;
                    }
                }
                true
            },
        (VarType::strct(strct1), VarType::strct(strct2)) => strct1.name() == strct2.name(),
        (VarType::reference(var_type1), VarType::reference(var_type2)) 
            => is_same_type(var_type1, var_type2),
        
        (VarType::generic(_), VarType::generic(_)) => true,
        _ => false,
    }
}

pub fn way_to_type(random: &mut Random, source_type: &VarType, aim_type: &VarType, is_a_ref: &mut bool) -> Option<String> {
    match source_type {
        VarType::field | VarType::bool | VarType::str(_) | VarType::uint(_) | VarType::int(_) | VarType::generic(_)
            => if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                *is_a_ref = false;
                None
            },
        VarType::array(type_param, size) 
            => if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                if let Some(str) = way_to_type(random, type_param, aim_type, is_a_ref) {
                    if *size != 0 {
                        return Some(format!("[{}]{}", random.gen_range(0, *size), str));
                    }
                }
                *is_a_ref = false;
                None
            },
        VarType::slice(type_param, size) 
            => if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                if let Some(str) = way_to_type(random, type_param, aim_type, is_a_ref) {
                    if *size != 0 {
                        return Some(format!("[{}]{}", random.gen_range(0, *size), str));
                    }
                }
                *is_a_ref = false;
                None
            },
        VarType::tup(vec_type_param) 
            => if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                for (ind,type_param) in vec_type_param.iter().enumerate() {
                    if let Some(str) = way_to_type(random, type_param, aim_type, is_a_ref) {
                        return Some(format!(".{}{}", ind, str));
                    }
                }
                *is_a_ref = false;
                None
            },
        VarType::strct(strct) 
            => if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                for (_,type_param) in strct.key_types().iter().enumerate() {
                    if let Some(str) = way_to_type(random, &type_param.0, aim_type, is_a_ref) {
                        return Some(format!(".{}{}", type_param.1, str));
                    }
                }
                *is_a_ref = false;
                None
            },
        VarType::reference(type_param) 
            => if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                *is_a_ref = true;
                way_to_type(random, type_param, aim_type, is_a_ref)          
            },
    } 
}