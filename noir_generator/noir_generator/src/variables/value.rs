use crate::variables::var_type::VarType;
use crate::constants::MAX_COMPOSITE_SIZE;
use crate::random;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Field(u128),
    Uint(u128),
    Int(i128),
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
            Value::Uint(value) => write!(f, "{}", value),
            Value::Int(value) => write!(f, "{}", value),
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
        VarType::uint(size) => Value::Uint(random::gen_random_uint(*size)),
        VarType::int(size) => Value::Int(random::gen_random_int(*size)),
        VarType::bool => Value::Bool(random::gen_bool()),
        VarType::str(size) => Value::Str(random::gen_str(*size)),
        VarType::Array(type_param, size) => {
            let mut random_vec = Vec::with_capacity(*size);
            for _ in 0..*size {
                random_vec.push(random_value(&type_param))
            }
            Value::Array(random_vec)
        },
        VarType::Slice(type_param, size) => {
            let mut random_vec = Vec::with_capacity(*size);
            for _ in 0..*size {
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
