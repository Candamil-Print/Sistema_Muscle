use rusqlite::Connection;
use crate::models::notificaciones::notificacion::{
    Notificacion, ESTADO_ATENDIDA, ESTADO_LEIDA, ESTADO_NO_LEIDA,
};

const QUERY_BASE: &str = r#"SELECT n.id_notificacion, n.id_producto, p.nombre,
                                  n.mensaje, n.stock_actual, n.stock_minimo, n.fecha, n.estado
                           FROM notificaciones n
                           INNER JOIN productos p ON n.id_producto = p.id_producto"#;

fn map_notificacion(row: &rusqlite::Row<'_>) -> rusqlite::Result<Notificacion> {
    Ok(Notificacion {
        id_notificacion: row.get(0)?,
        id_producto: row.get(1)?,
        nombre_producto: row.get(2)?,
        mensaje: row.get(3)?,
        stock_actual: row.get(4)?,
        stock_minimo: row.get(5)?,
        fecha: row.get(6)?,
        estado: row.get(7)?,
    })
}

/// Obtiene una notificación por su id.
pub fn obtener_notificacion_logic(
    conn: &Connection,
    id_notificacion: i32,
) -> Result<Notificacion, String> {
    let mut stmt = conn
        .prepare(&format!("{QUERY_BASE} WHERE n.id_notificacion = ?1"))
        .map_err(|e| e.to_string())?;

    stmt.query_row([id_notificacion], map_notificacion)
        .map_err(|e| e.to_string())
}

/// Lista notificaciones. Si `solo_no_leidas` es true, filtra estado = 0.
/// Las alertas se generan automáticamente con el trigger `trg_notificar_stock_bajo`.
pub fn listar_notificaciones_logic(
    conn: &Connection,
    solo_no_leidas: bool,
) -> Result<Vec<Notificacion>, String> {
    let query = if solo_no_leidas {
        format!("{QUERY_BASE} WHERE n.estado = 0 ORDER BY n.fecha DESC")
    } else {
        format!("{QUERY_BASE} ORDER BY n.fecha DESC")
    };

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], map_notificacion).map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Lista notificaciones de un producto específico.
pub fn notificaciones_por_producto_logic(
    conn: &Connection,
    id_producto: i32,
) -> Result<Vec<Notificacion>, String> {
    let mut stmt = conn
        .prepare(&format!(
            "{QUERY_BASE} WHERE n.id_producto = ?1 ORDER BY n.fecha DESC"
        ))
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([id_producto], map_notificacion)
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Lista notificaciones filtradas por estado (0: no leída, 1: leída, 2: atendida).
pub fn notificaciones_por_estado_logic(
    conn: &Connection,
    estado: i32,
) -> Result<Vec<Notificacion>, String> {
    if !es_estado_valido(estado) {
        return Err("Estado inválido. Use 0 (no leída), 1 (leída) o 2 (atendida)".to_string());
    }

    let mut stmt = conn
        .prepare(&format!(
            "{QUERY_BASE} WHERE n.estado = ?1 ORDER BY n.fecha DESC"
        ))
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([estado], map_notificacion)
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Cuenta las notificaciones no leídas (estado = 0).
pub fn contar_no_leidas_logic(conn: &Connection) -> Result<i32, String> {
    let total: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM notificaciones WHERE estado = 0",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(total)
}

/// Marca una notificación con el estado dado (1: leída, 2: atendida).
pub fn marcar_notificacion_logic(
    conn: &Connection,
    id_notificacion: i32,
    estado: i32,
) -> Result<(), String> {
    if estado != ESTADO_LEIDA && estado != ESTADO_ATENDIDA {
        return Err("Estado inválido. Use 1 (leída) o 2 (atendida)".to_string());
    }

    let filas = conn
        .execute(
            "UPDATE notificaciones SET estado = ?1 WHERE id_notificacion = ?2",
            rusqlite::params![estado, id_notificacion],
        )
        .map_err(|e| e.to_string())?;

    if filas == 0 {
        return Err("La notificación no existe".to_string());
    }

    Ok(())
}

/// Marca todas las notificaciones no leídas como leídas.
pub fn marcar_todas_leidas_logic(conn: &Connection) -> Result<i32, String> {
    let filas = conn
        .execute(
            "UPDATE notificaciones SET estado = ?1 WHERE estado = ?2",
            rusqlite::params![ESTADO_LEIDA, ESTADO_NO_LEIDA],
        )
        .map_err(|e| e.to_string())?;

    Ok(filas as i32)
}

fn es_estado_valido(estado: i32) -> bool {
    estado == ESTADO_NO_LEIDA || estado == ESTADO_LEIDA || estado == ESTADO_ATENDIDA
}
