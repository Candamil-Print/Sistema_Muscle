use rusqlite::Connection;
use crate::models::reportes::reporte::{
    DashboardResumen, ProductoMasVendido, ReporteCaja, ReporteEntradasProducto,
    ReporteInventario, ReporteStockBajo, ResumenVentasDiario, ResumenVentasRango,
    VentasPorMetodoPago, VentasPorUsuario,
};

fn validar_rango_fechas(fecha_inicio: &str, fecha_fin: &str) -> Result<(), String> {
    if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        return Err("Las fechas de inicio y fin son obligatorias".to_string());
    }
    Ok(())
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

/// Resumen de ventas diario filtrado por rango de fechas (YYYY-MM-DD).
pub fn resumen_ventas_diario_rango_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<ResumenVentasDiario>, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    let mut stmt = conn
        .prepare(
            r#"SELECT fecha, numero_ventas, total_efectivo, total_transferencia, total_general
               FROM vista_resumen_ventas_diario
               WHERE DATE(fecha) BETWEEN DATE(?1) AND DATE(?2)
               ORDER BY fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
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

/// Totales consolidados de ventas en un rango de fechas.
pub fn resumen_ventas_rango_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<ResumenVentasRango, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    conn.query_row(
        r#"SELECT COUNT(DISTINCT v.id_venta),
                  COALESCE(SUM(CASE WHEN mp.nombre = 'EFECTIVO' THEN dv.subtotal ELSE 0 END), 0),
                  COALESCE(SUM(CASE WHEN mp.nombre = 'TRANSFERENCIA' THEN dv.subtotal ELSE 0 END), 0),
                  COALESCE(SUM(dv.subtotal), 0)
           FROM ventas v
           INNER JOIN detalle_venta dv ON v.id_venta = dv.id_venta
           INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
           WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)"#,
        rusqlite::params![fecha_inicio, fecha_fin],
        |row| {
            Ok(ResumenVentasRango {
                fecha_inicio: fecha_inicio.to_string(),
                fecha_fin: fecha_fin.to_string(),
                numero_ventas: row.get(0)?,
                total_efectivo: row.get(1)?,
                total_transferencia: row.get(2)?,
                total_general: row.get(3)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

/// Top de productos más vendidos en un rango (por cantidad).
pub fn productos_mas_vendidos_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
    limite: i32,
) -> Result<Vec<ProductoMasVendido>, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;
    let limite = limite.max(1).min(100);

    let mut stmt = conn
        .prepare(
            r#"SELECT dv.id_producto, p.nombre, p.tipo_producto,
                      SUM(dv.cantidad) AS cantidad_vendida,
                      SUM(dv.subtotal) AS total_ventas
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               INNER JOIN productos p ON dv.id_producto = p.id_producto
               WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)
               GROUP BY dv.id_producto
               ORDER BY cantidad_vendida DESC
               LIMIT ?3"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin, limite], |row| {
            Ok(ProductoMasVendido {
                id_producto: row.get(0)?,
                nombre_producto: row.get(1)?,
                tipo_producto: row.get(2)?,
                cantidad_vendida: row.get(3)?,
                total_ventas: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Productos con stock bajo (vista `vista_productos_stock_bajo`).
pub fn reporte_stock_bajo_logic(conn: &Connection) -> Result<Vec<ReporteStockBajo>, String> {
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
            Ok(ReporteStockBajo {
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

/// Inventario actual de productos activos con stock.
pub fn reporte_inventario_logic(conn: &Connection) -> Result<Vec<ReporteInventario>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT p.id_producto, p.nombre, p.tipo_producto,
                      s.stock_actual, s.stock_maximo, s.stock_minimo,
                      p.precio_costo, p.precio_sugerido
               FROM productos p
               INNER JOIN stock s ON p.id_producto = s.id_producto
               WHERE p.activo = 1
               ORDER BY p.nombre"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ReporteInventario {
                id_producto: row.get(0)?,
                nombre: row.get(1)?,
                tipo_producto: row.get(2)?,
                stock_actual: row.get(3)?,
                stock_maximo: row.get(4)?,
                stock_minimo: row.get(5)?,
                precio_costo: row.get(6)?,
                precio_sugerido: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Entradas de inventario agrupadas por producto en un rango de fechas.
pub fn reporte_entradas_rango_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<ReporteEntradasProducto>, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    let mut stmt = conn
        .prepare(
            r#"SELECT me.id_producto, p.nombre,
                      SUM(me.cantidad) AS cantidad_ingresada,
                      COUNT(me.id_movimiento) AS numero_movimientos
               FROM movimientos_entrada me
               INNER JOIN productos p ON me.id_producto = p.id_producto
               WHERE DATE(me.fecha) BETWEEN DATE(?1) AND DATE(?2)
               GROUP BY me.id_producto
               ORDER BY cantidad_ingresada DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
            Ok(ReporteEntradasProducto {
                id_producto: row.get(0)?,
                nombre_producto: row.get(1)?,
                cantidad_ingresada: row.get(2)?,
                numero_movimientos: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Ventas totales por usuario en un rango de fechas.
pub fn ventas_por_usuario_reporte_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<VentasPorUsuario>, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    let mut stmt = conn
        .prepare(
            r#"SELECT v.id_usuario, u.nombre_completo,
                      COUNT(DISTINCT v.id_venta) AS numero_ventas,
                      COALESCE(SUM(dv.subtotal), 0) AS total_vendido
               FROM ventas v
               INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
               INNER JOIN detalle_venta dv ON v.id_venta = dv.id_venta
               WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)
               GROUP BY v.id_usuario
               ORDER BY total_vendido DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
            Ok(VentasPorUsuario {
                id_usuario: row.get(0)?,
                nombre_usuario: row.get(1)?,
                numero_ventas: row.get(2)?,
                total_vendido: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Ventas agrupadas por método de pago en un rango de fechas.
pub fn ventas_por_metodo_pago_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<VentasPorMetodoPago>, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    let mut stmt = conn
        .prepare(
            r#"SELECT mp.id_metodo, mp.nombre,
                      COUNT(dv.id_detalle) AS cantidad_lineas,
                      COALESCE(SUM(dv.subtotal), 0) AS total
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
               WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)
               GROUP BY mp.id_metodo
               ORDER BY total DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
            Ok(VentasPorMetodoPago {
                id_metodo: row.get(0)?,
                nombre_metodo: row.get(1)?,
                cantidad_lineas: row.get(2)?,
                total: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Cajas registradas en un rango de fechas de apertura.
pub fn reporte_cajas_rango_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<ReporteCaja>, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    let mut stmt = conn
        .prepare(
            r#"SELECT c.id_caja, c.fecha_apertura, c.fecha_cierre, c.estado,
                      c.monto_apertura, c.monto_cierre, c.total_efectivo,
                      c.total_transferencia, u.nombre_completo
               FROM caja c
               INNER JOIN usuarios u ON c.id_usuario_apertura = u.id_usuario
               WHERE DATE(c.fecha_apertura) BETWEEN DATE(?1) AND DATE(?2)
               ORDER BY c.fecha_apertura DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
            Ok(ReporteCaja {
                id_caja: row.get(0)?,
                fecha_apertura: row.get(1)?,
                fecha_cierre: row.get(2)?,
                estado: row.get(3)?,
                monto_apertura: row.get(4)?,
                monto_cierre: row.get(5)?,
                total_efectivo: row.get(6)?,
                total_transferencia: row.get(7)?,
                nombre_usuario_apertura: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// KPIs del día actual para el panel principal.
pub fn dashboard_resumen_logic(conn: &Connection) -> Result<DashboardResumen, String> {
    let ventas_hoy: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM ventas WHERE DATE(fecha) = DATE('now', 'localtime')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_vendido_hoy: f64 = conn
        .query_row(
            r#"SELECT COALESCE(SUM(dv.subtotal), 0)
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               WHERE DATE(v.fecha) = DATE('now', 'localtime')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let productos_stock_bajo: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM vista_productos_stock_bajo",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let notificaciones_pendientes: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM notificaciones WHERE estado = 0",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let entradas_hoy: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM movimientos_entrada WHERE DATE(fecha) = DATE('now', 'localtime')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let fecha: String = conn
        .query_row(
            "SELECT DATE('now', 'localtime')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(DashboardResumen {
        fecha,
        ventas_hoy,
        total_vendido_hoy,
        productos_stock_bajo,
        notificaciones_pendientes,
        entradas_hoy,
    })
}
