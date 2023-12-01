#[derive(Debug)]
pub struct Function {
    name: String,
    visible: bool,
    return_type: String,
    parameters: Vec<String>,
    body: Vec<String>,
}

impl Function {
    pub fn new(name: String, visible: bool, return_type: String, parameters: Vec<String>, body: Vec<String>) -> Self {
        Self {
            name,
            visible,
            return_type,
            parameters,
            body,
        }
    }

    pub fn is_public(&self) -> bool {
        self.visible
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_return_type(&self) -> &str {
        &self.return_type
    }

    pub fn get_parameters(&self) -> &Vec<String> {
        &self.parameters
    }

    pub fn get_body(&self) -> Vec<String> { // Change return type to Vec<String>
        self.body.clone()
    }

    pub fn set_body(&mut self, body: Vec<String>) {
        self.body = body;
    }

    pub fn add_line_to_body(&mut self, line: String) {
        self.body.push(line);
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parameters = String::new();
        for parameter in self.get_parameters() {
            parameters.push_str(parameter);
            parameters.push_str(", ");
        }
        parameters.pop();
        parameters.pop();

        let body: Vec<String> = self.get_body();
        let formatted_body: String = body.join("\n\t");

        let pub_keyword = if self.is_public() { "pub " } else { "" };

        write!(
            f,
            "{}fn {}({}) -> {} \n{{\n\t{}\n}}",
            pub_keyword,
            self.get_name(),
            parameters,
            self.get_return_type(),
            formatted_body
        )
    }
}
