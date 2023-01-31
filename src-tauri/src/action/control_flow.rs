
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
pub enum ControlFlow {
    Loop,
    While,
    For,
    If,
    /* ... */
}
