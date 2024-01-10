use crate::variables::var_type::VarType;
use crate::variables::value::Value;
use crate::variables::value_provider::ValueProviderEnum;
use crate::random;

use crate::variables::var_type;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Interaction {
    interaction_type: VarType,
    operand: String,
    value: Value,
    first_element: ValueProviderEnum,
    second_element: ValueProviderEnum,
}

impl std::fmt::Display for Interaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.first_element {
            ValueProviderEnum::Variable(variable) => write!(f, "({}", variable.name())?,
            ValueProviderEnum::Interaction(interaction) => write!(f, "({}", interaction)?,
        };

        write!(f, " {} ", self.operand)?;

        match &self.second_element {
            ValueProviderEnum::Variable(variable) => write!(f, "{})", variable.name())?,
            ValueProviderEnum::Interaction(interaction) => write!(f, "{})", interaction)?,
        };

        write!(f,"")

    }
}

impl Interaction {
    pub fn new(operand: Option<String>, first_element: ValueProviderEnum, second_element: ValueProviderEnum) -> Self {

        let interaction_type = match &first_element {
            ValueProviderEnum::Variable(variable) => variable.var_type(),
            ValueProviderEnum::Interaction(interaction) => interaction.interaction_type(),
        };

        let operand = match operand {
            Some(v) => v,
            None => random::choose_random_item_from_vec(&var_type::supported_operations_by_type(interaction_type.clone())).to_string(),
        };

        let first_value = match &first_element {
            ValueProviderEnum::Variable(variable) => variable.value(),
            ValueProviderEnum::Interaction(interaction) => interaction.value(),
        };

        let second_value = match &second_element {
            ValueProviderEnum::Variable(variable) => variable.value(),
            ValueProviderEnum::Interaction(interaction) => interaction.value(),
        };

        let value = first_value; // TODO operation between 1st and 2nd

        Interaction {
            interaction_type,
            operand,
            value,
            first_element,
            second_element,
        }
    }

    pub fn value(&self) -> Value {
        self.value.clone()
    }

    pub fn interaction_type(&self) -> VarType {
        self.interaction_type.clone()
    }
}