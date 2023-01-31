use super::input::Input;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FunctionCall {
    name: String,
    args: Vec<Input>,
}

impl FunctionCall {
    pub fn new(name: String, args: Vec<Input>) -> Self {
        Self { name, args }
    }
}
