use crate::variables::var_type::VarType;
use crate::random;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Field(u128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U127(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I127(i128),
    Bool(bool),
    Str(String),
    // Array(Vec<Value>),
    // Slice(Vec<Value>),
    // Vec(Vec<Value>),
    // Tup(Vec<Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Field(value) => write!(f, "{}", value),
            Value::U8(value) => write!(f, "{}", value),
            Value::U16(value) => write!(f, "{}", value),
            Value::U32(value) => write!(f, "{}", value),
            Value::U64(value) => write!(f, "{}", value),
            Value::U127(value) => write!(f, "{}", value),
            Value::I8(value) => write!(f, "{}", value),
            Value::I16(value) => write!(f, "{}", value),
            Value::I32(value) => write!(f, "{}", value),
            Value::I64(value) => write!(f, "{}", value),
            Value::I127(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Str(value) => write!(f, "\"{}\"", value),
            // Handle other variants as needed
        }
    }
}

pub fn random_value(var_type: &VarType) -> Value {
    match var_type {
        VarType::Field => Value::Field(random::gen_field()),
        VarType::U8 => Value::U8(random::gen_u8()),
        VarType::U16 => Value::U16(random::gen_u16()),
        VarType::U32 => Value::U32(random::gen_u32()),
        VarType::U64 => Value::U64(random::gen_u64()),
        VarType::U127 => Value::U127(random::gen_u127()),
        VarType::I8 => Value::I8(random::gen_i8()),
        VarType::I16 => Value::I16(random::gen_i16()),
        VarType::I32 => Value::I32(random::gen_i32()),
        VarType::I64 => Value::I64(random::gen_i64()),
        VarType::I127 => Value::I127(random::gen_i127()),
        VarType::Bool => Value::Bool(random::gen_bool()),
        VarType::Str => Value::Str(random::gen_str()),
        _ => unimplemented!("Type not yet supported"),
    }
}
