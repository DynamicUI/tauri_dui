use crate::action::control_flow::ControlFlow;
use crate::action::function_call::FunctionCall;
use crate::action::function_declaration::FunctionDeclaration;
use crate::action::variable_declaration::VariableDeclaration;
use crate::action::{Action, ActionData};
use crate::builtins::{execute_native_function, BUILTINS_FUNCTIONS_LIST};
use crate::sequence::Sequence;

pub fn execute_function_call(
    function: &FunctionCall,
    functions: &Vec<FunctionDeclaration>,
    variables: &Vec<VariableDeclaration>,
) -> Result<(), ()> {
    if (is_native_function(&function.name)) {
        execute_native_function(function);
        return Ok(());
    }
    todo!("Function call not implemented");
}

pub fn is_native_function(name: &str) -> bool {
    BUILTINS_FUNCTIONS_LIST.contains(&name)
}

pub fn execute_action(
    action: &Action,
    functions: &Vec<FunctionDeclaration>,
    variables: &Vec<VariableDeclaration>,
) {
    match &action.data {
        // TODO faire ces 2 pendant l'execution ou plus tot ??
        ActionData::VariableDeclaration(variable_declaration) => {}
        ActionData::FunctionDeclaration(function) => {}

        ActionData::ControlFlow(control_flow) => match control_flow {
            ControlFlow::While { condition, body } => {
                todo!("While not implemented")
            }
            ControlFlow::Loop { body } => {
                todo!("Loop not implemented")
            }
            ControlFlow::If {
                condition,
                body,
                else_body,
            } => {
                todo!("If not implemented")
            }
            ControlFlow::IfElseIf {
                condition,
                body,
                else_if,
            } => {
                todo!("IfElse not implemented")
            }
            ControlFlow::For {
                init,
                condition,
                update,
                body,
            } => {
                todo!("For not implemented")
            }
        },
        ActionData::FunctionCall(function) => {
            match execute_function_call(function, functions, variables) {
                Ok(_) => {}
                Err(_) => {
                    todo!("Function call failed");
                }
            }
        }
    }
}

pub fn execute_sequence(sequence: &Sequence) {
    let functions = sequence.get_user_functions();
    let variables = sequence.get_user_variables();
    for action in sequence.actions.iter() {
        execute_action(action, &functions, &variables);
    }
}
