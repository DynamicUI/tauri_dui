use crate::action::action::ActionData;
use crate::action::{
    action::Action, function_call::FunctionCall, input::Input,
    variable_declaration::VariableDeclaration,
};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
pub enum Context {
    Main,
    FunctionBody,
    LambdaFunction,
    ControlFlowBody,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Sequence {
    #[serde(default)]
    id: usize,
    context: Context,
    actions: Vec<Action>,
}

pub struct SequencesState(Mutex<HashMap<usize, Sequence>>);
impl Default for SequencesState {
    fn default() -> Self {
        SequencesState(Mutex::new(HashMap::from([(
            0,
            Sequence {
                id: 0,
                context: Context::Main,
                actions: vec![
                    Action::new(
                        ActionData::VariableDeclaration(VariableDeclaration::new(
                            "greeting".to_string(),
                            Input::from_text("Hello World".to_string()),
                        )),
                        0,
                    ),
                    Action::new(
                        ActionData::FunctionCall(FunctionCall::new(
                            "print".to_string(),
                            vec![Input::from_variable("greeting".to_string())],
                        )),
                        1,
                    ),
                ],
            },
        )])))
    }
}

#[tauri::command]
pub fn get_sequences(sequences: tauri::State<SequencesState>) -> HashMap<usize, Sequence> {
    sequences.0.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_sequence_by_id(
    sequence_id: usize,
    sequences: tauri::State<SequencesState>,
) -> Option<Sequence> {
    match sequences.0.lock().unwrap().get(&sequence_id) {
        Some(sequence) => Some(sequence.clone()),
        None => None,
    }
}

#[tauri::command]
pub fn add_sequence(mut sequence: Sequence, sequences: tauri::State<SequencesState>) {
    println!("Adding sequence: {:?}", sequence);
    let new_id: usize = sequences.0.lock().unwrap().len();
    sequence.id = new_id;
    sequences.0.lock().unwrap().insert(new_id, sequence);
}
