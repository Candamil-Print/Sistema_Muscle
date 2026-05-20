//! Tests del módulo de notificaciones
//! Ejecutar con: cargo test --test notificaciones_test -- --nocapture

use system_muscle_lib::commands::*;
use system_muscle_lib::models::productos::producto::NuevoProducto;
use system_muscle_lib::models::stock::stock::AjusteStock;
use system_muscle_lib::services::db::connection::get_db_connection;
use std::time::{SystemTime, UNIX_EPOCH};

fn nombre_unico(prefijo: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{} {}", prefijo, ts)
}

fn crear_producto_con_stock(conn: &rusqlite::Connection, nombre: &str, stock_maximo: i32) -> i32 {
    let nuevo = NuevoProducto {
        nombre: nombre.to_string(),
        tipo_producto: "SNACKS".to_string(),
        precio_costo: 5000.0,
        precio_sugerido: 8000.0,
        imagen_url: None,
        stock_maximo,
    };
    crear_producto_logic(conn, &nuevo).unwrap()
}

fn generar_notificacion_bajo_stock(conn: &rusqlite::Connection, id_producto: i32, stock_maximo: i32) {
    let stock_minimo = stock_maximo / 4;
    let ajuste = AjusteStock {
        stock_actual: stock_minimo - 1,
        stock_maximo: None,
    };
    ajustar_stock_logic(conn, id_producto, &ajuste).unwrap();
}

fn limpiar_producto(conn: &rusqlite::Connection, id_producto: i32) {
    let _ = conn.execute(
        "DELETE FROM notificaciones WHERE id_producto = ?1",
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
fn test_notificacion_se_genera_al_bajar_stock() {
    println!("\n🔔 TEST: Notificación generada por trigger de stock bajo");
    let conn = get_db_connection().unwrap();
    let nombre = nombre_unico("NotifTest Trigger");
    let id = crear_producto_con_stock(&conn, &nombre, 100);
    generar_notificacion_bajo_stock(&conn, id, 100);

    let notificaciones = listar_notificaciones_logic(&conn, false).unwrap();
    assert!(notificaciones.iter().any(|n| n.id_producto == id));
    println!("   ✅ Notificación creada por trg_notificar_stock_bajo");

    limpiar_producto(&conn, id);
}

#[test]
fn test_obtener_notificacion() {
    println!("\n🔔 TEST: Obtener notificación por id");
    let conn = get_db_connection().unwrap();
    let nombre = nombre_unico("NotifTest Obtener");
    let id = crear_producto_con_stock(&conn, &nombre, 80);
    generar_notificacion_bajo_stock(&conn, id, 80);

    let lista = notificaciones_por_producto_logic(&conn, id).unwrap();
    let id_notif = lista.first().unwrap().id_notificacion;

    let notif = obtener_notificacion_logic(&conn, id_notif).unwrap();
    assert_eq!(notif.id_producto, id);
    assert_eq!(notif.estado, 0);

    limpiar_producto(&conn, id);
}

#[test]
fn test_marcar_notificacion_como_leida() {
    println!("\n🔔 TEST: Marcar notificación como leída");
    let conn = get_db_connection().unwrap();
    let nombre = nombre_unico("NotifTest Leida");
    let id = crear_producto_con_stock(&conn, &nombre, 100);
    generar_notificacion_bajo_stock(&conn, id, 100);

    let notif = listar_notificaciones_logic(&conn, true)
        .unwrap()
        .into_iter()
        .find(|n| n.id_producto == id)
        .unwrap();

    marcar_notificacion_logic(&conn, notif.id_notificacion, 1).unwrap();

    let no_leidas = listar_notificaciones_logic(&conn, true).unwrap();
    assert!(!no_leidas.iter().any(|n| n.id_notificacion == notif.id_notificacion));

    let leidas = notificaciones_por_estado_logic(&conn, 1).unwrap();
    assert!(leidas.iter().any(|n| n.id_notificacion == notif.id_notificacion));
    println!("   ✅ Notificación marcada como leída");

    limpiar_producto(&conn, id);
}

#[test]
fn test_marcar_notificacion_como_atendida() {
    println!("\n🔔 TEST: Marcar notificación como atendida");
    let conn = get_db_connection().unwrap();
    let nombre = nombre_unico("NotifTest Atendida");
    let id = crear_producto_con_stock(&conn, &nombre, 100);
    generar_notificacion_bajo_stock(&conn, id, 100);

    let notif = notificaciones_por_producto_logic(&conn, id)
        .unwrap()
        .into_iter()
        .find(|n| n.estado == 0)
        .unwrap();

    marcar_notificacion_logic(&conn, notif.id_notificacion, 2).unwrap();

    let actualizada = obtener_notificacion_logic(&conn, notif.id_notificacion).unwrap();
    assert_eq!(actualizada.estado, 2);

    limpiar_producto(&conn, id);
}

#[test]
fn test_marcar_notificacion_estado_invalido() {
    println!("\n🔔 TEST: Estado inválido al marcar");
    let conn = get_db_connection().unwrap();
    let resultado = marcar_notificacion_logic(&conn, 1, 99);
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("inválido"));
}

#[test]
fn test_contar_no_leidas() {
    println!("\n🔔 TEST: Contar notificaciones no leídas");
    let conn = get_db_connection().unwrap();

    let nombre = nombre_unico("NotifTest Contar");
    let id = crear_producto_con_stock(&conn, &nombre, 60);
    generar_notificacion_bajo_stock(&conn, id, 60);

    let no_leidas_producto = notificaciones_por_estado_logic(&conn, 0)
        .unwrap()
        .into_iter()
        .filter(|n| n.id_producto == id)
        .count();
    assert!(no_leidas_producto >= 1);

    let total = contar_no_leidas_logic(&conn).unwrap();
    assert!(total >= no_leidas_producto as i32);
    println!("   ✅ Producto con {} alerta(s); total no leídas: {}", no_leidas_producto, total);

    limpiar_producto(&conn, id);
}

#[test]
fn test_marcar_todas_leidas() {
    println!("\n🔔 TEST: Marcar todas las notificaciones como leídas");
    let conn = get_db_connection().unwrap();
    let nombre = nombre_unico("NotifTest Todas");
    let id = crear_producto_con_stock(&conn, &nombre, 40);
    generar_notificacion_bajo_stock(&conn, id, 40);

    let marcadas = marcar_todas_leidas_logic(&conn).unwrap();
    assert!(marcadas >= 1);

    let no_leidas_producto = notificaciones_por_estado_logic(&conn, 0)
        .unwrap()
        .into_iter()
        .filter(|n| n.id_producto == id)
        .count();
    assert_eq!(no_leidas_producto, 0);

    limpiar_producto(&conn, id);
}
