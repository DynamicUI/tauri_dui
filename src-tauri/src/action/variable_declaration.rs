use crate::data::input::Input;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct VariableDeclaration {
    name: String,
    input: Input,
}

impl VariableDeclaration {
    pub fn new(name: String, input: Input) -> Self {
        Self { name, input }
    }
}
