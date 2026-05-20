use rusqlite::Connection;
use crate::models::movimientos_entrada::movimiento_entrada::{
    MovimientoEntrada, MovimientoEntradaDetalle, NuevoMovimientoEntrada,
};

/// Registra un movimiento de entrada de mercancía.
/// El trigger `trg_actualizar_stock_entrada` suma automáticamente la cantidad al stock.
/// El trigger `trg_historial_entrada` registra la acción en historial_acciones.
pub fn registrar_entrada_logic(
    conn: &Connection,
    entrada: &NuevoMovimientoEntrada,
) -> Result<i32, String> {
    // Verificar que el producto existe y está activo
    let mut stmt = conn
        .prepare("SELECT 1 FROM productos WHERE id_producto = ?1 AND activo = 1")
        .map_err(|e| e.to_string())?;

    let existe_producto = stmt
        .exists([entrada.id_producto])
        .map_err(|e| e.to_string())?;

    if !existe_producto {
        return Err("El producto no existe o está inactivo".to_string());
    }

    // Verificar que el usuario existe y está activo
    let mut stmt_u = conn
        .prepare("SELECT 1 FROM usuarios WHERE id_usuario = ?1 AND estado = 1")
        .map_err(|e| e.to_string())?;

    let existe_usuario = stmt_u
        .exists([entrada.id_usuario])
        .map_err(|e| e.to_string())?;

    if !existe_usuario {
        return Err("El usuario no existe o está inactivo".to_string());
    }

    // Validar cantidad positiva
    if entrada.cantidad <= 0 {
        return Err("La cantidad debe ser mayor a cero".to_string());
    }

    conn.execute(
        r#"INSERT INTO movimientos_entrada (id_producto, cantidad, id_usuario)
           VALUES (?1, ?2, ?3)"#,
        rusqlite::params![entrada.id_producto, entrada.cantidad, entrada.id_usuario],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid() as i32)
}

/// Obtiene un movimiento de entrada por su id_movimiento.
pub fn obtener_movimiento_logic(conn: &Connection, id: i32) -> Result<MovimientoEntrada, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT id_movimiento, id_producto, cantidad, fecha, id_usuario
               FROM movimientos_entrada WHERE id_movimiento = ?1"#,
        )
        .map_err(|e| e.to_string())?;

    let movimiento = stmt
        .query_row([id], |row| {
            Ok(MovimientoEntrada {
                id_movimiento: row.get(0)?,
                id_producto: row.get(1)?,
                cantidad: row.get(2)?,
                fecha: row.get(3)?,
                id_usuario: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(movimiento)
}

/// Lista todos los movimientos de entrada con detalles del producto y usuario.
pub fn listar_movimientos_entrada_logic(
    conn: &Connection,
) -> Result<Vec<MovimientoEntradaDetalle>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT me.id_movimiento, me.id_producto, p.nombre, p.tipo_producto,
                      me.cantidad, me.fecha, me.id_usuario, u.nombre_completo
               FROM movimientos_entrada me
               INNER JOIN productos p ON me.id_producto = p.id_producto
               INNER JOIN usuarios u ON me.id_usuario = u.id_usuario
               ORDER BY me.fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(MovimientoEntradaDetalle {
                id_movimiento: row.get(0)?,
                id_producto: row.get(1)?,
                nombre_producto: row.get(2)?,
                tipo_producto: row.get(3)?,
                cantidad: row.get(4)?,
                fecha: row.get(5)?,
                id_usuario: row.get(6)?,
                nombre_usuario: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }

    Ok(lista)
}

/// Lista los movimientos de entrada de un producto específico.
pub fn movimientos_por_producto_logic(
    conn: &Connection,
    id_producto: i32,
) -> Result<Vec<MovimientoEntradaDetalle>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT me.id_movimiento, me.id_producto, p.nombre, p.tipo_producto,
                      me.cantidad, me.fecha, me.id_usuario, u.nombre_completo
               FROM movimientos_entrada me
               INNER JOIN productos p ON me.id_producto = p.id_producto
               INNER JOIN usuarios u ON me.id_usuario = u.id_usuario
               WHERE me.id_producto = ?1
               ORDER BY me.fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([id_producto], |row| {
            Ok(MovimientoEntradaDetalle {
                id_movimiento: row.get(0)?,
                id_producto: row.get(1)?,
                nombre_producto: row.get(2)?,
                tipo_producto: row.get(3)?,
                cantidad: row.get(4)?,
                fecha: row.get(5)?,
                id_usuario: row.get(6)?,
                nombre_usuario: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }

    Ok(lista)
}

/// Lista los movimientos registrados por un usuario específico.
pub fn movimientos_por_usuario_logic(
    conn: &Connection,
    id_usuario: i32,
) -> Result<Vec<MovimientoEntradaDetalle>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT me.id_movimiento, me.id_producto, p.nombre, p.tipo_producto,
                      me.cantidad, me.fecha, me.id_usuario, u.nombre_completo
               FROM movimientos_entrada me
               INNER JOIN productos p ON me.id_producto = p.id_producto
               INNER JOIN usuarios u ON me.id_usuario = u.id_usuario
               WHERE me.id_usuario = ?1
               ORDER BY me.fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([id_usuario], |row| {
            Ok(MovimientoEntradaDetalle {
                id_movimiento: row.get(0)?,
                id_producto: row.get(1)?,
                nombre_producto: row.get(2)?,
                tipo_producto: row.get(3)?,
                cantidad: row.get(4)?,
                fecha: row.get(5)?,
                id_usuario: row.get(6)?,
                nombre_usuario: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }

    Ok(lista)
}

/// Lista los movimientos de entrada dentro de un rango de fechas.
/// Las fechas deben estar en formato 'YYYY-MM-DD'.
pub fn movimientos_por_rango_fechas_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<MovimientoEntradaDetalle>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT me.id_movimiento, me.id_producto, p.nombre, p.tipo_producto,
                      me.cantidad, me.fecha, me.id_usuario, u.nombre_completo
               FROM movimientos_entrada me
               INNER JOIN productos p ON me.id_producto = p.id_producto
               INNER JOIN usuarios u ON me.id_usuario = u.id_usuario
               WHERE DATE(me.fecha) BETWEEN DATE(?1) AND DATE(?2)
               ORDER BY me.fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
            Ok(MovimientoEntradaDetalle {
                id_movimiento: row.get(0)?,
                id_producto: row.get(1)?,
                nombre_producto: row.get(2)?,
                tipo_producto: row.get(3)?,
                cantidad: row.get(4)?,
                fecha: row.get(5)?,
                id_usuario: row.get(6)?,
                nombre_usuario: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }

    Ok(lista)
}

/// Devuelve la cantidad total de unidades ingresadas para un producto.
pub fn total_entradas_por_producto_logic(
    conn: &Connection,
    id_producto: i32,
) -> Result<i32, String> {
    let total: i32 = conn
        .query_row(
            "SELECT COALESCE(SUM(cantidad), 0) FROM movimientos_entrada WHERE id_producto = ?1",
            [id_producto],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(total)
}
