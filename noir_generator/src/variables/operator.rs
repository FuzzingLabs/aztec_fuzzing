// Represent an operator
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Xor,
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    Lesser,
    LesserOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Subtract => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
                Operator::Xor => "^",
                Operator::And => "&",
                Operator::Or => "|",
                Operator::Lshift => "<<",
                Operator::Rshift => ">>",
                Operator::Not => "!",
                Operator::Lesser => "<",
                Operator::LesserOrEqual => "<=",
                Operator::Greater => ">",
                Operator::GreaterOrEqual => ">=",
                Operator::Equal => "==",
                Operator::NotEqual => "!=",
            }
        )
    }
}
