use super::{
    basic_trait::BasicTrait, list_structs::ListStructs, operator::Operator, struct_type::StructType,
};
use crate::{tools::constants::CONFIG, tools::random::Random};

/// List of every types avalaible
#[derive(Clone)]
pub enum VarType {
    Field,
    UInt(usize),
    Int(usize),
    Bool,
    Str(usize),
    Array(Box<VarType>, usize),
    Slice(Box<VarType>, usize),
    Tup(Vec<Box<VarType>>),
    Strct(StructType),
    Reference(Box<VarType>),
    Generic(Vec<BasicTrait>),
}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarType::Field => write!(f, "Field"),
            VarType::Bool => write!(f, "bool"),
            VarType::UInt(size) => write!(f, "u{}", size),
            VarType::Int(size) => write!(f, "i{}", size),
            VarType::Str(size) => write!(f, "str<{}>", size),
            VarType::Array(type_param, size) => {
                if *size == usize::max_value() {
                    return write!(f, "[{}; N]", type_param);
                }
                write!(f, "[{}; {}]", type_param, size)
            }
            VarType::Slice(type_param, _) => write!(f, "[{}]", type_param),
            VarType::Tup(vec_type_param) => {
                write!(f, "(")?;
                for (i, type_param) in vec_type_param.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", type_param)?;
                }
                write!(f, ")")
            }
            VarType::Strct(strct) => write!(f, "{}", strct.name()),
            VarType::Reference(type_param) => write!(f, "&mut {}", type_param),
            VarType::Generic(_) => write!(f, "T"),
        }
    }
}

/// Returns a list of basic non-composite types
pub fn basic_types() -> Vec<&'static VarType> {
    vec![
        &VarType::Field,
        &VarType::Bool,
        &VarType::UInt(1),
        &VarType::UInt(8),
        &VarType::UInt(32),
        &VarType::UInt(64),
        &VarType::Int(1),
        &VarType::Int(8),
        &VarType::Int(32),
        &VarType::Int(64),
    ]
}

/// Returns a random bit size for integer and unsigned integer types
fn random_bit_size(random: &mut Random) -> usize {
    match random.gen_range(0, 4) {
        0 => 1,
        1 => 8,
        2 => 32,
        3 => 64,
        _ => panic!("Impossible case"),
    }
}

/// Returns a randomly selected basic non-composite type
pub fn random_basic_type(random: &mut Random) -> VarType {
    match random.gen_range(0, 5) {
        0 => VarType::Field,
        1 => VarType::UInt(random_bit_size(random)),
        2 => VarType::Int(random_bit_size(random)),
        3 => VarType::Bool,
        4 => VarType::Str(random.gen_range(0, CONFIG.max_composite_size)),
        _ => VarType::Field,
    }
}

/// Return a random selected type
pub fn random_type(random: &mut Random, list_structs: &ListStructs) -> VarType {
    match random.gen_range(0, 9 + CONFIG.use_of_slice) {
        0 => VarType::Field,
        1 => VarType::UInt(random_bit_size(random)),
        2 => VarType::Int(random_bit_size(random)),
        3 => VarType::Bool,
        4 => VarType::Str(random.gen_range(0, CONFIG.max_composite_size)),
        5 => VarType::Array(
            Box::new(random_type_with_depth(
                random,
                list_structs,
                CONFIG.max_composite_depth,
            )),
            random.gen_range(0, CONFIG.max_composite_size),
        ),
        6 => {
            let size = random.gen_range(2, CONFIG.max_composite_size);
            let mut vec_tup = Vec::with_capacity(size);
            for _ in 0..size {
                vec_tup.push(Box::new(random_type_with_depth(
                    random,
                    list_structs,
                    CONFIG.max_composite_depth,
                )));
            }
            VarType::Tup(vec_tup)
        }
        7 => {
            if list_structs.is_empty() {
                random_type_with_depth(random, list_structs, CONFIG.max_composite_depth)
            } else {
                VarType::Strct(list_structs.get_random(random))
            }
        }
        8 => VarType::Reference(Box::new(random_basic_type(random))),
        9 => VarType::Slice(
            Box::new(random_type_with_depth(
                random,
                list_structs,
                CONFIG.max_composite_depth,
            )),
            random.gen_range(0, CONFIG.max_composite_size),
        ),
        _ => VarType::Field,
    }
}

