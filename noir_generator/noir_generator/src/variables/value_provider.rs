use crate::variables::variable::Variable;
use crate::variables::interaction::Interaction;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ValueProviderEnum {
    Variable(Variable),
    Interaction(Box<Interaction>),
}
