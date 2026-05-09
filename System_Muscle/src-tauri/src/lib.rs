// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;

use tauri_plugin_sql::Builder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(Builder::default().build())
        .setup(|_app| {
            // Ejecutar prueba de conexión al inicio
            match commands::test_db_connection() {
                Ok(msg) => println!("{}", msg),
                Err(e) => println!("{}", e),
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::test_db_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}