/// Used to limit the depth of composite types randomly generated
pub fn random_type_with_depth(
    random: &mut Random,
    list_structs: &ListStructs,
    depth: usize,
) -> VarType {
    if depth == 0 {
        match random.gen_range(0, 5) {
            0 => VarType::Field,
            1 => VarType::UInt(random_bit_size(random)),
            2 => VarType::Int(random_bit_size(random)),
            3 => VarType::Bool,
            4 => VarType::Str(random.gen_range(0, CONFIG.max_composite_size)),
            _ => VarType::Field,
        }
    } else {
        match random.gen_range(0, 9) {
            0 => VarType::Field,
            1 => VarType::UInt(random_bit_size(random)),
            2 => VarType::Int(random_bit_size(random)),
            3 => VarType::Bool,
            4 => VarType::Str(random.gen_range(0, CONFIG.max_composite_size)),
            5 => VarType::Array(
                Box::new(random_type_with_depth(random, list_structs, depth - 1)),
                random.gen_range(0, CONFIG.max_composite_size),
            ),
            6 => {
                let size = random.gen_range(2, CONFIG.max_composite_size);
                let mut vec_tup = Vec::with_capacity(size);
                for _ in 0..size {
                    vec_tup.push(Box::new(random_type_with_depth(
                        random,
                        list_structs,
                        depth - 1,
                    )));
                }
                VarType::Tup(vec_tup)
            }
            7 => {
                if list_structs.is_empty() || depth != CONFIG.max_composite_depth {
                    random_type_with_depth(random, list_structs, depth - 1)
                } else {
                    VarType::Strct(list_structs.get_random(random))
                }
            }
            8 => VarType::Reference(Box::new(random_basic_type(random))),
            _ => VarType::Field,
        }
    }
}

/// Return a random selected type
pub fn random_type_without_reference(random: &mut Random, list_structs: &ListStructs) -> VarType {
    match random.gen_range(0, 7 + CONFIG.use_of_slice) {
        0 => VarType::Field,
        1 => VarType::UInt(random_bit_size(random)),
        2 => VarType::Int(random_bit_size(random)),
        3 => VarType::Bool,
        4 => VarType::Str(random.gen_range(0, CONFIG.max_composite_size)),
        5 => VarType::Array(
            Box::new(random_type_without_reference_with_depth(
                random,
                list_structs,
                CONFIG.max_composite_depth,
            )),
            random.gen_range(0, CONFIG.max_composite_size),
        ),
        6 => {
            let size = random.gen_range(2, CONFIG.max_composite_size);
            let mut vec_tup = Vec::with_capacity(size);
            for _ in 0..size {
                vec_tup.push(Box::new(random_type_without_reference_with_depth(
                    random,
                    list_structs,
                    CONFIG.max_composite_depth,
                )));
            }
            VarType::Tup(vec_tup)
        }
        7 => VarType::Slice(
            Box::new(random_type_without_reference_with_depth(
                random,
                list_structs,
                CONFIG.max_composite_depth,
            )),
            random.gen_range(0, CONFIG.max_composite_size),
        ),
        _ => VarType::Field,
    }
}

/// Used to limit the depth of composite types randomly generated
pub fn random_type_without_reference_with_depth(
    random: &mut Random,
    list_structs: &ListStructs,
    depth: usize,
) -> VarType {
    if depth == 0 {
        match random.gen_range(0, 5) {
            0 => VarType::Field,
            1 => VarType::UInt(random_bit_size(random)),
            2 => VarType::Int(random_bit_size(random)),
            3 => VarType::Bool,
            4 => VarType::Str(random.gen_range(0, CONFIG.max_composite_size)),
            _ => VarType::Field,
        }
    } else {
        match random.gen_range(0, 7) {
            0 => VarType::Field,
            1 => VarType::UInt(random_bit_size(random)),
            2 => VarType::Int(random_bit_size(random)),
            3 => VarType::Bool,
            4 => VarType::Str(random.gen_range(0, CONFIG.max_composite_size)),
            5 => VarType::Array(
                Box::new(random_type_without_reference_with_depth(
                    random,
                    list_structs,
                    depth - 1,
                )),
                random.gen_range(0, CONFIG.max_composite_size),
            ),
            6 => {
                let size = random.gen_range(2, CONFIG.max_composite_size);
                let mut vec_tup = Vec::with_capacity(size);
                for _ in 0..size {
                    vec_tup.push(Box::new(random_type_without_reference_with_depth(
                        random,
                        list_structs,
                        depth - 1,
                    )));
                }
                VarType::Tup(vec_tup)
            }
            _ => VarType::Field,
        }
    }
}

/// Returns the list of comparison operators supported by the given variable type
pub fn supported_arithmetic_operator_by_type(var_type: &VarType) -> Vec<Operator> {
    match var_type {
        VarType::Field => vec![
            Operator::Add,
            Operator::Subtract,
            Operator::Multiply,
            Operator::Divide,
        ],
        VarType::Int(_) => vec![
            Operator::Add,
            Operator::Subtract,
            Operator::Multiply,
            Operator::Divide,
        ], //Operator::Lshift, Operator::Rshift
        VarType::UInt(_) => vec![
            Operator::Add,
            Operator::Subtract,
            Operator::Multiply,
            Operator::Divide,
        ], //Operator::Lshift, Operator::Rshift
        VarType::Bool => vec![
            Operator::Equal,
            Operator::NotEqual,
            Operator::Or,
            Operator::And,
        ],
        _ => vec![], // Handle unknown types
    }
}

