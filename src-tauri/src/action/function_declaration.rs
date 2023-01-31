use super::super::Sequence;
use crate::action::arg::Arg;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FunctionDeclaration {
    name: String,
    args: Vec<Arg>,
    body: Sequence,
}
