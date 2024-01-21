use crate::variables::var_type::VarType;
use crate::variables::operand::Operand;
use crate::random;

use crate::variables::var_type;

use super::operator::{self, Operator};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Operation {
    operation_type: VarType,
    operator: Operator,
    first_element: Operand,
    second_element: Operand,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.first_element {
            Operand::Variable(variable) => write!(f, "({}",variable.name_and_way(&self.interaction_type()))?,
            Operand::Operation(interaction) => write!(f, "({}", interaction)?,
            Operand::Value(value,_) => write!(f, "({}", value)?,
        };

        write!(f, " {} ", self.operator)?;

        match &self.second_element {
            Operand::Variable(variable) =>  write!(f, "{})",variable.name_and_way(&self.interaction_type()))?,
            Operand::Operation(interaction) => write!(f, "{})", interaction)?,
            Operand::Value(value,_) => write!(f, "{})", value)?,
        };

        write!(f,"")

    }
}

impl Operation {
    pub fn new(operation_type: &VarType, operator: Option<Operator>, first_element: Operand, second_element: Operand) -> Self {

        let operator = match operator {
            Some(v) => v,
            None => random::choose_random_item_from_vec(&var_type::supported_arithmetic_operator_by_type(operation_type)),
        };

        Operation {
            operation_type: operation_type.clone(),
            operator,
            first_element,
            second_element,
        }
    }

    pub fn interaction_type(&self) -> VarType {
        self.operation_type.clone()
    }
}