// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod models;
pub mod services;

use services::db::connection::DbState;
use std::sync::Mutex;

// Exportar para tests
pub use services::db::connection::get_db_connection;
pub use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = match services::db::connection::init_db() {
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
            // Utilidades
            commands::test_db_connection,
            // Usuarios
            commands::crear_usuario,
            commands::modificar_usuario,
            commands::obtener_usuario,
            commands::listar_usuarios,
            commands::habilitar_usuario,
            commands::deshabilitar_usuario,
            commands::login,
            // Productos
            commands::crear_producto,
            commands::modificar_producto,
            commands::obtener_producto,
            commands::listar_productos,
            commands::buscar_productos,
            commands::activar_producto,
            commands::desactivar_producto,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}