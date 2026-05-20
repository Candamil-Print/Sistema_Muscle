use rusqlite::Connection;
use crate::models::historial::historial::{
    FiltroHistorial, HistorialAccion, HistorialAccionDetalle, NuevaAccion,
};

fn map_historial_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<HistorialAccion> {
    Ok(HistorialAccion {
        id_historial: row.get(0)?,
        id_usuario: row.get(1)?,
        accion: row.get(2)?,
        tabla_afectada: row.get(3)?,
        id_registro_afectado: row.get(4)?,
        descripcion: row.get(5)?,
        fecha: row.get(6)?,
        hora: row.get(7)?,
        id_turno: row.get(8)?,
    })
}

fn map_historial_detalle_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<HistorialAccionDetalle> {
    Ok(HistorialAccionDetalle {
        id_historial: row.get(0)?,
        usuario: row.get(1)?,
        accion: row.get(2)?,
        tabla_afectada: row.get(3)?,
        id_registro_afectado: row.get(4)?,
        descripcion: row.get(5)?,
        fecha: row.get(6)?,
        hora: row.get(7)?,
        turno: row.get(8)?,
    })
}

pub fn registrar_accion_logic(conn: &Connection, accion: &NuevaAccion) -> Result<i32, String> {
    conn.execute(
        r#"INSERT INTO historial_acciones (
            id_usuario, accion, tabla_afectada, id_registro_afectado,
            descripcion, fecha, hora, id_turno
        ) VALUES (?1, ?2, ?3, ?4, ?5, DATE('now'), TIME('now'), ?6)"#,
        rusqlite::params![
            accion.id_usuario,
            accion.accion,
            accion.tabla_afectada.as_deref().unwrap_or(""),
            accion.id_registro_afectado.unwrap_or(0),
            accion.descripcion.as_deref().unwrap_or(""),
            accion.id_turno.unwrap_or(0),
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid() as i32)
}

pub fn obtener_accion_logic(conn: &Connection, id: i32) -> Result<HistorialAccion, String> {
    conn.query_row(
        r#"SELECT id_historial, id_usuario, accion, tabla_afectada, id_registro_afectado,
                  descripcion, fecha, hora, id_turno
           FROM historial_acciones WHERE id_historial = ?1"#,
        [id],
        map_historial_row,
    )
    .map_err(|e| e.to_string())
}

