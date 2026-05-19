// ==============================================
// MÓDULO USUARIOS
// ==============================================
pub mod usuarios;

pub use usuarios::logic::{
    crear_usuario_logic,
    modificar_usuario_logic,
    obtener_usuario_logic,
    listar_usuarios_logic,
    habilitar_usuario_logic,
    deshabilitar_usuario_logic,
    login_logic,
};

// ==============================================
// MÓDULO PRODUCTOS
// ==============================================
pub mod productos;

pub use productos::logic::{
    crear_producto_logic,
    modificar_producto_logic,
    obtener_producto_logic,
    obtener_producto_simple_logic,
    listar_productos_logic,
    buscar_productos_logic,
    activar_producto_logic,
    desactivar_producto_logic,
};

// ==============================================
// Comandos de Tauri
// ==============================================
use tauri::State;
use crate::services::db::connection::DbState;
use crate::models::usuarios::usuario::{NuevoUsuario, UsuarioModificacion};
use crate::models::productos::producto::{NuevoProducto, ModificarProducto};

// ----- Comandos de utilidad -----

#[tauri::command]
pub fn test_db_connection() -> Result<String, String> {
    match rusqlite::Connection::open("system_muscle.db") {
        Ok(conn) => {
            match conn.query_row("SELECT 'Conectado exitosamente'", [], |row| {
                row.get::<_, String>(0)
            }) {
                Ok(mensaje) => Ok(mensaje),
                Err(e) => Err(format!("Error en consulta: {}", e)),
            }
        }
        Err(e) => Err(format!("Error al abrir DB: {}", e)),
    }
}

// ----- Comandos de Usuarios -----

#[tauri::command]
pub fn crear_usuario(
    state: State<'_, DbState>,
    nuevo: NuevoUsuario,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    crear_usuario_logic(&conn, &nuevo)
}

#[tauri::command]
pub fn modificar_usuario(
    state: State<'_, DbState>,
    id: i32,
    modificacion: UsuarioModificacion,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    modificar_usuario_logic(&conn, id, &modificacion)
}

#[tauri::command]
pub fn obtener_usuario(
    state: State<'_, DbState>,
    id: i32,
) -> Result<crate::models::usuarios::usuario::Usuario, String> {
    let conn = state.conn.lock().unwrap();
    obtener_usuario_logic(&conn, id)
}

#[tauri::command]
pub fn listar_usuarios(
    state: State<'_, DbState>,
    solo_activos: bool,
) -> Result<Vec<crate::models::usuarios::usuario::Usuario>, String> {
    let conn = state.conn.lock().unwrap();
    listar_usuarios_logic(&conn, solo_activos)
}

#[tauri::command]
pub fn habilitar_usuario(
    state: State<'_, DbState>,
    id: i32,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    habilitar_usuario_logic(&conn, id)
}

#[tauri::command]
pub fn deshabilitar_usuario(
    state: State<'_, DbState>,
    id: i32,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    deshabilitar_usuario_logic(&conn, id)
}

#[tauri::command]
pub fn login(
    state: State<'_, DbState>,
    documento: String,
    password: String,
) -> Result<Option<crate::models::usuarios::usuario::Usuario>, String> {
    let conn = state.conn.lock().unwrap();
    login_logic(&conn, &documento, &password)
}

// ----- Comandos de Productos -----

#[tauri::command]
pub fn crear_producto(
    state: State<'_, DbState>,
    nuevo: NuevoProducto,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    crear_producto_logic(&conn, &nuevo)
}

#[tauri::command]
pub fn modificar_producto(
    state: State<'_, DbState>,
    id: i32,
    datos: ModificarProducto,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    modificar_producto_logic(&conn, id, &datos)
}

#[tauri::command]
pub fn obtener_producto(
    state: State<'_, DbState>,
    id: i32,
) -> Result<crate::models::productos::producto::ProductoConStock, String> {
    let conn = state.conn.lock().unwrap();
    obtener_producto_logic(&conn, id)
}

#[tauri::command]
pub fn listar_productos(
    state: State<'_, DbState>,
    solo_activos: bool,
) -> Result<Vec<crate::models::productos::producto::ProductoConStock>, String> {
    let conn = state.conn.lock().unwrap();
    listar_productos_logic(&conn, solo_activos)
}

#[tauri::command]
pub fn buscar_productos(
    state: State<'_, DbState>,
    termino: String,
    solo_activos: bool,
) -> Result<Vec<crate::models::productos::producto::ProductoConStock>, String> {
    let conn = state.conn.lock().unwrap();
    buscar_productos_logic(&conn, &termino, solo_activos)
}

#[tauri::command]
pub fn activar_producto(
    state: State<'_, DbState>,
    id: i32,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    activar_producto_logic(&conn, id)
}

#[tauri::command]
pub fn desactivar_producto(
    state: State<'_, DbState>,
    id: i32,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    desactivar_producto_logic(&conn, id)
}