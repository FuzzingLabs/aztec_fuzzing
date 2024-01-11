use crate::variables::var_type::VarType;
use crate::constants::MAX_COMPOSITE_SIZE;
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
    Array(Vec<Value>),
    Slice(Vec<Value>),
    // Vec(Vec<Value>),
    Tup(Vec<Value>),
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
            Value::Array(value) => {
                write!(f, "[")?;
                for (i, item) in value.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Value::Slice(value) => {
                write!(f, "[")?;
                if value.len() > 1 && value.iter().all(|x| *x == value[0]) {
                    write!(f, "{}; {}", value[0], value.len())?;
                } else {
                    for (i, item) in value.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", item)?;
                    }
                }
                write!(f, "]")
            },

            // Value::Vec(value) => {
            //     write!(f, "(")?;
            //     for (i, item) in value.iter().enumerate() {
            //         if i > 0 {
            //             write!(f, ", ")?;
            //         }
            //         write!(f, "{}", item)?;
            //     }
            //     write!(f, ")")
            // },

            Value::Tup(value) => {
                write!(f, "(")?;
                for (i, item) in value.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            },
        }
    }
}

pub fn random_value(var_type: &VarType) -> Value {
    match var_type {
        VarType::Field => Value::Field(random::gen_field()),
        VarType::u8 => Value::U8(random::gen_u8()),
        VarType::u16 => Value::U16(random::gen_u16()),
        VarType::u32 => Value::U32(random::gen_u32()),
        VarType::u64 => Value::U64(random::gen_u64()),
        VarType::u127 => Value::U127(random::gen_u127()),
        VarType::i8 => Value::I8(random::gen_i8()),
        VarType::i16 => Value::I16(random::gen_i16()),
        VarType::i32 => Value::I32(random::gen_i32()),
        VarType::i64 => Value::I64(random::gen_i64()),
        VarType::i127 => Value::I127(random::gen_i127()),
        VarType::bool => Value::Bool(random::gen_bool()),
        VarType::str(size) => Value::Str(random::gen_str(*size)),
        VarType::Array(type_param, size) => {
            let mut random_vec = Vec::with_capacity(*size);
            for _ in 0..*size {
                random_vec.push(random_value(&type_param))
            }
            Value::Array(random_vec)
        },
        VarType::Slice(type_param) => {
            let size = random::gen_range(0, MAX_COMPOSITE_SIZE);
            let mut random_vec = Vec::with_capacity(size);
            for _ in 0..size {
                random_vec.push(random_value(&type_param))
            }
            Value::Slice(random_vec)
        },

        // VarType::Vec(value) => {
        //     let size = random::gen_range(0, MAX_COMPOSITE_SIZE);
        //     let mut random_vec = Vec::with_capacity(size);
        //     for _ in 0..size {
        //         random_vec.push(random_value(&value))
        //     }
        //     Value::Vec(random_vec)
        // },

        VarType::tup(vec_type_param) => {
            let size = vec_type_param.len();
            let mut random_vec = Vec::with_capacity(size);
            for i in 0..size {
                random_vec.push(random_value(&vec_type_param[i]))
            }
            Value::Tup(random_vec)
        },

        _ => unimplemented!("Type not yet supported"),
    }
}
