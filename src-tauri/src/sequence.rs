use crate::action::function_declaration::FunctionDeclaration;
use crate::{
    action::variable_declaration::VariableDeclaration,
    action::{function_call::FunctionCall, Action, ActionData},
    data::input::Input,
};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, Default)]
pub enum Context {
    #[default]
    Main,
    FunctionBody,
    LambdaFunction,
    ControlFlowBody,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Sequence {
    #[serde(default)]
    id: usize,
    context: Context,
    pub actions: Vec<Action>,
}
impl Sequence {
    pub fn get_user_variables(&self) -> Vec<VariableDeclaration> {
        let mut res: Vec<VariableDeclaration> = Vec::new();
        self.actions.iter().for_each(|action| {
            if let ActionData::VariableDeclaration(declaration) = &action.data {
                res.push(declaration.clone());
            };
        });
        return res;
    }

    pub fn get_user_functions(&self) -> Vec<FunctionDeclaration> {
        let mut res: Vec<FunctionDeclaration> = Vec::new();
        self.actions.iter().for_each(|action| {
            if let ActionData::FunctionDeclaration(declaration) = &action.data {
                res.push(declaration.clone());
            };
        });
        return res;
    }
}

pub struct SequencesState(pub Mutex<HashMap<usize, Sequence>>);
impl SequencesState {
    pub fn demo() -> Self {
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
                        ActionData::VariableDeclaration(VariableDeclaration::new(
                            "copy".to_string(),
                            Input::from_variable("greeting".to_string()),
                        )),
                        2,
                    ),
                    Action::new(
                        ActionData::FunctionCall(FunctionCall::new(
                            "print".to_string(),
                            vec![Input::from_variable("copy".to_string())],
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

#[tauri::command]
pub fn get_variables_name_in_sequence(
    sequence_id: usize,
    sequences: tauri::State<SequencesState>,
) -> Vec<String> {
    let sequence = sequences.0.lock().unwrap();
    let sequence = sequence.get(&sequence_id).unwrap();
    let mut variables_name: Vec<String> = Vec::new();
    sequence.actions.iter().for_each(|action| {
        if let ActionData::VariableDeclaration(variable_declaration) = &action.data {
            variables_name.push(variable_declaration.name.clone())
        }
    });
    variables_name
}