pub fn listar_historial_logic(
    conn: &Connection,
    filtro: &FiltroHistorial,
) -> Result<Vec<HistorialAccion>, String> {
    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(id_usuario) = filtro.id_usuario {
        conditions.push("id_usuario = ?".to_string());
        params.push(id_usuario.to_string());
    }
    if let Some(ref accion) = filtro.accion {
        conditions.push("accion = ?".to_string());
        params.push(accion.clone());
    }
    if let Some(ref fecha_desde) = filtro.fecha_desde {
        conditions.push("fecha >= ?".to_string());
        params.push(fecha_desde.clone());
    }
    if let Some(ref fecha_hasta) = filtro.fecha_hasta {
        conditions.push("fecha <= ?".to_string());
        params.push(fecha_hasta.clone());
    }
    if let Some(id_turno) = filtro.id_turno {
        conditions.push("id_turno = ?".to_string());
        params.push(id_turno.to_string());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let query = format!(
        r#"SELECT id_historial, id_usuario, accion, tabla_afectada, id_registro_afectado,
                  descripcion, fecha, hora, id_turno
           FROM historial_acciones
           {where_clause}
           ORDER BY fecha DESC, hora DESC"#
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let params_slice: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
    let rows = stmt
        .query_map(rusqlite::params_from_iter(params_slice), map_historial_row)
        .map_err(|e| e.to_string())?;

    let mut historial = Vec::new();
    for item in rows {
        historial.push(item.map_err(|e| e.to_string())?);
    }

    Ok(historial)
}

pub fn listar_historial_detalle_logic(
    conn: &Connection,
    filtro: &FiltroHistorial,
) -> Result<Vec<HistorialAccionDetalle>, String> {
    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(id_usuario) = filtro.id_usuario {
        conditions.push("h.id_usuario = ?".to_string());
        params.push(id_usuario.to_string());
    }
    if let Some(ref accion) = filtro.accion {
        conditions.push("h.accion = ?".to_string());
        params.push(accion.clone());
    }
    if let Some(ref fecha_desde) = filtro.fecha_desde {
        conditions.push("h.fecha >= ?".to_string());
        params.push(fecha_desde.clone());
    }
    if let Some(ref fecha_hasta) = filtro.fecha_hasta {
        conditions.push("h.fecha <= ?".to_string());
        params.push(fecha_hasta.clone());
    }
    if let Some(id_turno) = filtro.id_turno {
        conditions.push("h.id_turno = ?".to_string());
        params.push(id_turno.to_string());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let query = format!(
        r#"SELECT
            h.id_historial,
            u.nombre_completo as usuario,
            h.accion,
            h.tabla_afectada,
            h.id_registro_afectado,
            h.descripcion,
            h.fecha,
            h.hora,
            tt.nombre as turno
         FROM historial_acciones h
         LEFT JOIN usuarios u ON h.id_usuario = u.id_usuario
         LEFT JOIN turnos t ON h.id_turno = t.id_turno
         LEFT JOIN tipos_turno tt ON t.id_tipo_turno = tt.id_tipo_turno
         {where_clause}
         ORDER BY h.fecha DESC, h.hora DESC"#
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let params_slice: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
    let rows = stmt
        .query_map(rusqlite::params_from_iter(params_slice), map_historial_detalle_row)
        .map_err(|e| e.to_string())?;

    let mut historial = Vec::new();
    for item in rows {
        historial.push(item.map_err(|e| e.to_string())?);
    }

    Ok(historial)
}

pub fn historial_por_usuario_logic(
    conn: &Connection,
    id_usuario: i32,
) -> Result<Vec<HistorialAccion>, String> {
    let filtro = FiltroHistorial {
        id_usuario: Some(id_usuario),
        accion: None,
        fecha_desde: None,
        fecha_hasta: None,
        id_turno: None,
    };
    listar_historial_logic(conn, &filtro)
}

pub fn historial_por_turno_logic(
    conn: &Connection,
    id_turno: i32,
) -> Result<Vec<HistorialAccion>, String> {
    let filtro = FiltroHistorial {
        id_usuario: None,
        accion: None,
        fecha_desde: None,
        fecha_hasta: None,
        id_turno: Some(id_turno),
    };
    listar_historial_logic(conn, &filtro)
}

pub fn historial_por_accion_logic(
    conn: &Connection,
    accion: &str,
) -> Result<Vec<HistorialAccion>, String> {
    let filtro = FiltroHistorial {
        id_usuario: None,
        accion: Some(accion.to_string()),
        fecha_desde: None,
        fecha_hasta: None,
        id_turno: None,
    };
    listar_historial_logic(conn, &filtro)
}

pub fn historial_por_rango_fechas_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<HistorialAccion>, String> {
    let filtro = FiltroHistorial {
        id_usuario: None,
        accion: None,
        fecha_desde: Some(fecha_inicio.to_string()),
        fecha_hasta: Some(fecha_fin.to_string()),
        id_turno: None,
    };
    listar_historial_logic(conn, &filtro)
}

pub fn ultimos_historial_logic(
    conn: &Connection,
    limite: i32,
) -> Result<Vec<HistorialAccionDetalle>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT
            h.id_historial,
            u.nombre_completo as usuario,
            h.accion,
            h.tabla_afectada,
            h.id_registro_afectado,
            h.descripcion,
            h.fecha,
            h.hora,
            tt.nombre as turno
        FROM historial_acciones h
        LEFT JOIN usuarios u ON h.id_usuario = u.id_usuario
        LEFT JOIN turnos t ON h.id_turno = t.id_turno
        LEFT JOIN tipos_turno tt ON t.id_tipo_turno = tt.id_tipo_turno
        ORDER BY h.fecha DESC, h.hora DESC
        LIMIT ?1"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([limite], map_historial_detalle_row)
        .map_err(|e| e.to_string())?;

    let mut historial = Vec::new();
    for item in rows {
        historial.push(item.map_err(|e| e.to_string())?);
    }

    Ok(historial)
}