/// Returns the list of comparison operators supported by the given variable type
pub fn supported_comparator_operator_by_type(var_type: &VarType) -> Vec<Operator> {
    match var_type {
        VarType::UInt(_) | VarType::Int(_) => vec![
            Operator::Equal,
            Operator::NotEqual,
            Operator::Lesser,
            Operator::Greater,
            Operator::LesserOrEqual,
            Operator::GreaterOrEqual,
        ],
        VarType::Bool => vec![
            Operator::Equal,
            Operator::NotEqual,
            Operator::Or,
            Operator::And,
        ],
        VarType::Field | VarType::Str(_) | VarType::Slice(_, _) | VarType::Array(_, _) => {
            vec![Operator::Equal, Operator::NotEqual]
        }
        VarType::Generic(vec_trait) => {
            let mut ret = Vec::new();
            if vec_trait.contains(&BasicTrait::Eq) {
                ret.push(Operator::Equal);
                ret.push(Operator::NotEqual);
            }
            if vec_trait.contains(&BasicTrait::Ord) {
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

/// Returns true if both types given in parameters are the same
pub fn is_same_type(first_type: &VarType, second_type: &VarType) -> bool {
    match (first_type, second_type) {
        (VarType::Field, VarType::Field) => true,
        (VarType::UInt(s1), VarType::UInt(s2)) => s1 == s2,
        (VarType::Int(s1), VarType::Int(s2)) => s1 == s2,
        (VarType::Bool, VarType::Bool) => true,
        (VarType::Str(s1), VarType::Str(s2)) => s1 == s2,
        (VarType::Array(var_type1, s1), VarType::Array(var_type2, s2)) => {
            s1 == s2 && is_same_type(var_type1, var_type2)
        }
        (VarType::Slice(var_type1, s1), VarType::Slice(var_type2, s2)) => {
            s1 == s2 && is_same_type(var_type1, var_type2)
        }
        (VarType::Tup(vec_type1), VarType::Tup(vec_type2)) => {
            if vec_type1.len() != vec_type2.len() {
                false
            } else {
                for i in 0..vec_type1.len() {
                    if !is_same_type(
                        vec_type1.get(i).expect("None in tup"),
                        vec_type2.get(i).expect("None in tup"),
                    ) {
                        return false;
                    }
                }
                true
            }
        }
        (VarType::Strct(strct1), VarType::Strct(strct2)) => strct1.name() == strct2.name(),
        (VarType::Reference(var_type1), VarType::Reference(var_type2)) => {
            is_same_type(var_type1, var_type2)
        }

        (VarType::Generic(_), VarType::Generic(_)) => true,
        _ => false,
    }
}

/// Returns a string representing a statement of the same type as aim_type using source_type, or None if there is no way
/// # Example
/// The way from [u8; 1] to u8 is [0]
pub fn way_to_type(
    random: &mut Random,
    source_type: &VarType,
    aim_type: &VarType,
    is_a_ref: &mut bool,
) -> Option<String> {
    match source_type {
        VarType::Field
        | VarType::Bool
        | VarType::Str(_)
        | VarType::UInt(_)
        | VarType::Int(_)
        | VarType::Generic(_) => {
            if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                *is_a_ref = false;
                None
            }
        }
        VarType::Array(type_param, size) => {
            if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                if let Some(str) = way_to_type(random, type_param, aim_type, is_a_ref) {
                    if *size != 0 {
                        return Some(format!("[{}]{}", random.gen_range(0, *size), str));
                    }
                }
                *is_a_ref = false;
                None
            }
        }
        VarType::Slice(type_param, size) => {
            if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                if let Some(str) = way_to_type(random, type_param, aim_type, is_a_ref) {
                    if *size != 0 {
                        return Some(format!("[{}]{}", random.gen_range(0, *size), str));
                    }
                }
                *is_a_ref = false;
                None
            }
        }
        VarType::Tup(vec_type_param) => {
            if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                for (ind, type_param) in vec_type_param.iter().enumerate() {
                    if let Some(str) = way_to_type(random, type_param, aim_type, is_a_ref) {
                        return Some(format!(".{}{}", ind, str));
                    }
                }
                *is_a_ref = false;
                None
            }
        }
        VarType::Strct(strct) => {
            if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                for (_, type_param) in strct.key_types().iter().enumerate() {
                    if let Some(str) = way_to_type(random, &type_param.0, aim_type, is_a_ref) {
                        return Some(format!(".{}{}", type_param.1, str));
                    }
                }
                *is_a_ref = false;
                None
            }
        }
        VarType::Reference(type_param) => {
            if is_same_type(source_type, aim_type) {
                Some("".to_string())
            } else {
                *is_a_ref = true;
                way_to_type(random, type_param, aim_type, is_a_ref)
            }
        }
    }
}
