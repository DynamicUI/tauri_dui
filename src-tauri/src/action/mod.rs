pub mod control_flow;
pub mod function_call;
pub mod function_declaration;
pub mod variable_declaration;
use {
    control_flow::ControlFlow, function_call::FunctionCall,
    function_declaration::FunctionDeclaration, variable_declaration::VariableDeclaration,
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum ActionData {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    FunctionCall(FunctionCall),
    ControlFlow(ControlFlow),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Action {
    pub data: ActionData,
    pub id: usize,
    /* TODO autre ? */
}

impl Action {
    pub fn new(data: ActionData, id: usize) -> Self {
        Action { data, id }
    }
}
