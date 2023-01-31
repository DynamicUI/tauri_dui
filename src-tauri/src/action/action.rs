use super::{
    control_flow::ControlFlow, function_call::FunctionCall,
    function_declaration::FunctionDeclaration, variable_declaration::VariableDeclaration,
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum Action {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    FunctionCall(FunctionCall),
    ControlFlow(ControlFlow),
}
