use rusqlite::Connection;
use crate::models::stock::stock::{
    Stock, StockConProducto, AjusteStock,
    MovimientoEntrada, MovimientoEntradaDetalle,
    NuevoMovimientoEntrada, Notificacion, ProductoStockBajo,
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
/// Útil para correcciones manuales de inventario.
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

    let existe = stmt
        .exists([entrada.id_producto])
        .map_err(|e| e.to_string())?;

    if !existe {
        return Err("El producto no existe o está inactivo".to_string());
    }

    // Verificar que el usuario existe
    let mut stmt_u = conn
        .prepare("SELECT 1 FROM usuarios WHERE id_usuario = ?1 AND estado = 1")
        .map_err(|e| e.to_string())?;

    let existe_usuario = stmt_u
        .exists([entrada.id_usuario])
        .map_err(|e| e.to_string())?;

    if !existe_usuario {
        return Err("El usuario no existe o está inactivo".to_string());
    }

    conn.execute(
        r#"INSERT INTO movimientos_entrada (id_producto, cantidad, id_usuario)
           VALUES (?1, ?2, ?3)"#,
        rusqlite::params![entrada.id_producto, entrada.cantidad, entrada.id_usuario],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid() as i32)
}

/// Lista todos los movimientos de entrada con detalles del producto y usuario.
pub fn listar_movimientos_entrada_logic(
    conn: &Connection,
) -> Result<Vec<MovimientoEntradaDetalle>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT me.id_movimiento, me.id_producto, p.nombre,
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
                cantidad: row.get(3)?,
                fecha: row.get(4)?,
                id_usuario: row.get(5)?,
                nombre_usuario: row.get(6)?,
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
) -> Result<Vec<MovimientoEntrada>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT id_movimiento, id_producto, cantidad, fecha, id_usuario
               FROM movimientos_entrada
               WHERE id_producto = ?1
               ORDER BY fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([id_producto], |row| {
            Ok(MovimientoEntrada {
                id_movimiento: row.get(0)?,
                id_producto: row.get(1)?,
                cantidad: row.get(2)?,
                fecha: row.get(3)?,
                id_usuario: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }

    Ok(lista)
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
