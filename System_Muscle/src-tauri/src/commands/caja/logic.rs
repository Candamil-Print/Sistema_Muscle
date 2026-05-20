use rusqlite::Connection;
use crate::models::caja::caja::{Caja, NuevaCaja, CierreCaja};

// ==============================================
// FUNCIONES LÓGICAS
// ==============================================

pub fn abrir_caja_logic(conn: &Connection, nueva: &NuevaCaja) -> Result<i32, String> {
    let mut stmt = conn.prepare("SELECT 1 FROM caja WHERE estado = 'ABIERTA'")
        .map_err(|e| e.to_string())?;
    let existe_abierta = stmt.exists([]).map_err(|e| e.to_string())?;
    
    if existe_abierta {
        return Err("Ya existe una caja abierta. Debe cerrarla antes de abrir una nueva.".to_string());
    }
    
    let mut stmt = conn.prepare("SELECT 1 FROM turnos WHERE id_turno = ?1 AND estado = 'ABIERTO'")
        .map_err(|e| e.to_string())?;
    let turno_valido = stmt.exists([nueva.id_turno]).map_err(|e| e.to_string())?;
    
    if !turno_valido {
        return Err("El turno especificado no existe o no está abierto.".to_string());
    }
    
    conn.execute(
        r#"INSERT INTO caja (
            fecha_apertura, monto_apertura, total_efectivo, total_transferencia, 
            estado, id_usuario_apertura, id_turno
        ) VALUES (CURRENT_TIMESTAMP, ?1, 0, 0, 'ABIERTA', ?2, ?3)"#,
        [&nueva.monto_apertura, &nueva.id_usuario_apertura, &nueva.id_turno]
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid() as i32)
}

pub fn cerrar_caja_logic(conn: &Connection, cierre: &CierreCaja) -> Result<(), String> {
    let mut stmt = conn.prepare("SELECT estado FROM caja WHERE id_caja = ?1")
        .map_err(|e| e.to_string())?;
    
    let estado: String = stmt.query_row([cierre.id_caja], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
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
        [&cierre.monto_cierre, &cierre.total_efectivo, &cierre.total_transferencia, 
         &cierre.id_usuario_cierre, &cierre.id_caja]
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

pub fn obtener_caja_logic(conn: &Connection, id: i32) -> Result<Caja, String> {
    let mut stmt = conn.prepare(
        "SELECT id_caja, fecha_apertura, fecha_cierre, monto_apertura, monto_cierre,
                total_efectivo, total_transferencia, estado, id_usuario_apertura, 
                id_usuario_cierre, id_turno
         FROM caja WHERE id_caja = ?1"
    ).map_err(|e| e.to_string())?;
    
    let caja = stmt.query_row([id], |row| {
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
    }).map_err(|e| e.to_string())?;
    
    Ok(caja)
}

pub fn obtener_caja_activa_logic(conn: &Connection) -> Result<Option<Caja>, String> {
    let mut stmt = conn.prepare(
        "SELECT id_caja, fecha_apertura, fecha_cierre, monto_apertura, monto_cierre,
                total_efectivo, total_transferencia, estado, id_usuario_apertura, 
                id_usuario_cierre, id_turno
         FROM caja WHERE estado = 'ABIERTA' LIMIT 1"
    ).map_err(|e| e.to_string())?;
    
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(Some(Caja {
            id_caja: row.get(0).map_err(|e| e.to_string())?,
            fecha_apertura: row.get(1).map_err(|e| e.to_string())?,
            fecha_cierre: row.get(2).map_err(|e| e.to_string())?,
            monto_apertura: row.get(3).map_err(|e| e.to_string())?,
            monto_cierre: row.get(4).map_err(|e| e.to_string())?,
            total_efectivo: row.get(5).map_err(|e| e.to_string())?,
            total_transferencia: row.get(6).map_err(|e| e.to_string())?,
            estado: row.get(7).map_err(|e| e.to_string())?,
            id_usuario_apertura: row.get(8).map_err(|e| e.to_string())?,
            id_usuario_cierre: row.get(9).map_err(|e| e.to_string())?,
            id_turno: row.get(10).map_err(|e| e.to_string())?,
        }))
    } else {
        Ok(None)
    }
}

pub fn listar_cajas_logic(conn: &Connection, solo_abiertas: bool) -> Result<Vec<Caja>, String> {
    let query = if solo_abiertas {
        "SELECT id_caja, fecha_apertura, fecha_cierre, monto_apertura, monto_cierre,
                total_efectivo, total_transferencia, estado, id_usuario_apertura, 
                id_usuario_cierre, id_turno
         FROM caja WHERE estado = 'ABIERTA' ORDER BY fecha_apertura DESC"
    } else {
        "SELECT id_caja, fecha_apertura, fecha_cierre, monto_apertura, monto_cierre,
                total_efectivo, total_transferencia, estado, id_usuario_apertura, 
                id_usuario_cierre, id_turno
         FROM caja ORDER BY fecha_apertura DESC"
    };
    
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(Caja {
            id_caja: row.get(0).map_err(|e| e.to_string())?,
            fecha_apertura: row.get(1).map_err(|e| e.to_string())?,
            fecha_cierre: row.get(2).map_err(|e| e.to_string())?,
            monto_apertura: row.get(3).map_err(|e| e.to_string())?,
            monto_cierre: row.get(4).map_err(|e| e.to_string())?,
            total_efectivo: row.get(5).map_err(|e| e.to_string())?,
            total_transferencia: row.get(6).map_err(|e| e.to_string())?,
            estado: row.get(7).map_err(|e| e.to_string())?,
            id_usuario_apertura: row.get(8).map_err(|e| e.to_string())?,
            id_usuario_cierre: row.get(9).map_err(|e| e.to_string())?,
            id_turno: row.get(10).map_err(|e| e.to_string())?,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut cajas = Vec::new();
    for caja in rows {
        cajas.push(caja.map_err(|e| e.to_string())?);
    }
    
    Ok(cajas)
}