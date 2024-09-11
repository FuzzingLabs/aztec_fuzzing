// Represent a Trait that is already implemented in the language
#[derive(Clone, PartialEq)]
pub enum BasicTrait {
    Eq,
    Ord,
}

impl BasicTrait {
    pub fn iterator() -> impl Iterator<Item = BasicTrait> {
        [BasicTrait::Eq, BasicTrait::Ord].iter().cloned()
    }
}

impl std::fmt::Display for BasicTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicTrait::Eq => write!(f, "Eq"),
            BasicTrait::Ord => write!(f, "Ord"),
        }
    }
}
