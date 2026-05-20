//! Tests del módulo de reportes
//! Ejecutar con: cargo test --test reportes_test -- --nocapture

use system_muscle_lib::commands::*;
use system_muscle_lib::models::movimientos_entrada::movimiento_entrada::NuevoMovimientoEntrada;
use system_muscle_lib::models::productos::producto::NuevoProducto;
use system_muscle_lib::models::ventas::venta::{LineaVenta, NuevaVenta};
use system_muscle_lib::services::db::connection::get_db_connection;
use std::time::{SystemTime, UNIX_EPOCH};

fn nombre_unico(prefijo: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{} {}", prefijo, ts)
}

fn crear_caja_abierta(conn: &rusqlite::Connection) -> i32 {
    conn.execute(
        r#"INSERT INTO caja (fecha_apertura, monto_apertura, estado, id_usuario_apertura)
           VALUES (datetime('now'), 0, 'ABIERTA', 1)"#,
        [],
    )
    .unwrap();
    conn.last_insert_rowid() as i32
}

fn limpiar_caja(conn: &rusqlite::Connection, id_caja: i32) {
    let _ = conn.execute("DELETE FROM movimientos_caja WHERE id_caja = ?1", [id_caja]);
    let _ = conn.execute("DELETE FROM ventas WHERE id_caja = ?1", [id_caja]);
    let _ = conn.execute("DELETE FROM caja WHERE id_caja = ?1", [id_caja]);
}

fn crear_producto_con_stock(conn: &rusqlite::Connection, nombre: &str, stock_inicial: i32) -> i32 {
    let id = crear_producto_logic(
        conn,
        &NuevoProducto {
            nombre: nombre.to_string(),
            tipo_producto: "SNACKS".to_string(),
            precio_costo: 5000.0,
            precio_sugerido: 8000.0,
            imagen_url: None,
            stock_maximo: 100,
        },
    )
    .unwrap();

    if stock_inicial > 0 {
        registrar_entrada_logic(
            conn,
            &NuevoMovimientoEntrada {
                id_producto: id,
                cantidad: stock_inicial,
                id_usuario: 1,
            },
        )
        .unwrap();
    }
    id
}

fn limpiar_producto(conn: &rusqlite::Connection, id_producto: i32) {
    let _ = conn.execute("DELETE FROM detalle_venta WHERE id_producto = ?1", [id_producto]);
    let _ = conn.execute("DELETE FROM movimientos_entrada WHERE id_producto = ?1", [id_producto]);
    let _ = conn.execute("DELETE FROM stock WHERE id_producto = ?1", [id_producto]);
    let _ = conn.execute("DELETE FROM productos WHERE id_producto = ?1", [id_producto]);
}

#[test]
fn test_dashboard_resumen() {
    println!("\n📊 TEST: Dashboard resumen");
    let conn = get_db_connection().unwrap();
    let dashboard = dashboard_resumen_logic(&conn).unwrap();
    assert!(!dashboard.fecha.is_empty());
    println!("   ✅ Dashboard fecha: {}", dashboard.fecha);
}

#[test]
fn test_resumen_ventas_rango() {
    println!("\n📊 TEST: Resumen ventas por rango");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre = nombre_unico("ReporteTest Venta");
    let id_producto = crear_producto_con_stock(&conn, &nombre, 50);

    registrar_venta_logic(
        &conn,
        &NuevaVenta {
            id_usuario: 1,
            id_caja,
            id_turno: None,
            lineas: vec![LineaVenta {
                id_producto,
                cantidad: 5,
                precio_unitario: 10000.0,
                metodo_pago: 1,
            }],
        },
    )
    .unwrap();

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let resumen = resumen_ventas_rango_logic(&conn, &hoy, &hoy).unwrap();
    assert!(resumen.numero_ventas >= 1);
    assert!(resumen.total_general >= 50000.0);
    println!("   ✅ Ventas en rango: {} por ${}", resumen.numero_ventas, resumen.total_general);

    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_productos_mas_vendidos() {
    println!("\n📊 TEST: Productos más vendidos");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre = nombre_unico("ReporteTest Top");
    let id_producto = crear_producto_con_stock(&conn, &nombre, 30);

    registrar_venta_logic(
        &conn,
        &NuevaVenta {
            id_usuario: 1,
            id_caja,
            id_turno: None,
            lineas: vec![LineaVenta {
                id_producto,
                cantidad: 8,
                precio_unitario: 5000.0,
                metodo_pago: 2,
            }],
        },
    )
    .unwrap();

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let top = productos_mas_vendidos_logic(&conn, &hoy, &hoy, 10).unwrap();
    assert!(top.iter().any(|p| p.id_producto == id_producto && p.cantidad_vendida == 8));

    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_reporte_inventario() {
    println!("\n📊 TEST: Reporte inventario");
    let conn = get_db_connection().unwrap();
    let nombre = nombre_unico("ReporteTest Inv");
    let id = crear_producto_con_stock(&conn, &nombre, 15);

    let inventario = reporte_inventario_logic(&conn).unwrap();
    assert!(inventario.iter().any(|p| p.id_producto == id && p.stock_actual == 15));

    limpiar_producto(&conn, id);
}

#[test]
fn test_reporte_entradas_rango() {
    println!("\n📊 TEST: Reporte entradas por rango");
    let conn = get_db_connection().unwrap();
    let nombre = nombre_unico("ReporteTest Entrada");
    let id = crear_producto_con_stock(&conn, &nombre, 25);

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let entradas = reporte_entradas_rango_logic(&conn, &hoy, &hoy).unwrap();
    let item = entradas.iter().find(|e| e.id_producto == id).unwrap();
    assert_eq!(item.cantidad_ingresada, 25);

    limpiar_producto(&conn, id);
}

#[test]
fn test_ventas_por_metodo_pago_reporte() {
    println!("\n📊 TEST: Ventas por método de pago");
    let conn = get_db_connection().unwrap();
    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let metodos = ventas_por_metodo_pago_logic(&conn, &hoy, &hoy).unwrap();
    assert!(metodos.iter().all(|m| m.id_metodo == 1 || m.id_metodo == 2));
}

#[test]
fn test_resumen_ventas_diario() {
    println!("\n📊 TEST: Resumen ventas diario (vista)");
    let conn = get_db_connection().unwrap();
    let resumen = resumen_ventas_diario_logic(&conn).unwrap();
    for r in &resumen {
        assert!(r.total_general >= 0.0);
    }
    println!("   ✅ Días con datos: {}", resumen.len());
}

#[test]
fn test_rango_fechas_invalido() {
    println!("\n📊 TEST: Rango de fechas inválido");
    let conn = get_db_connection().unwrap();
    let resultado = resumen_ventas_rango_logic(&conn, "", "2026-01-01");
    assert!(resultado.is_err());
}
