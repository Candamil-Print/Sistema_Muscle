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
            println!("Base de datos conectada");
            c
        }
        Err(e) => {
            eprintln!("Error al conectar a la BD: {}", e);
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
            // Stock
            commands::obtener_stock_por_producto,
            commands::listar_stock,
            commands::ajustar_stock,
            commands::listar_stock_bajo,
            commands::listar_notificaciones,
            commands::marcar_notificacion,
            // Movimientos de entrada
            commands::registrar_entrada,
            commands::obtener_movimiento,
            commands::listar_movimientos_entrada,
            commands::movimientos_por_producto,
            commands::movimientos_por_usuario,
            commands::movimientos_por_rango_fechas,
            commands::total_entradas_por_producto,
            // Ventas
            commands::registrar_venta,
            commands::obtener_venta,
            commands::listar_detalle_venta,
            commands::listar_ventas,
            commands::ventas_por_usuario,
            commands::ventas_por_caja,
            commands::ventas_por_rango_fechas,
            commands::resumen_ventas_diario,
            commands::total_ventas_por_producto,
            // Caja
            commands::abrir_caja,
            commands::cerrar_caja,
            commands::obtener_caja,
            commands::obtener_caja_activa,
            commands::listar_cajas,
            // Historial
            commands::registrar_accion,
            commands::listar_historial,
            commands::obtener_accion,
            commands::historial_por_usuario,
            commands::historial_por_turno,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}