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

    pub fn from_number(number: f64) -> Self {
        Self::Text(number.to_string())
    }

    pub fn printable(&self) -> Option<String> {
        match self {
            Self::Text(text) => Some(text.clone()),
            Self::Lambda(_) => None,
            Self::Variable(_) => Some("Variable".to_string()),
            Self::FunctionCall(_) => Some("FunctionCall".to_string()),
        }
    }
}
