use bcrypt::verify;
use rusqlite::Connection;
use crate::models::login::login::{CredencialesLogin, SesionUsuario};

const QUERY_SESION: &str = r#"SELECT u.id_usuario, u.nombre_completo, u.tipo_documento, u.numero_documento,
                                    u.correo, u.telefono, u.id_rol, r.nombre, u.estado, u.password_hash
                             FROM usuarios u
                             INNER JOIN roles r ON u.id_rol = r.id_rol"#;

fn map_sesion_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SesionUsuario> {
    Ok(SesionUsuario {
        id_usuario: row.get(0)?,
        nombre_completo: row.get(1)?,
        tipo_documento: row.get(2)?,
        numero_documento: row.get(3)?,
        correo: row.get(4)?,
        telefono: row.get(5)?,
        id_rol: row.get(6)?,
        nombre_rol: row.get(7)?,
        estado: row.get(8)?,
    })
}

fn validar_credenciales(credenciales: &CredencialesLogin) -> Result<(String, String), String> {
    let documento = credenciales.documento.trim().to_string();
    let password = credenciales.password.clone();

    if documento.is_empty() {
        return Err("El número de documento es obligatorio".to_string());
    }
    if password.is_empty() {
        return Err("La contraseña es obligatoria".to_string());
    }

    Ok((documento, password))
}

/// Autentica un usuario activo por documento y contraseña.
/// Devuelve `None` si las credenciales no son válidas (sin revelar el motivo).
pub fn login_logic(
    conn: &Connection,
    credenciales: &CredencialesLogin,
) -> Result<Option<SesionUsuario>, String> {
    let (documento, password) = validar_credenciales(credenciales)?;

    let (hash, sesion_base): (String, SesionUsuario) = match conn.query_row(
        &format!("{QUERY_SESION} WHERE u.numero_documento = ?1 AND u.estado = 1"),
        [&documento],
        |row| {
            Ok((
                row.get::<_, String>(9)?,
                SesionUsuario {
                    id_usuario: row.get(0)?,
                    nombre_completo: row.get(1)?,
                    tipo_documento: row.get(2)?,
                    numero_documento: row.get(3)?,
                    correo: row.get(4)?,
                    telefono: row.get(5)?,
                    id_rol: row.get(6)?,
                    nombre_rol: row.get(7)?,
                    estado: row.get(8)?,
                },
            ))
        },
    ) {
        Ok(data) => data,
        Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
        Err(e) => return Err(e.to_string()),
    };

    if verify(&password, &hash).map_err(|e| e.to_string())? {
        Ok(Some(sesion_base))
    } else {
        Ok(None)
    }
}

/// Verifica que un usuario siga activo y devuelve su sesión actualizada.
pub fn verificar_sesion_logic(
    conn: &Connection,
    id_usuario: i32,
) -> Result<Option<SesionUsuario>, String> {
    match conn.query_row(
        &format!("{QUERY_SESION} WHERE u.id_usuario = ?1 AND u.estado = 1"),
        [id_usuario],
        map_sesion_row,
    ) {
        Ok(sesion) => Ok(Some(sesion)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}
