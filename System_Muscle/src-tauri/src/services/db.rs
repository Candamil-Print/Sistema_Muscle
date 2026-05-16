use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct DbState {
    pub conn: Mutex<Connection>,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("system_muscle.db")?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    Ok(conn)
}

pub fn get_db_connection() -> Result<Connection> {
    let conn = Connection::open("system_muscle.db")?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    Ok(conn)
}