use std::vec;

use crate::{random, constants::{MAX_COMPOSITE_SIZE, MIN_TUP_SIZE}, constants::MAX_COMPOSITE_DEPTH};
use super::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum VarType {
    Field,
    uint(usize),
    int(usize),
    bool,
    str(usize),
    Array(Box<VarType>, usize),
    Slice(Box<VarType>, usize),
    // Vec(Box<VarType>),
    tup(Vec<Box<VarType>>),
}

impl Eq for VarType {}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            
            VarType::Field | VarType::bool => write!(f, "{:?}", self),
            VarType::uint(size) => write!(f, "u{}", size),
            VarType::int(size) => write!(f, "i{}", size),
            VarType::str(size) => write!(f, "str<{}>", size),
            VarType::Array(type_param, size) => write!(f, "[{}; {}]", type_param, size),
            VarType::Slice(type_param, _) => write!(f, "[{}]", type_param),
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
        }
    }
}

pub fn basic_types() -> Vec<VarType> {
    let mut vec = vec![
        VarType::Field,
        VarType::bool,
    ];

    for i in 1..128{
        vec.push(VarType::uint(i));
        vec.push(VarType::int(i));
    }
    vec
}

pub fn random_int_type() -> VarType {
    match random::gen_range(0, 3) {
        0 => VarType::Field,
        1 => VarType::uint(random::gen_range(1, 128)),
        2 => VarType::int(random::gen_range(1, 128)),
        _ => VarType::Field,
    }
}

pub fn random_basic_type() -> VarType {
    match random::gen_range(0, 4) {
        0 => VarType::Field,
        1 => VarType::uint(random::gen_range(1, 128)),
        2 => VarType::int(random::gen_range(1, 128)),
        3 => VarType::bool,
        _ => VarType::Field,
    }
}

pub fn random_type() -> VarType {
    match random::gen_range(0, 8) {
        0 => VarType::Field,
        1 => VarType::uint(random::gen_range(1, 128)),
        2 => VarType::int(random::gen_range(1, 128)),
        3 => VarType::bool,
        4 => VarType::str(random::gen_range(0, MAX_COMPOSITE_SIZE)),
        5 => VarType::Array(Box::new(random_type_with_depth(MAX_COMPOSITE_DEPTH).clone()), random::gen_range(0, MAX_COMPOSITE_SIZE)),
        6 => VarType::Slice(Box::new(random_type_with_depth(MAX_COMPOSITE_DEPTH).clone()), random::gen_range(0, MAX_COMPOSITE_SIZE)),
        7 => {
            let size = random::gen_range(MIN_TUP_SIZE, MAX_COMPOSITE_SIZE);
            let mut vec_tup = Vec::with_capacity(size);
            for _ in 0..size {
                vec_tup.push(Box::new(random_type_with_depth(MAX_COMPOSITE_DEPTH).clone()));
            }
            VarType::tup(vec_tup)
        },
        _ => VarType::Field,
    }
}

fn random_type_with_depth(depth: usize) -> VarType {
    if depth == 0 {
        match random::gen_range(0, 5) {
            0 => VarType::Field,
            1 => VarType::uint(random::gen_range(1, 128)),
            2 => VarType::int(random::gen_range(1, 128)),
            3 => VarType::bool,
            4 => VarType::str(random::gen_range(0, MAX_COMPOSITE_SIZE)),
            _ => VarType::Field,
        }
    } else {
        match random::gen_range(0, 7) {
            0 => VarType::Field,
            1 => VarType::uint(random::gen_range(1, 128)),
            2 => VarType::int(random::gen_range(1, 128)),
            3 => VarType::bool,
            4 => VarType::str(random::gen_range(0, MAX_COMPOSITE_SIZE)),
            5 => VarType::Array(Box::new(random_type_with_depth(depth -1).clone()), random::gen_range(0, MAX_COMPOSITE_SIZE)),
            6 => {
                let size = random::gen_range(MIN_TUP_SIZE, MAX_COMPOSITE_SIZE);
                let mut vec_tup = Vec::with_capacity(size);
                for _ in 0..size {
                    vec_tup.push(Box::new(random_type_with_depth(depth -1).clone()));
                }
                VarType::tup(vec_tup)
            },
            _ => VarType::Field,
        }
    }
}

pub fn supported_arithmetic_operator_by_type(var_type: &VarType) -> Vec<Operator> {
    match var_type {
        VarType::Field => vec![Operator::Add, Operator::Subtract, Operator::Multiply, Operator::Divide],
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
        VarType::Field | VarType::str(_) | VarType::Slice(_,_) | VarType::Array(_,_) => vec![Operator::Equal, Operator::NotEqual],
        _ => vec![], // Handle unknown types
    }
}

pub fn way_to_type(source_type: &VarType, aim_type: &VarType) -> Option<String> {
    match source_type {
        VarType::Field | VarType::bool | VarType::str(_)
            => if source_type == aim_type {
                return Some("".to_string());
            } else {
                return None;
        },
        VarType::uint(size)
            => match aim_type {
                VarType::uint(aim_size) 
                    => if aim_size == size {
                        return Some("".to_string());
                    } else {
                        return None;
                    },
                _ => None,
        },
        VarType::int(size)
            => match aim_type {
                VarType::int(aim_size) 
                    => if aim_size == size {
                        return Some("".to_string());
                    } else {
                        return None;
                    },
                _ => None,
        },
        VarType::Array(type_param, size) => match way_to_type(type_param, aim_type) {
            Some(str) => if *size == 0 {
                    return None;
                } else {
                    return Some(format!("[{}]{}", random::gen_range(0, *size), str));
                },
            None => return None,
        },
        VarType::Slice(type_param, size) => match way_to_type(type_param, aim_type) {
            Some(str) => if *size == 0 {
                    return None;
                } else {
                    return Some(format!("[{}]{}", random::gen_range(0, *size), str));
                },
            None => return None,
        },
        VarType::tup(vec_type_param) => {
            for (ind,type_param) in vec_type_param.iter().enumerate() {
                if let Some(str) = way_to_type(type_param, aim_type) {
                    return Some(format!(".{}{}", ind, str));
                }
            }
            return None
        },
    }
}