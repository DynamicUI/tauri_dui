#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod sequence;
mod action;
use sequence::*;
use tauri::Manager;

fn main() {
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
        .manage(SequencesState::default())
        .invoke_handler(tauri::generate_handler![
            get_sequences,
            add_sequence,
            get_sequence_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

