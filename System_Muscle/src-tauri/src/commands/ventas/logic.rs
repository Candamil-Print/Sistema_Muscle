use rusqlite::Connection;
use crate::models::ventas::venta::{
    DetalleVentaDetalle, NuevaVenta, ResumenVentasDiario, Venta, VentaResumen,
};

const METODO_EFECTIVO: i32 = 1;
const METODO_TRANSFERENCIA: i32 = 2;

/// Registra una venta con sus líneas de detalle en una transacción.
/// Los triggers `trg_historial_venta` y `trg_actualizar_stock_venta` actualizan historial y stock.
pub fn registrar_venta_logic(conn: &Connection, venta: &NuevaVenta) -> Result<i32, String> {
    if venta.lineas.is_empty() {
        return Err("La venta debe tener al menos una línea".to_string());
    }

    validar_usuario_activo(conn, venta.id_usuario)?;
    validar_caja_abierta(conn, venta.id_caja)?;

    if let Some(id_turno) = venta.id_turno {
        validar_turno_abierto(conn, id_turno)?;
    }

    for linea in &venta.lineas {
        validar_linea(conn, linea)?;
    }

    validar_stock_suficiente(conn, &venta.lineas)?;

    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;

    tx.execute(
        r#"INSERT INTO ventas (id_usuario, id_caja, id_turno)
           VALUES (?1, ?2, ?3)"#,
        rusqlite::params![venta.id_usuario, venta.id_caja, venta.id_turno],
    )
    .map_err(|e| e.to_string())?;

    let id_venta = tx.last_insert_rowid() as i32;
    let mut total_efectivo = 0.0_f64;
    let mut total_transferencia = 0.0_f64;

    for linea in &venta.lineas {
        tx.execute(
            r#"INSERT INTO detalle_venta (id_venta, id_producto, cantidad, precio_unitario, metodo_pago)
               VALUES (?1, ?2, ?3, ?4, ?5)"#,
            rusqlite::params![
                id_venta,
                linea.id_producto,
                linea.cantidad,
                linea.precio_unitario,
                linea.metodo_pago,
            ],
        )
        .map_err(|e| e.to_string())?;

        let subtotal = linea.cantidad as f64 * linea.precio_unitario;

        if linea.metodo_pago == METODO_EFECTIVO {
            total_efectivo += subtotal;
        } else {
            total_transferencia += subtotal;
        }

        tx.execute(
            r#"INSERT INTO movimientos_caja (id_caja, id_venta, tipo_movimiento, monto, descripcion)
               VALUES (?1, ?2, 'INGRESO', ?3, ?4)"#,
            rusqlite::params![
                venta.id_caja,
                id_venta,
                subtotal,
                format!("Venta #{} - producto {}", id_venta, linea.id_producto),
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    if total_efectivo > 0.0 {
        tx.execute(
            r#"UPDATE caja
               SET total_efectivo = total_efectivo + ?1
               WHERE id_caja = ?2"#,
            rusqlite::params![total_efectivo, venta.id_caja],
        )
        .map_err(|e| e.to_string())?;
    }

    if total_transferencia > 0.0 {
        tx.execute(
            r#"UPDATE caja
               SET total_transferencia = total_transferencia + ?1
               WHERE id_caja = ?2"#,
            rusqlite::params![total_transferencia, venta.id_caja],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(id_venta)
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

fn validar_caja_abierta(conn: &Connection, id_caja: i32) -> Result<(), String> {
    let estado: String = conn
        .query_row(
            "SELECT estado FROM caja WHERE id_caja = ?1",
            [id_caja],
            |row| row.get(0),
        )
        .map_err(|_| "La caja no existe".to_string())?;

    if estado != "ABIERTA" {
        return Err("La caja no está abierta".to_string());
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

fn validar_linea(
    conn: &Connection,
    linea: &crate::models::ventas::venta::LineaVenta,
) -> Result<(), String> {
    if linea.cantidad <= 0 {
        return Err("La cantidad debe ser mayor a cero".to_string());
    }
    if linea.precio_unitario <= 0.0 {
        return Err("El precio unitario debe ser mayor a cero".to_string());
    }
    if linea.metodo_pago != METODO_EFECTIVO && linea.metodo_pago != METODO_TRANSFERENCIA {
        return Err("Método de pago inválido".to_string());
    }

    let mut stmt = conn
        .prepare("SELECT 1 FROM productos WHERE id_producto = ?1 AND activo = 1")
        .map_err(|e| e.to_string())?;

    if !stmt
        .exists([linea.id_producto])
        .map_err(|e| e.to_string())?
    {
        return Err(format!(
            "El producto {} no existe o está inactivo",
            linea.id_producto
        ));
    }

    let mut stmt_mp = conn
        .prepare("SELECT 1 FROM metodos_pago WHERE id_metodo = ?1")
        .map_err(|e| e.to_string())?;

    if !stmt_mp
        .exists([linea.metodo_pago])
        .map_err(|e| e.to_string())?
    {
        return Err("Método de pago no registrado".to_string());
    }

    Ok(())
}

fn validar_stock_suficiente(
    conn: &Connection,
    lineas: &[crate::models::ventas::venta::LineaVenta],
) -> Result<(), String> {
    use std::collections::HashMap;

    let mut cantidad_por_producto: HashMap<i32, i32> = HashMap::new();
    for linea in lineas {
        *cantidad_por_producto
            .entry(linea.id_producto)
            .or_insert(0) += linea.cantidad;
    }

    for (id_producto, cantidad_total) in cantidad_por_producto {
        let stock_actual: i32 = conn
            .query_row(
                "SELECT stock_actual FROM stock WHERE id_producto = ?1",
                [id_producto],
                |row| row.get(0),
            )
            .map_err(|_| format!("No hay registro de stock para el producto {}", id_producto))?;

        if stock_actual < cantidad_total {
            return Err(format!(
                "Stock insuficiente para el producto {}: disponible {}, solicitado {}",
                id_producto, stock_actual, cantidad_total
            ));
        }
    }

    Ok(())
}

/// Obtiene la cabecera de una venta por id.
pub fn obtener_venta_logic(conn: &Connection, id_venta: i32) -> Result<Venta, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT id_venta, fecha, id_usuario, id_caja, id_turno
               FROM ventas WHERE id_venta = ?1"#,
        )
        .map_err(|e| e.to_string())?;

    stmt.query_row([id_venta], |row| {
        Ok(Venta {
            id_venta: row.get(0)?,
            fecha: row.get(1)?,
            id_usuario: row.get(2)?,
            id_caja: row.get(3)?,
            id_turno: row.get(4)?,
        })
    })
    .map_err(|e| e.to_string())
}

/// Lista las líneas de detalle de una venta con nombres de producto y método de pago.
pub fn listar_detalle_venta_logic(
    conn: &Connection,
    id_venta: i32,
) -> Result<Vec<DetalleVentaDetalle>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT dv.id_detalle, dv.id_venta, dv.id_producto, p.nombre,
                      dv.cantidad, dv.precio_unitario, dv.metodo_pago, mp.nombre, dv.subtotal
               FROM detalle_venta dv
               INNER JOIN productos p ON dv.id_producto = p.id_producto
               INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
               WHERE dv.id_venta = ?1
               ORDER BY dv.id_detalle"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([id_venta], |row| {
            Ok(DetalleVentaDetalle {
                id_detalle: row.get(0)?,
                id_venta: row.get(1)?,
                id_producto: row.get(2)?,
                nombre_producto: row.get(3)?,
                cantidad: row.get(4)?,
                precio_unitario: row.get(5)?,
                metodo_pago: row.get(6)?,
                nombre_metodo_pago: row.get(7)?,
                subtotal: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Lista todas las ventas con total y nombre del usuario.
pub fn listar_ventas_logic(conn: &Connection) -> Result<Vec<VentaResumen>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT v.id_venta, v.fecha, v.id_usuario, u.nombre_completo, v.id_caja,
                      COALESCE(SUM(dv.subtotal), 0) AS total
               FROM ventas v
               INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
               LEFT JOIN detalle_venta dv ON v.id_venta = dv.id_venta
               GROUP BY v.id_venta
               ORDER BY v.fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(VentaResumen {
                id_venta: row.get(0)?,
                fecha: row.get(1)?,
                id_usuario: row.get(2)?,
                nombre_usuario: row.get(3)?,
                id_caja: row.get(4)?,
                total: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Lista ventas registradas por un usuario.
pub fn ventas_por_usuario_logic(
    conn: &Connection,
    id_usuario: i32,
) -> Result<Vec<VentaResumen>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT v.id_venta, v.fecha, v.id_usuario, u.nombre_completo, v.id_caja,
                      COALESCE(SUM(dv.subtotal), 0) AS total
               FROM ventas v
               INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
               LEFT JOIN detalle_venta dv ON v.id_venta = dv.id_venta
               WHERE v.id_usuario = ?1
               GROUP BY v.id_venta
               ORDER BY v.fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([id_usuario], |row| {
            Ok(VentaResumen {
                id_venta: row.get(0)?,
                fecha: row.get(1)?,
                id_usuario: row.get(2)?,
                nombre_usuario: row.get(3)?,
                id_caja: row.get(4)?,
                total: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Lista ventas asociadas a una caja.
pub fn ventas_por_caja_logic(
    conn: &Connection,
    id_caja: i32,
) -> Result<Vec<VentaResumen>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT v.id_venta, v.fecha, v.id_usuario, u.nombre_completo, v.id_caja,
                      COALESCE(SUM(dv.subtotal), 0) AS total
               FROM ventas v
               INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
               LEFT JOIN detalle_venta dv ON v.id_venta = dv.id_venta
               WHERE v.id_caja = ?1
               GROUP BY v.id_venta
               ORDER BY v.fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([id_caja], |row| {
            Ok(VentaResumen {
                id_venta: row.get(0)?,
                fecha: row.get(1)?,
                id_usuario: row.get(2)?,
                nombre_usuario: row.get(3)?,
                id_caja: row.get(4)?,
                total: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Lista ventas en un rango de fechas (formato 'YYYY-MM-DD').
pub fn ventas_por_rango_fechas_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<VentaResumen>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT v.id_venta, v.fecha, v.id_usuario, u.nombre_completo, v.id_caja,
                      COALESCE(SUM(dv.subtotal), 0) AS total
               FROM ventas v
               INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
               LEFT JOIN detalle_venta dv ON v.id_venta = dv.id_venta
               WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)
               GROUP BY v.id_venta
               ORDER BY v.fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
            Ok(VentaResumen {
                id_venta: row.get(0)?,
                fecha: row.get(1)?,
                id_usuario: row.get(2)?,
                nombre_usuario: row.get(3)?,
                id_caja: row.get(4)?,
                total: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Resumen diario de ventas desde la vista del sistema.
pub fn resumen_ventas_diario_logic(conn: &Connection) -> Result<Vec<ResumenVentasDiario>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT fecha, numero_ventas, total_efectivo, total_transferencia, total_general
               FROM vista_resumen_ventas_diario
               ORDER BY fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ResumenVentasDiario {
                fecha: row.get(0)?,
                numero_ventas: row.get(1)?,
                total_efectivo: row.get(2)?,
                total_transferencia: row.get(3)?,
                total_general: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Total de unidades vendidas de un producto.
pub fn total_ventas_por_producto_logic(
    conn: &Connection,
    id_producto: i32,
) -> Result<i32, String> {
    let total: i32 = conn
        .query_row(
            "SELECT COALESCE(SUM(cantidad), 0) FROM detalle_venta WHERE id_producto = ?1",
            [id_producto],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(total)
}
