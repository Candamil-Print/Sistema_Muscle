// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod models;
pub mod services;  // ← DEBE estar exactamente así

use services::db::DbState;  // ← Así se importa desde la carpeta services
use std::sync::Mutex;

pub use services::db::get_db_connection;
pub use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = match services::db::init_db() {
        Ok(c) => {
            println!("✅ Base de datos conectada");
            c
        }
        Err(e) => {
            eprintln!("❌ Error al conectar a la BD: {}", e);
            std::process::exit(1);
        }
    };
    
    tauri::Builder::default()
        .manage(DbState {
            conn: Mutex::new(conn),
        })
        .invoke_handler(tauri::generate_handler![
            commands::test_db_connection,
            commands::crear_usuario,
            commands::modificar_usuario,
            commands::obtener_usuario,
            commands::listar_usuarios,
            commands::habilitar_usuario,
            commands::deshabilitar_usuario,
            commands::login,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}