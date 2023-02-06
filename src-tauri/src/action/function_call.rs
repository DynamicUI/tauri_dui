use crate::data::input::Input;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Input>,
    pub is_variadic: bool,
}

impl FunctionCall {
    pub fn new(name: String, args: Vec<Input>) -> Self {
        Self {
            name,
            args,
            is_variadic: false,
        }
    }
}
