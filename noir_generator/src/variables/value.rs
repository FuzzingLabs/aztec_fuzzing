use crate::variables::var_type::VarType;
use crate::tools::random::Random;

/// Represent a value depending on the type
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Field(u128),
    Uint(u128),
    Int(i128),
    Bool(bool),
    Str(String),
    Array(Vec<Value>),
    Slice(Vec<Value>),
    Tup(Vec<Value>),
    Strct(Vec<(Value, String)>, String),
    Reference(Box<Value>),

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
                    if i != 0 {
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
                        if i != 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", item)?;
                    }
                }
                write!(f, "]")
            },

            Value::Tup(value) => {
                write!(f, "(")?;
                for (i, item) in value.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            },
            Value::Strct(key_val, name) => {
                write!(f, "{} {{", name)?;
                for (i, item) in key_val.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", item.1, item.0)?;
                }
                write!(f, "}}")
            },
            Value::Reference(value) => {
                write!(f, "&mut {}", value)
            },
        }
    }
}

/// Return a random value of a type given in the parameter
pub fn random_value(random: &mut Random, var_type: &VarType) -> Value {
    match var_type {
        VarType::field => Value::Field(random.gen_field()),
        VarType::uint(size) => Value::Uint(random.gen_random_uint(*size)),
        VarType::int(size) => Value::Int(random.gen_random_int(*size)),
        VarType::bool => Value::Bool(random.gen_bool()),
        VarType::str(size) => Value::Str(random.gen_str(*size)),
        VarType::array(type_param, size) => {
            let mut random_vec = Vec::with_capacity(*size);
            for _ in 0..*size {
                random_vec.push(random_value(random, &type_param))
            }
            Value::Array(random_vec)
        },
        VarType::slice(type_param, size) => {
            let mut random_vec = Vec::with_capacity(*size);
            for _ in 0..*size {
                random_vec.push(random_value(random, &type_param))
            }
            Value::Slice(random_vec)
        },

        VarType::tup(vec_type_param) => {
            let size = vec_type_param.len();
            let mut random_vec = Vec::with_capacity(size);
            for i in 0..size {
                random_vec.push(random_value(random, &vec_type_param[i]))
            }
            Value::Tup(random_vec)
        },

        VarType::strct(strct) => {
            let vec_type_param = strct.key_types();
            let size = vec_type_param.len();
            let mut random_vec = Vec::with_capacity(size);
            for i in 0..size {
                random_vec.push((random_value(random, &vec_type_param[i].0), vec_type_param[i].1.clone()))
            }
            Value::Strct(random_vec, strct.name().clone())
        },

        VarType::reference(type_param) => Value::Reference(Box::new(random_value(random, &type_param))),

        _ => unimplemented!("Type not yet supported"),
    }
}
