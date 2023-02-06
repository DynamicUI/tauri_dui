use crate::data::input::Input;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub input: Input,
}

impl VariableDeclaration {
    pub fn new(name: String, input: Input) -> Self {
        Self { name, input }
    }
}
