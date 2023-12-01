#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    visible: bool,
    mutable: bool,
    type_: String,
    value: String,
}

impl Variable {
    pub fn new(name: String, visible: bool, mutable: bool, type_: String, value: String) -> Self {
        Self {
            name,
            visible,
            mutable,
            type_,
            value,
        }
    }

    pub fn is_public(&self) -> bool {
        self.visible
    }

    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &str {
        &self.type_
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "let{}{} {}: {} = {}",
            if self.is_public() { " pub" } else { "" },
            if self.is_mutable() { " mut" } else { "" },
            self.get_name(),
            self.get_type(),
            self.get_value()
        )
    }
}
