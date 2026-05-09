use rusqlite::Connection;

#[tauri::command]
pub fn test_db_connection() -> Result<String, String> {
    // Ruta del archivo .db (debe estar en src-tauri/)
    let db_path = "system_muscle.db";
    
    // Conectar a la base de datos
    match Connection::open(db_path) {
        Ok(conn) => {
            // Consulta de prueba
            match conn.query_row("SELECT 'Conectado exitosamente'", [], |row| {
                row.get::<_, String>(0)
            }) {
                Ok(mensaje) => {
                    println!("✅ {}", mensaje);
                    Ok(mensaje)
                }
                Err(e) => Err(format!("❌ Error en consulta: {}", e)),
            }
        }
        Err(e) => Err(format!("❌ Error al abrir DB: {}", e)),
    }
}