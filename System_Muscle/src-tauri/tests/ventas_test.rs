//! Tests del módulo de ventas
//! Ejecutar con: cargo test --test ventas_test -- --nocapture

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

/// Abre una caja de prueba y devuelve su id_caja
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

/// Crea producto con stock inicial vía entrada de inventario
fn crear_producto_con_stock(conn: &rusqlite::Connection, nombre: &str, stock_inicial: i32) -> i32 {
    let nuevo = NuevoProducto {
        nombre: nombre.to_string(),
        tipo_producto: "SNACKS".to_string(),
        precio_costo: 5000.0,
        precio_sugerido: 8000.0,
        imagen_url: None,
        stock_maximo: 200,
    };
    let id = crear_producto_logic(conn, &nuevo).unwrap();

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
    let _ = conn.execute(
        "DELETE FROM detalle_venta WHERE id_producto = ?1",
        [id_producto],
    );
    let _ = conn.execute(
        "DELETE FROM movimientos_entrada WHERE id_producto = ?1",
        [id_producto],
    );
    let _ = conn.execute("DELETE FROM stock WHERE id_producto = ?1", [id_producto]);
    let _ = conn.execute("DELETE FROM productos WHERE id_producto = ?1", [id_producto]);
}

#[test]
fn test_registrar_venta_exitosa() {
    println!("\n🛒 TEST: Registrar venta exitosa");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre = nombre_unico("VentaTest Ok");
    let id_producto = crear_producto_con_stock(&conn, &nombre, 50);

    let venta = NuevaVenta {
        id_usuario: 1,
        id_caja,
        id_turno: None,
        lineas: vec![LineaVenta {
            id_producto,
            cantidad: 10,
            precio_unitario: 8000.0,
            metodo_pago: 1,
        }],
    };

    let id_venta = registrar_venta_logic(&conn, &venta).unwrap();
    assert!(id_venta > 0);

    let stock = obtener_stock_por_producto_logic(&conn, id_producto).unwrap();
    assert_eq!(stock.stock_actual, 40);
    println!("   ✅ Stock descontado por trigger: 50 → 40");

    let detalle = listar_detalle_venta_logic(&conn, id_venta).unwrap();
    assert_eq!(detalle.len(), 1);
    assert_eq!(detalle[0].cantidad, 10);
    assert_eq!(detalle[0].subtotal, 80000.0);

    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_registrar_venta_stock_insuficiente() {
    println!("\n🛒 TEST: Venta con stock insuficiente");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre = nombre_unico("VentaTest Sin Stock");
    let id_producto = crear_producto_con_stock(&conn, &nombre, 5);

    let venta = NuevaVenta {
        id_usuario: 1,
        id_caja,
        id_turno: None,
        lineas: vec![LineaVenta {
            id_producto,
            cantidad: 20,
            precio_unitario: 5000.0,
            metodo_pago: 1,
        }],
    };

    let resultado = registrar_venta_logic(&conn, &venta);
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("Stock insuficiente"));

    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_registrar_venta_sin_lineas() {
    println!("\n🛒 TEST: Venta sin líneas");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);

    let venta = NuevaVenta {
        id_usuario: 1,
        id_caja,
        id_turno: None,
        lineas: vec![],
    };

    let resultado = registrar_venta_logic(&conn, &venta);
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("al menos una línea"));

    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_registrar_venta_caja_cerrada() {
    println!("\n🛒 TEST: Venta con caja cerrada");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    conn.execute(
        "UPDATE caja SET estado = 'CERRADA' WHERE id_caja = ?1",
        [id_caja],
    )
    .unwrap();

    let nombre = nombre_unico("VentaTest Caja Cerrada");
    let id_producto = crear_producto_con_stock(&conn, &nombre, 10);

    let venta = NuevaVenta {
        id_usuario: 1,
        id_caja,
        id_turno: None,
        lineas: vec![LineaVenta {
            id_producto,
            cantidad: 1,
            precio_unitario: 1000.0,
            metodo_pago: 2,
        }],
    };

    let resultado = registrar_venta_logic(&conn, &venta);
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("no está abierta"));

    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_obtener_venta_y_listar() {
    println!("\n🛒 TEST: Obtener venta y listar detalle");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre = nombre_unico("VentaTest Obtener");
    let id_producto = crear_producto_con_stock(&conn, &nombre, 30);

    let id_venta = registrar_venta_logic(
        &conn,
        &NuevaVenta {
            id_usuario: 1,
            id_caja,
            id_turno: None,
            lineas: vec![
                LineaVenta {
                    id_producto,
                    cantidad: 2,
                    precio_unitario: 5000.0,
                    metodo_pago: 1,
                },
                LineaVenta {
                    id_producto,
                    cantidad: 3,
                    precio_unitario: 5000.0,
                    metodo_pago: 2,
                },
            ],
        },
    )
    .unwrap();

    let cabecera = obtener_venta_logic(&conn, id_venta).unwrap();
    assert_eq!(cabecera.id_usuario, 1);
    assert_eq!(cabecera.id_caja, id_caja);

    let lineas = listar_detalle_venta_logic(&conn, id_venta).unwrap();
    assert_eq!(lineas.len(), 2);
    assert_eq!(lineas.iter().map(|l| l.subtotal).sum::<f64>(), 25000.0);

    let ventas = listar_ventas_logic(&conn).unwrap();
    assert!(ventas.iter().any(|v| v.id_venta == id_venta && v.total == 25000.0));

    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_total_ventas_por_producto() {
    println!("\n🛒 TEST: Total vendido por producto");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre = nombre_unico("VentaTest Total");
    let id_producto = crear_producto_con_stock(&conn, &nombre, 100);

    for cantidad in [5, 15] {
        registrar_venta_logic(
            &conn,
            &NuevaVenta {
                id_usuario: 1,
                id_caja,
                id_turno: None,
                lineas: vec![LineaVenta {
                    id_producto,
                    cantidad,
                    precio_unitario: 1000.0,
                    metodo_pago: 1,
                }],
            },
        )
        .unwrap();
    }

    let total = total_ventas_por_producto_logic(&conn, id_producto).unwrap();
    assert_eq!(total, 20);

    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_ventas_por_usuario() {
    println!("\n🛒 TEST: Ventas por usuario");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre = nombre_unico("VentaTest Usuario");
    let id_producto = crear_producto_con_stock(&conn, &nombre, 20);

    registrar_venta_logic(
        &conn,
        &NuevaVenta {
            id_usuario: 1,
            id_caja,
            id_turno: None,
            lineas: vec![LineaVenta {
                id_producto,
                cantidad: 4,
                precio_unitario: 2000.0,
                metodo_pago: 1,
            }],
        },
    )
    .unwrap();

    let ventas = ventas_por_usuario_logic(&conn, 1).unwrap();
    assert!(ventas.iter().any(|v| v.id_caja == id_caja && v.total == 8000.0));

    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}
