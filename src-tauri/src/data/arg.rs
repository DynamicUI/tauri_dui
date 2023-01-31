use super::input::Input;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Arg {
    name: String,
    input: Input,
}
