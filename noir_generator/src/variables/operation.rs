use crate::{tools::random::Random, variables::var_type::VarType};
use crate::variables::operand::Operand;
use super::operator::Operator;

/// Represent an operation between two operand
#[derive(Clone)]
pub(crate) struct Operation {
    operation_type: VarType,
    operator: Operator,
    first_element: Operand,
    second_element: Operand,
}

impl Operation {
    pub fn new(operation_type: &VarType, operator: Operator, first_element: Operand, second_element: Operand) -> Self {

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

    pub fn to_string(&self, random: &mut Random) -> String {
        let mut ret = String::new();

        match &self.first_element {
            Operand::Variable(variable) => ret = format!("{}({}", ret, variable.name_and_way(random, &self.interaction_type())),
            Operand::Operation(interaction) => ret = format!("{}({}", ret, interaction.to_string(random)),
            Operand::Value(value,_) => ret = format!("{}({}", ret, value),
        };

        ret = format!("{} {} ", ret, self.operator);

        match &self.second_element {
            Operand::Variable(variable) => ret = format!("{}{})", ret, variable.name_and_way(random, &self.interaction_type())),
            Operand::Operation(interaction) => ret = format!("{}{})", ret, interaction.to_string(random)),
            Operand::Value(value,_) => ret = format!("{}{})", ret, value),
        };

        ret
    }
}