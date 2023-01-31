use super::super::Sequence;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FunctionDeclaration {
    name: String,
    args: Vec<Arg>,
    body: Sequence,
}
