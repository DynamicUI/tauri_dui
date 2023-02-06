use crate::{data::arg::Arg, sequence::Sequence};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FunctionDeclaration {
    name: String,
    args: Vec<Arg>,
    is_variadic: bool,
    body: Sequence,
}
