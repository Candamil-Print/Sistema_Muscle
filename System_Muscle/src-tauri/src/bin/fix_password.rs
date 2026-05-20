use rusqlite::Connection;

fn main() {
    let conn = Connection::open("system_muscle.db").unwrap();
    let hash = "$2b$12$pjnBdb4mSbkf1k64BliQvecJ2W5KFbnYnwNOzwCqI2ho5K0K9LRbu";
    
    conn.execute(
        "UPDATE usuarios SET password_hash = ?1 WHERE id_usuario = 1",
        [hash]
    ).unwrap();
    
    println!("✅ Contraseña actualizada");
    
    // Verificar
    let mut stmt = conn.prepare("SELECT password_hash FROM usuarios WHERE id_usuario = 1").unwrap();
    let result: String = stmt.query_row([], |row| row.get(0)).unwrap();
    println!("Hash en BD: {}", result);
}