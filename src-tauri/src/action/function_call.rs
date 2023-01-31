use super::input::Input;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FunctionCall {
    name: String,
    args: Vec<Input>,
}
