use crate::input::*;

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
pub struct VariableDeclaration {
    name: String,
    input: Input,
}

