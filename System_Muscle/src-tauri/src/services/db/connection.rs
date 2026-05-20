use rusqlite::{Connection, Result};
use std::sync::Mutex;

// Estructura para compartir la conexión a la base de datos entre componentes
pub struct DbState {
    pub conn: Mutex<Connection>,
}

// Inicializa la conexión a la base de datos
pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("system_muscle.db")?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    Ok(conn)
}

// Obtiene una conexión a la base de datos
pub fn get_db_connection() -> Result<Connection> {
    let conn = Connection::open("system_muscle.db")?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    Ok(conn)
}