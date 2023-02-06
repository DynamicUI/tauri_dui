#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate core;

mod action;
mod builtins;
mod data;
mod executor;
mod sequence;

use builtins::*;
use sequence::*;
use tauri::Manager;

fn main() {
    print!("Hello, world! {}\n", JSON_BUILTINS_FUNCTIONS);
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // n'incluez ce code que sur les versions de débogage
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(SequencesState::demo())
        .invoke_handler(tauri::generate_handler![
            get_sequences,
            add_sequence,
            get_sequence_by_id,
            get_builtins_functions_list,
            get_variables_name_in_sequence,
            run
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn run(sequences: tauri::State<SequencesState>) {
    let main_sequence = sequences.0.lock().unwrap();
    let main_sequence = main_sequence.get(&0).unwrap();
    executor::execute_sequence(main_sequence);
}
