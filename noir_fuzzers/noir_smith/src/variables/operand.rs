use super::value::Value;
use super::var_type::VarType;
use crate::variables::operation::Operation;
use crate::variables::variable::Variable;

/// Represent a part of an operation such that it can be a variable, a raw value, or another operation
#[derive(Clone)]
pub enum Operand {
    Variable(Variable),
    Operation(Box<Operation>),
    Value(Value, VarType),
}
