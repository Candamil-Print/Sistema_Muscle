use rusqlite::Connection;
use crate::models::caja::caja::{Caja, CierreCaja, NuevaCaja};

fn map_caja_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Caja> {
    Ok(Caja {
        id_caja: row.get(0)?,
        fecha_apertura: row.get(1)?,
        fecha_cierre: row.get(2)?,
        monto_apertura: row.get(3)?,
        monto_cierre: row.get(4)?,
        total_efectivo: row.get(5)?,
        total_transferencia: row.get(6)?,
        estado: row.get(7)?,
        id_usuario_apertura: row.get(8)?,
        id_usuario_cierre: row.get(9)?,
        id_turno: row.get(10)?,
    })
}

fn validar_usuario_activo(conn: &Connection, id_usuario: i32) -> Result<(), String> {
    let mut stmt = conn
        .prepare("SELECT 1 FROM usuarios WHERE id_usuario = ?1 AND estado = 1")
        .map_err(|e| e.to_string())?;

    if !stmt.exists([id_usuario]).map_err(|e| e.to_string())? {
        return Err("El usuario no existe o está inactivo".to_string());
    }
    Ok(())
}

fn validar_turno_abierto(conn: &Connection, id_turno: i32) -> Result<(), String> {
    let mut stmt = conn
        .prepare("SELECT 1 FROM turnos WHERE id_turno = ?1 AND estado = 'ABIERTO'")
        .map_err(|e| e.to_string())?;

    if !stmt.exists([id_turno]).map_err(|e| e.to_string())? {
        return Err("El turno no existe o no está abierto".to_string());
    }
    Ok(())
}

/// Abre una nueva caja si no hay otra abierta y el turno está activo.
pub fn abrir_caja_logic(conn: &Connection, nueva: &NuevaCaja) -> Result<i32, String> {
    if nueva.monto_apertura < 0.0 {
        return Err("El monto de apertura no puede ser negativo".to_string());
    }

    validar_usuario_activo(conn, nueva.id_usuario_apertura)?;
    validar_turno_abierto(conn, nueva.id_turno)?;

    let mut stmt = conn
        .prepare("SELECT 1 FROM caja WHERE estado = 'ABIERTA'")
        .map_err(|e| e.to_string())?;

    if stmt.exists([]).map_err(|e| e.to_string())? {
        return Err(
            "Ya existe una caja abierta. Debe cerrarla antes de abrir una nueva.".to_string(),
        );
    }

    conn.execute(
        r#"INSERT INTO caja (
            fecha_apertura, monto_apertura, total_efectivo, total_transferencia,
            estado, id_usuario_apertura, id_turno
        ) VALUES (CURRENT_TIMESTAMP, ?1, 0, 0, 'ABIERTA', ?2, ?3)"#,
        rusqlite::params![
            nueva.monto_apertura,
            nueva.id_usuario_apertura,
            nueva.id_turno
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid() as i32)
}

/// Cierra una caja abierta registrando montos y usuario de cierre.
pub fn cerrar_caja_logic(conn: &Connection, cierre: &CierreCaja) -> Result<(), String> {
    validar_usuario_activo(conn, cierre.id_usuario_cierre)?;

    let estado: String = conn
        .query_row(
            "SELECT estado FROM caja WHERE id_caja = ?1",
            [cierre.id_caja],
            |row| row.get(0),
        )
        .map_err(|_| "La caja no está abierta o no existe.".to_string())?;

    if estado != "ABIERTA" {
        return Err("La caja no está abierta o no existe.".to_string());
    }

    conn.execute(
        r#"UPDATE caja SET
            fecha_cierre = CURRENT_TIMESTAMP,
            monto_cierre = ?1,
            total_efectivo = ?2,
            total_transferencia = ?3,
            estado = 'CERRADA',
            id_usuario_cierre = ?4
        WHERE id_caja = ?5"#,
        rusqlite::params![
            cierre.monto_cierre,
            cierre.total_efectivo,
            cierre.total_transferencia,
            cierre.id_usuario_cierre,
            cierre.id_caja
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Obtiene una caja por su id.
pub fn obtener_caja_logic(conn: &Connection, id: i32) -> Result<Caja, String> {
    conn.query_row(
        r#"SELECT id_caja, fecha_apertura, fecha_cierre, monto_apertura, monto_cierre,
                  total_efectivo, total_transferencia, estado, id_usuario_apertura,
                  id_usuario_cierre, id_turno
           FROM caja WHERE id_caja = ?1"#,
        [id],
        map_caja_row,
    )
    .map_err(|e| e.to_string())
}

/// Devuelve la caja actualmente abierta, si existe.
pub fn obtener_caja_activa_logic(conn: &Connection) -> Result<Option<Caja>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT id_caja, fecha_apertura, fecha_cierre, monto_apertura, monto_cierre,
                      total_efectivo, total_transferencia, estado, id_usuario_apertura,
                      id_usuario_cierre, id_turno
               FROM caja WHERE estado = 'ABIERTA' LIMIT 1"#,
        )
        .map_err(|e| e.to_string())?;

    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;

    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(Some(map_caja_row(&row).map_err(|e| e.to_string())?))
    } else {
        Ok(None)
    }
}

/// Lista cajas; con `solo_abiertas` filtra por estado ABIERTA.
pub fn listar_cajas_logic(conn: &Connection, solo_abiertas: bool) -> Result<Vec<Caja>, String> {
    let query = if solo_abiertas {
        r#"SELECT id_caja, fecha_apertura, fecha_cierre, monto_apertura, monto_cierre,
                  total_efectivo, total_transferencia, estado, id_usuario_apertura,
                  id_usuario_cierre, id_turno
           FROM caja WHERE estado = 'ABIERTA' ORDER BY fecha_apertura DESC"#
    } else {
        r#"SELECT id_caja, fecha_apertura, fecha_cierre, monto_apertura, monto_cierre,
                  total_efectivo, total_transferencia, estado, id_usuario_apertura,
                  id_usuario_cierre, id_turno
           FROM caja ORDER BY fecha_apertura DESC"#
    };

    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], map_caja_row)
        .map_err(|e| e.to_string())?;

    let mut cajas = Vec::new();
    for caja in rows {
        cajas.push(caja.map_err(|e| e.to_string())?);
    }

    Ok(cajas)
}
