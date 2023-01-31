use crate::{action::function_call::FunctionCall, sequence::Sequence};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum Input {
    Text(String),
    Lambda(Sequence),
    Variable(String),           // TODO un string ? pour l'id ?
    FunctionCall(FunctionCall), // TODO name and args ?
}

impl Input {
    pub fn from_text(text: String) -> Self {
        Self::Text(text)
    }

    pub fn from_lambda(sequence: Sequence) -> Self {
        Self::Lambda(sequence)
    }

    pub fn from_variable(name: String) -> Self {
        Self::Variable(name)
    }
}
