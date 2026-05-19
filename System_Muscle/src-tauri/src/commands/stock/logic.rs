use rusqlite::Connection;
use crate::models::stock::stock::{
    Stock, StockConProducto, AjusteStock, Notificacion, ProductoStockBajo,
};

/// Obtiene el stock de un producto por su id_producto.
pub fn obtener_stock_por_producto_logic(conn: &Connection, id_producto: i32) -> Result<Stock, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT id_stock, id_producto, stock_actual, stock_maximo, stock_minimo, fecha_actualizacion
               FROM stock WHERE id_producto = ?1"#,
        )
        .map_err(|e| e.to_string())?;

    let stock = stmt
        .query_row([id_producto], |row| {
            Ok(Stock {
                id_stock: row.get(0)?,
                id_producto: row.get(1)?,
                stock_actual: row.get(2)?,
                stock_maximo: row.get(3)?,
                stock_minimo: row.get(4)?,
                fecha_actualizacion: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(stock)
}

/// Lista todo el stock con información del producto (JOIN con productos).
pub fn listar_stock_logic(conn: &Connection) -> Result<Vec<StockConProducto>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT s.id_stock, s.id_producto, p.nombre, p.tipo_producto,
                      s.stock_actual, s.stock_maximo, s.stock_minimo, s.fecha_actualizacion
               FROM stock s
               INNER JOIN productos p ON s.id_producto = p.id_producto
               WHERE p.activo = 1
               ORDER BY p.nombre"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(StockConProducto {
                id_stock: row.get(0)?,
                id_producto: row.get(1)?,
                nombre_producto: row.get(2)?,
                tipo_producto: row.get(3)?,
                stock_actual: row.get(4)?,
                stock_maximo: row.get(5)?,
                stock_minimo: row.get(6)?,
                fecha_actualizacion: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }

    Ok(lista)
}

/// Ajuste directo del stock_actual (y opcionalmente stock_maximo) de un producto.
pub fn ajustar_stock_logic(
    conn: &Connection,
    id_producto: i32,
    ajuste: &AjusteStock,
) -> Result<(), String> {
    if let Some(stock_maximo) = ajuste.stock_maximo {
        conn.execute(
            r#"UPDATE stock
               SET stock_actual = ?1, stock_maximo = ?2, fecha_actualizacion = CURRENT_TIMESTAMP
               WHERE id_producto = ?3"#,
            rusqlite::params![ajuste.stock_actual, stock_maximo, id_producto],
        )
        .map_err(|e| e.to_string())?;
    } else {
        conn.execute(
            r#"UPDATE stock
               SET stock_actual = ?1, fecha_actualizacion = CURRENT_TIMESTAMP
               WHERE id_producto = ?2"#,
            rusqlite::params![ajuste.stock_actual, id_producto],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Lista los productos con stock por debajo del mínimo usando la vista de la BD.
pub fn listar_stock_bajo_logic(conn: &Connection) -> Result<Vec<ProductoStockBajo>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT id_producto, nombre, tipo_producto,
                      stock_actual, stock_maximo, stock_minimo, porcentaje_stock
               FROM vista_productos_stock_bajo
               ORDER BY porcentaje_stock ASC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ProductoStockBajo {
                id_producto: row.get(0)?,
                nombre: row.get(1)?,
                tipo_producto: row.get(2)?,
                stock_actual: row.get(3)?,
                stock_maximo: row.get(4)?,
                stock_minimo: row.get(5)?,
                porcentaje_stock: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }

    Ok(lista)
}

/// Lista las notificaciones de stock bajo.
/// Si `solo_no_leidas` es true, filtra estado = 0.
pub fn listar_notificaciones_logic(
    conn: &Connection,
    solo_no_leidas: bool,
) -> Result<Vec<Notificacion>, String> {
    let query = if solo_no_leidas {
        r#"SELECT n.id_notificacion, n.id_producto, p.nombre,
                  n.mensaje, n.stock_actual, n.stock_minimo, n.fecha, n.estado
           FROM notificaciones n
           INNER JOIN productos p ON n.id_producto = p.id_producto
           WHERE n.estado = 0
           ORDER BY n.fecha DESC"#
    } else {
        r#"SELECT n.id_notificacion, n.id_producto, p.nombre,
                  n.mensaje, n.stock_actual, n.stock_minimo, n.fecha, n.estado
           FROM notificaciones n
           INNER JOIN productos p ON n.id_producto = p.id_producto
           ORDER BY n.fecha DESC"#
    };

    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
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
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }

    Ok(lista)
}

/// Marca una notificación con el estado dado (1: leída, 2: atendida).
pub fn marcar_notificacion_logic(
    conn: &Connection,
    id_notificacion: i32,
    estado: i32,
) -> Result<(), String> {
    if estado != 1 && estado != 2 {
        return Err("Estado inválido. Use 1 (leída) o 2 (atendida)".to_string());
    }

    conn.execute(
        "UPDATE notificaciones SET estado = ?1 WHERE id_notificacion = ?2",
        rusqlite::params![estado, id_notificacion],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
