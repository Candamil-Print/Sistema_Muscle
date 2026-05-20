use rusqlite::Connection;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::usuarios::usuario::{Usuario, NuevoUsuario, UsuarioModificacion};
// ==============================================
// FUNCIONES LÓGICAS (reutilizables)
// ==============================================

pub fn crear_usuario_logic(conn: &Connection, nuevo: &NuevoUsuario) -> Result<i32, String> {
    let mut stmt = conn.prepare("SELECT 1 FROM usuarios WHERE numero_documento = ?1")
        .map_err(|e| e.to_string())?;
    
    let existe = stmt.exists([&nuevo.numero_documento])
        .map_err(|e| e.to_string())?;
    
    if existe {
        return Err("Ya existe un usuario con este número de documento".to_string());
    }
    
    let hashed = hash(&nuevo.password, DEFAULT_COST)
        .map_err(|e| e.to_string())?;
    
    conn.execute(
        r#"INSERT INTO usuarios (
            nombre_completo, tipo_documento, numero_documento,
            direccion, tipo_sangre, eps, genero,
            correo, telefono, password_hash, id_rol, estado
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 2, 1)"#,
        [
            &nuevo.nombre_completo,
            &nuevo.tipo_documento,
            &nuevo.numero_documento,
            &nuevo.direccion.as_ref().unwrap_or(&"".to_string()),
            &nuevo.tipo_sangre.as_ref().unwrap_or(&"".to_string()),
            &nuevo.eps.as_ref().unwrap_or(&"".to_string()),
            &nuevo.genero.as_ref().unwrap_or(&"".to_string()),
            &nuevo.correo.as_ref().unwrap_or(&"".to_string()),
            &nuevo.telefono.as_ref().unwrap_or(&"".to_string()),
            &hashed,
        ]
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid() as i32)
}

pub fn modificar_usuario_logic(
    conn: &Connection, 
    id: i32, 
    modificacion: &UsuarioModificacion
) -> Result<(), String> {
    let mut updates = Vec::new();
    let mut params: Vec<String> = Vec::new();
    
    if let Some(ref direccion) = modificacion.direccion {
        updates.push("direccion = ?".to_string());
        params.push(direccion.clone());
    }
    if let Some(ref telefono) = modificacion.telefono {
        updates.push("telefono = ?".to_string());
        params.push(telefono.clone());
    }
    if let Some(ref correo) = modificacion.correo {
        updates.push("correo = ?".to_string());
        params.push(correo.clone());
    }
    
    if updates.is_empty() {
        return Ok(());
    }
    
    params.push(id.to_string());
    
    let query = format!(
        "UPDATE usuarios SET {} WHERE id_usuario = ?",
        updates.join(", ")
    );
    
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let params_slice: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
    stmt.execute(rusqlite::params_from_iter(params_slice))
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

pub fn obtener_usuario_logic(conn: &Connection, id: i32) -> Result<Usuario, String> {
    let mut stmt = conn.prepare(
        "SELECT id_usuario, nombre_completo, tipo_documento, numero_documento,
                direccion, tipo_sangre, eps, genero, correo, telefono,
                estado, id_rol, fecha_creacion
         FROM usuarios WHERE id_usuario = ?1"
    ).map_err(|e| e.to_string())?;
    
    let usuario = stmt.query_row([id], |row| {
        Ok(Usuario {
            id_usuario: row.get(0)?,
            nombre_completo: row.get(1)?,
            tipo_documento: row.get(2)?,
            numero_documento: row.get(3)?,
            direccion: row.get(4)?,
            tipo_sangre: row.get(5)?,
            eps: row.get(6)?,
            genero: row.get(7)?,
            correo: row.get(8)?,
            telefono: row.get(9)?,
            estado: row.get(10)?,
            id_rol: row.get(11)?,
            fecha_creacion: row.get(12)?,
        })
    }).map_err(|e| e.to_string())?;
    
    Ok(usuario)
}

pub fn listar_usuarios_logic(conn: &Connection, solo_activos: bool) -> Result<Vec<Usuario>, String> {
    let query = if solo_activos {
        "SELECT id_usuario, nombre_completo, tipo_documento, numero_documento,
                direccion, tipo_sangre, eps, genero, correo, telefono,
                estado, id_rol, fecha_creacion
         FROM usuarios WHERE estado = 1 ORDER BY nombre_completo"
    } else {
        "SELECT id_usuario, nombre_completo, tipo_documento, numero_documento,
                direccion, tipo_sangre, eps, genero, correo, telefono,
                estado, id_rol, fecha_creacion
         FROM usuarios ORDER BY nombre_completo"
    };
    
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(Usuario {
            id_usuario: row.get(0)?,
            nombre_completo: row.get(1)?,
            tipo_documento: row.get(2)?,
            numero_documento: row.get(3)?,
            direccion: row.get(4)?,
            tipo_sangre: row.get(5)?,
            eps: row.get(6)?,
            genero: row.get(7)?,
            correo: row.get(8)?,
            telefono: row.get(9)?,
            estado: row.get(10)?,
            id_rol: row.get(11)?,
            fecha_creacion: row.get(12)?,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut usuarios = Vec::new();
    for usuario in rows {
        usuarios.push(usuario.map_err(|e| e.to_string())?);
    }
    
    Ok(usuarios)
}

pub fn habilitar_usuario_logic(conn: &Connection, id: i32) -> Result<(), String> {
    conn.execute(
        "UPDATE usuarios SET estado = 1 WHERE id_usuario = ?1",
        [id]
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn deshabilitar_usuario_logic(conn: &Connection, id: i32) -> Result<(), String> {
    conn.execute(
        "UPDATE usuarios SET estado = 0 WHERE id_usuario = ?1",
        [id]
    ).map_err(|e| e.to_string())?;
    Ok(())
}