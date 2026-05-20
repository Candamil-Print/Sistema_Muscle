//! Tests del módulo de movimientos_entrada
//! Ejecutar con: cargo test --test movimientos_entrada_test -- --nocapture

use system_muscle_lib::services::db::connection::get_db_connection;
use system_muscle_lib::commands::*;
use system_muscle_lib::models::movimientos_entrada::movimiento_entrada::NuevoMovimientoEntrada;
use system_muscle_lib::models::productos::producto::NuevoProducto;
use std::time::{SystemTime, UNIX_EPOCH};

/// Crea un producto de prueba y devuelve su id_producto
fn crear_producto_con_stock(conn: &rusqlite::Connection, nombre: &str, stock_maximo: i32) -> i32 {
    let _ = conn.execute("DELETE FROM productos WHERE nombre = ?1", [nombre]);
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

/// Limpia un producto de prueba
fn limpiar_producto(conn: &rusqlite::Connection, nombre: &str) {
    let _ = conn.execute("DELETE FROM productos WHERE nombre = ?1", [nombre]);
}

//TEST Registrar entrada
#[test]
fn test_registrar_entrada_exitosa() {
    println!("\n📥 TEST: Registrar movimiento de entrada");
    let conn = get_db_connection().unwrap();
    let id = crear_producto_con_stock(&conn, "MovTest Entrada", 100);
    let stock_antes = obtener_stock_por_producto_logic(&conn, id).unwrap();
    assert_eq!(stock_antes.stock_actual, 0);

    let entrada = NuevoMovimientoEntrada {
        id_producto: id,
        cantidad: 30,
        id_usuario: 1, // admin existente en la BD
    };
    let resultado = registrar_entrada_logic(&conn, &entrada);

    match resultado {
        Ok(id_movimiento) => {
            println!("   ✅ Movimiento registrado con ID: {}", id_movimiento);
            assert!(id_movimiento > 0);
            let stock_despues = obtener_stock_por_producto_logic(&conn, id).unwrap();
            assert_eq!(stock_despues.stock_actual, 30);
            println!("   📦 Stock actualizado por trigger: 0 → 30");
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
    limpiar_producto(&conn, "MovTest Entrada");
}

#[test]
fn test_registrar_entrada_producto_inexistente() {
    println!("\n📥 TEST: Registrar entrada con producto inexistente");
    let conn = get_db_connection().unwrap();
    let entrada = NuevoMovimientoEntrada {
        id_producto: 999999,
        cantidad: 10,
        id_usuario: 1,
    };
    let resultado = registrar_entrada_logic(&conn, &entrada);
    match resultado {
        Err(e) => {
            println!("   ✅ Error esperado: {}", e);
            assert!(e.contains("no existe") || e.contains("inactivo"));
        }
        Ok(_) => panic!("❌ No debería registrar entrada con producto inexistente"),
    }
}

#[test]
fn test_registrar_entrada_usuario_inexistente() {
    println!("\n📥 TEST: Registrar entrada con usuario inexistente");
    let conn = get_db_connection().unwrap();
    let id = crear_producto_con_stock(&conn, "MovTest Usuario Inv", 100);
    let entrada = NuevoMovimientoEntrada {
        id_producto: id,
        cantidad: 10,
        id_usuario: 999999,
    };
    let resultado = registrar_entrada_logic(&conn, &entrada);
    match resultado {
        Err(e) => {
            println!("   ✅ Error esperado: {}", e);
            assert!(e.contains("usuario"));
        }
        Ok(_) => panic!("❌ No debería registrar entrada con usuario inexistente"),
    }
    limpiar_producto(&conn, "MovTest Usuario Inv");
}

#[test]
fn test_registrar_entrada_cantidad_invalida() {
    println!("\n📥 TEST: Registrar entrada con cantidad 0 o negativa");
    let conn = get_db_connection().unwrap();
    let id = crear_producto_con_stock(&conn, "MovTest Cantidad Inv", 100);
    let entrada = NuevoMovimientoEntrada {
        id_producto: id,
        cantidad: 0,
        id_usuario: 1,
    };
    let resultado = registrar_entrada_logic(&conn, &entrada);
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("mayor a cero"));
    limpiar_producto(&conn, "MovTest Cantidad Inv");
}

#[test]
fn test_multiples_entradas_acumulan_stock() {
    println!("\n📥 TEST: Múltiples entradas acumulan stock correctamente");
    let conn = get_db_connection().unwrap();
    let id = crear_producto_con_stock(&conn, "MovTest Acumulado", 200);

    for i in 1..=3 {
        let entrada = NuevoMovimientoEntrada {
            id_producto: id,
            cantidad: 20,
            id_usuario: 1,
        };
        registrar_entrada_logic(&conn, &entrada).unwrap();
        println!("   📥 Entrada #{}: +20 unidades", i);
    }
    let stock = obtener_stock_por_producto_logic(&conn, id).unwrap();
    assert_eq!(stock.stock_actual, 60);
    println!("   ✅ Stock final: {} (esperado: 60)", stock.stock_actual);
    limpiar_producto(&conn, "MovTest Acumulado");
}

//TEST Obtener movimiento
#[test]
fn test_obtener_movimiento() {
    println!("\n🔍 TEST: Obtener movimiento de entrada específico");
    let conn = get_db_connection().unwrap();
    let id = crear_producto_con_stock(&conn, "MovTest Obtener", 100);
    let entrada = NuevoMovimientoEntrada {
        id_producto: id,
        cantidad: 25,
        id_usuario: 1,
    };
    let id_mov = registrar_entrada_logic(&conn, &entrada).unwrap();
    let mov = obtener_movimiento_logic(&conn, id_mov).unwrap();
    assert_eq!(mov.cantidad, 25);
    assert_eq!(mov.id_producto, id);
    println!("   ✅ Movimiento obtenido correctamente: cantidad {}", mov.cantidad);
    limpiar_producto(&conn, "MovTest Obtener");
}

//TEST Listar movimientos
#[test]
fn test_listar_movimientos_entrada() {
    println!("\n📋 TEST: Listar todos los movimientos de entrada");
    let conn = get_db_connection().unwrap();
    let id = crear_producto_con_stock(&conn, "MovTest Listar", 100);
    let entrada = NuevoMovimientoEntrada { id_producto: id, cantidad: 15, id_usuario: 1 };
    registrar_entrada_logic(&conn, &entrada).unwrap();

    let movimientos = listar_movimientos_entrada_logic(&conn).unwrap();
    println!("   📊 Total movimientos: {}", movimientos.len());
    assert!(movimientos.iter().any(|m| m.id_producto == id));
    limpiar_producto(&conn, "MovTest Listar");
}

#[test]
fn test_movimientos_por_producto() {
    println!("\n📋 TEST: Listar movimientos de un producto específico");
    let conn = get_db_connection().unwrap();
    let id = crear_producto_con_stock(&conn, "MovTest Por Producto", 100);

    for cantidad in [10, 20, 30] {
        let entrada = NuevoMovimientoEntrada { id_producto: id, cantidad, id_usuario: 1 };
        registrar_entrada_logic(&conn, &entrada).unwrap();
    }
    let movimientos = movimientos_por_producto_logic(&conn, id).unwrap();
    println!("   📊 Movimientos del producto: {}", movimientos.len());
    assert_eq!(movimientos.len(), 3);
    assert!(movimientos.iter().all(|m| m.id_producto == id));
    let total: i32 = movimientos.iter().map(|m| m.cantidad).sum();
    assert_eq!(total, 60);
    limpiar_producto(&conn, "MovTest Por Producto");
}

#[test]
fn test_movimientos_por_usuario() {
    println!("\n👤 TEST: Listar movimientos de un usuario específico");
    let conn = get_db_connection().unwrap();
    let id = crear_producto_con_stock(&conn, "MovTest Por Usuario", 100);
    let id_usuario = 1;

    let entrada = NuevoMovimientoEntrada { id_producto: id, cantidad: 50, id_usuario };
    registrar_entrada_logic(&conn, &entrada).unwrap();

    let movimientos = movimientos_por_usuario_logic(&conn, id_usuario).unwrap();
    assert!(movimientos.len() >= 1);
    assert!(movimientos.iter().any(|m| m.id_producto == id && m.cantidad == 50));
    println!("   ✅ Movimiento encontrado por usuario");
    limpiar_producto(&conn, "MovTest Por Usuario");
}

#[test]
fn test_total_entradas_por_producto() {
    println!("\n➕ TEST: Total de entradas por producto");
    let conn = get_db_connection().unwrap();
    let id = crear_producto_con_stock(&conn, "MovTest Total", 100);

    for cantidad in [5, 15, 20] {
        let entrada = NuevoMovimientoEntrada { id_producto: id, cantidad, id_usuario: 1 };
        registrar_entrada_logic(&conn, &entrada).unwrap();
    }
    let total = total_entradas_por_producto_logic(&conn, id).unwrap();
    assert_eq!(total, 40);
    println!("   ✅ Total calculado correctamente: {}", total);
    limpiar_producto(&conn, "MovTest Total");
}
