//! Tests del módulo de stock
//! Ejecutar con: cargo test --test stock_test -- --nocapture

use system_muscle_lib::services::db::connection::get_db_connection;
use system_muscle_lib::commands::*;
use system_muscle_lib::models::stock::stock::{AjusteStock, NuevoMovimientoEntrada};
use system_muscle_lib::models::productos::producto::NuevoProducto;
use std::time::{SystemTime, UNIX_EPOCH};

/// Crea un producto de prueba con stock y devuelve su id_producto
fn crear_producto_con_stock(conn: &rusqlite::Connection, nombre: &str, stock_maximo: i32) -> i32 {
    // Limpiar si ya existe
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

/// Limpia un producto de prueba (y su stock/movimientos por CASCADE) dado su nombre
fn limpiar_producto(conn: &rusqlite::Connection, nombre: &str) {
    let _ = conn.execute("DELETE FROM productos WHERE nombre = ?1", [nombre]);
}

//TEST obtener stock
#[test]
fn test_obtener_stock_por_producto() {
    println!("\n📦 TEST: Obtener stock de un producto");
    let conn = get_db_connection().unwrap();

    let id = crear_producto_con_stock(&conn, "Stock Test Obtener", 100);

    let resultado = obtener_stock_por_producto_logic(&conn, id);

    match resultado {
        Ok(stock) => {
            println!("   ✅ Stock encontrado");
            println!("   📊 Stock actual: {}", stock.stock_actual);
            println!("   📈 Stock máximo: {}", stock.stock_maximo);
            println!("   📉 Stock mínimo: {}", stock.stock_minimo);
            assert_eq!(stock.id_producto, id);
            assert_eq!(stock.stock_actual, 0);
            assert_eq!(stock.stock_maximo, 100);
            assert_eq!(stock.stock_minimo, 25); // 25% de 100
        }
        Err(e) => panic!("❌ Error: {}", e),
    }

    limpiar_producto(&conn, "Stock Test Obtener");
}

#[test]
fn test_obtener_stock_producto_inexistente() {
    println!("\n📦 TEST: Obtener stock de producto inexistente");
    let conn = get_db_connection().unwrap();

    let resultado = obtener_stock_por_producto_logic(&conn, 999999);

    match resultado {
        Err(e) => println!("   ✅ Error esperado: {}", e),
        Ok(_) => panic!("❌ No debería encontrar stock"),
    }
}

//TEST Listar stock
#[test]
fn test_listar_stock_activos() {
    println!("\n📋 TEST: Listar stock de productos activos");
    let conn = get_db_connection().unwrap();

    let id = crear_producto_con_stock(&conn, "Stock Test Listar", 50);

    let lista = listar_stock_logic(&conn).unwrap();

    println!("   📊 Total registros de stock: {}", lista.len());
    assert!(lista.iter().any(|s| s.id_producto == id));

    for item in &lista {
        println!("   🏷️  {} - actual: {} / máx: {}", item.nombre_producto, item.stock_actual, item.stock_maximo);
    }

    limpiar_producto(&conn, "Stock Test Listar");
}

//TEST Ajustar stock
#[test]
fn test_ajustar_stock_actual() {
    println!("\n🔧 TEST: Ajustar stock_actual directamente");
    let conn = get_db_connection().unwrap();

    let id = crear_producto_con_stock(&conn, "Stock Test Ajustar", 200);

    let ajuste = AjusteStock {
        stock_actual: 80,
        stock_maximo: None,
    };

    ajustar_stock_logic(&conn, id, &ajuste).unwrap();

    let stock = obtener_stock_por_producto_logic(&conn, id).unwrap();
    assert_eq!(stock.stock_actual, 80);
    assert_eq!(stock.stock_maximo, 200); // no cambia
    println!("   ✅ Stock actual ajustado a 80, máximo sigue en 200");

    limpiar_producto(&conn, "Stock Test Ajustar");
}

#[test]
fn test_ajustar_stock_actual_y_maximo() {
    println!("\n🔧 TEST: Ajustar stock_actual y stock_maximo");
    let conn = get_db_connection().unwrap();

    let id = crear_producto_con_stock(&conn, "Stock Test Ajustar Max", 100);

    let ajuste = AjusteStock {
        stock_actual: 150,
        stock_maximo: Some(300),
    };

    ajustar_stock_logic(&conn, id, &ajuste).unwrap();

    let stock = obtener_stock_por_producto_logic(&conn, id).unwrap();
    assert_eq!(stock.stock_actual, 150);
    assert_eq!(stock.stock_maximo, 300);
    assert_eq!(stock.stock_minimo, 75); // 25% de 300
    println!("   ✅ Stock actual=150, máximo=300, mínimo=75");

    limpiar_producto(&conn, "Stock Test Ajustar Max");
}

//TEST Registrar entrada
#[test]
fn test_registrar_entrada_exitosa() {
    println!("\n📥 TEST: Registrar movimiento de entrada");
    let conn = get_db_connection().unwrap();

    let id = crear_producto_con_stock(&conn, "Stock Test Entrada", 100);
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

            // El trigger debe haber actualizado el stock
            let stock_despues = obtener_stock_por_producto_logic(&conn, id).unwrap();
            assert_eq!(stock_despues.stock_actual, 30);
            println!("   📦 Stock actualizado por trigger: 0 → 30");
        }
        Err(e) => panic!("❌ Error: {}", e),
    }

    limpiar_producto(&conn, "Stock Test Entrada");
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

    let id = crear_producto_con_stock(&conn, "Stock Test Usuario Inv", 100);

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

    limpiar_producto(&conn, "Stock Test Usuario Inv");
}

#[test]
fn test_multiples_entradas_acumulan_stock() {
    println!("\n📥 TEST: Múltiples entradas acumulan stock correctamente");
    let conn = get_db_connection().unwrap();

    let id = crear_producto_con_stock(&conn, "Stock Test Acumulado", 200);

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
    assert_eq!(stock.stock_actual, 60); // 3 × 20
    println!("   ✅ Stock final: {} (esperado: 60)", stock.stock_actual);

    limpiar_producto(&conn, "Stock Test Acumulado");
}

//TEST Listar movimientos
#[test]
fn test_listar_movimientos_entrada() {
    println!("\n📋 TEST: Listar todos los movimientos de entrada");
    let conn = get_db_connection().unwrap();

    let id = crear_producto_con_stock(&conn, "Stock Test Movimientos", 100);

    let entrada = NuevoMovimientoEntrada {
        id_producto: id,
        cantidad: 15,
        id_usuario: 1,
    };
    registrar_entrada_logic(&conn, &entrada).unwrap();

    let movimientos = listar_movimientos_entrada_logic(&conn).unwrap();

    println!("   📊 Total movimientos: {}", movimientos.len());
    assert!(movimientos.iter().any(|m| m.id_producto == id));

    for m in movimientos.iter().take(3) {
        println!("   📥 {} → {} unidades por {}", m.nombre_producto, m.cantidad, m.nombre_usuario);
    }

    limpiar_producto(&conn, "Stock Test Movimientos");
}

#[test]
fn test_movimientos_por_producto() {
    println!("\n📋 TEST: Listar movimientos de un producto específico");
    let conn = get_db_connection().unwrap();

    let id = crear_producto_con_stock(&conn, "Stock Test Mov Producto", 100);

    for cantidad in [10, 20, 30] {
        let entrada = NuevoMovimientoEntrada {
            id_producto: id,
            cantidad,
            id_usuario: 1,
        };
        registrar_entrada_logic(&conn, &entrada).unwrap();
    }

    let movimientos = movimientos_por_producto_logic(&conn, id).unwrap();

    println!("   📊 Movimientos del producto: {}", movimientos.len());
    assert_eq!(movimientos.len(), 3);
    assert!(movimientos.iter().all(|m| m.id_producto == id));

    let total: i32 = movimientos.iter().map(|m| m.cantidad).sum();
    assert_eq!(total, 60); // 10 + 20 + 30
    println!("   ✅ 3 movimientos, total acumulado: {}", total);

    limpiar_producto(&conn, "Stock Test Mov Producto");
}

//TEST Stock bajo
#[test]
fn test_listar_stock_bajo() {
    println!("\n⚠️ TEST: Listar productos con stock bajo");
    let conn = get_db_connection().unwrap();

    // Crear producto con stock máximo 100 → mínimo = 25
    let id = crear_producto_con_stock(&conn, "Stock Test Bajo", 100);

    // Ajustar stock a 10 (por debajo del mínimo de 25)
    let ajuste = AjusteStock { stock_actual: 10, stock_maximo: None };
    ajustar_stock_logic(&conn, id, &ajuste).unwrap();

    let bajo = listar_stock_bajo_logic(&conn).unwrap();

    println!("   📊 Productos con stock bajo: {}", bajo.len());
    assert!(bajo.iter().any(|p| p.id_producto == id));

    for p in &bajo {
        println!("   ⚠️  {} - {}% (actual: {}/mínimo: {})",
            p.nombre, p.porcentaje_stock, p.stock_actual, p.stock_minimo);
        assert!(p.stock_actual < p.stock_minimo);
    }

    limpiar_producto(&conn, "Stock Test Bajo");
}

#[test]
fn test_producto_con_stock_suficiente_no_aparece_en_stock_bajo() {
    println!("\n✅ TEST: Producto con stock suficiente no aparece en stock bajo");
    let conn = get_db_connection().unwrap();

    let id = crear_producto_con_stock(&conn, "Stock Test Suficiente", 100);

    // Ajustar a 80 (por encima del mínimo de 25)
    let ajuste = AjusteStock { stock_actual: 80, stock_maximo: None };
    ajustar_stock_logic(&conn, id, &ajuste).unwrap();

    let bajo = listar_stock_bajo_logic(&conn).unwrap();

    let encontrado = bajo.iter().any(|p| p.id_producto == id);
    assert!(!encontrado);
    println!("   ✅ Producto con stock 80/100 no aparece en la lista de stock bajo");

    limpiar_producto(&conn, "Stock Test Suficiente");
}

//TEST Notificaciones
#[test]
fn test_notificacion_se_genera_al_bajar_stock() {
    println!("\n🔔 TEST: Verificar que se genera notificación al bajar el stock");
    let conn = get_db_connection().unwrap();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap().as_secs();
    let nombre = format!("Stock Test Notif {}", timestamp);

    let id = crear_producto_con_stock(&conn, &nombre, 100);

    // Ajustar a 10 → debería disparar el trigger de notificación
    let ajuste = AjusteStock { stock_actual: 10, stock_maximo: None };
    ajustar_stock_logic(&conn, id, &ajuste).unwrap();

    let notificaciones = listar_notificaciones_logic(&conn, false).unwrap();
    let tiene_notif = notificaciones.iter().any(|n| n.id_producto == id);

    assert!(tiene_notif, "❌ Debería existir una notificación de stock bajo");
    println!("   ✅ Notificación generada automáticamente por el trigger");

    limpiar_producto(&conn, &nombre);
}

#[test]
fn test_marcar_notificacion_como_leida() {
    println!("\n🔔 TEST: Marcar notificación como leída");
    let conn = get_db_connection().unwrap();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap().as_secs();
    let nombre = format!("Stock Test Leida {}", timestamp);

    let id = crear_producto_con_stock(&conn, &nombre, 100);
    let ajuste = AjusteStock { stock_actual: 5, stock_maximo: None };
    ajustar_stock_logic(&conn, id, &ajuste).unwrap();

    // Obtener la notificación generada
    let notificaciones = listar_notificaciones_logic(&conn, true).unwrap();
    let notif = notificaciones.iter().find(|n| n.id_producto == id);

    assert!(notif.is_some(), "❌ Debería haber notificación no leída");
    let id_notif = notif.unwrap().id_notificacion;

    // Marcar como leída
    marcar_notificacion_logic(&conn, id_notif, 1).unwrap();

    // No debería aparecer en no leídas
    let no_leidas = listar_notificaciones_logic(&conn, true).unwrap();
    assert!(!no_leidas.iter().any(|n| n.id_notificacion == id_notif));
    println!("   ✅ Notificación marcada como leída y excluida del listado de no leídas");

    limpiar_producto(&conn, &nombre);
}

#[test]
fn test_marcar_notificacion_como_atendida() {
    println!("\n🔔 TEST: Marcar notificación como atendida");
    let conn = get_db_connection().unwrap();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap().as_secs();
    let nombre = format!("Stock Test Atendida {}", timestamp);

    let id = crear_producto_con_stock(&conn, &nombre, 100);
    let ajuste = AjusteStock { stock_actual: 3, stock_maximo: None };
    ajustar_stock_logic(&conn, id, &ajuste).unwrap();

    let notificaciones = listar_notificaciones_logic(&conn, false).unwrap();
    let notif = notificaciones.iter().find(|n| n.id_producto == id).unwrap();
    let id_notif = notif.id_notificacion;

    marcar_notificacion_logic(&conn, id_notif, 2).unwrap();

    // Verificar que el estado cambió a 2
    let todas = listar_notificaciones_logic(&conn, false).unwrap();
    let actualizada = todas.iter().find(|n| n.id_notificacion == id_notif).unwrap();
    assert_eq!(actualizada.estado, 2);
    println!("   ✅ Notificación marcada como atendida (estado=2)");

    limpiar_producto(&conn, &nombre);
}

#[test]
fn test_marcar_notificacion_estado_invalido() {
    println!("\n🔔 TEST: Estado inválido al marcar notificación");
    let conn = get_db_connection().unwrap();

    let resultado = marcar_notificacion_logic(&conn, 1, 99);

    match resultado {
        Err(e) => {
            println!("   ✅ Error esperado: {}", e);
            assert!(e.contains("inválido") || e.contains("inv\u{00e1}lido"));
        }
        Ok(_) => panic!("❌ No debería aceptar estado 99"),
    }
}

//TEST Integración Flujo completo
#[test]
fn test_flujo_completo_stock() {
    println!("\n🔄 TEST: Flujo completo del módulo de stock");
    let conn = get_db_connection().unwrap();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap().as_secs();
    let nombre = format!("Stock Flujo {}", timestamp);

    // 1. CREAR PRODUCTO CON STOCK
    println!("   1️⃣ Creando producto con stock...");
    let id = crear_producto_con_stock(&conn, &nombre, 120);
    println!("      ✅ Producto creado con stock_maximo=120");

    // 2. VERIFICAR STOCK INICIAL
    println!("   2️⃣ Verificando stock inicial...");
    let s = obtener_stock_por_producto_logic(&conn, id).unwrap();
    assert_eq!(s.stock_actual, 0);
    assert_eq!(s.stock_maximo, 120);
    assert_eq!(s.stock_minimo, 30); // 25% de 120
    println!("      ✅ Stock: 0/120 (mínimo: 30)");

    // 3. REGISTRAR ENTRADAS
    println!("   3️⃣ Registrando entradas...");
    for cantidad in [40, 30] {
        registrar_entrada_logic(&conn, &NuevoMovimientoEntrada {
            id_producto: id, cantidad, id_usuario: 1,
        }).unwrap();
    }
    let s = obtener_stock_por_producto_logic(&conn, id).unwrap();
    assert_eq!(s.stock_actual, 70);
    println!("      ✅ Stock después de entradas: 70");

    // 4. AJUSTAR STOCK (corrección manual)
    println!("   4️⃣ Ajuste manual a 25 (debajo del mínimo)...");
    ajustar_stock_logic(&conn, id, &AjusteStock { stock_actual: 25, stock_maximo: None }).unwrap();
    let s = obtener_stock_por_producto_logic(&conn, id).unwrap();
    assert_eq!(s.stock_actual, 25);
    println!("      ✅ Stock ajustado a 25 → por debajo del mínimo (30)");

    // 5. VERIFICAR ALERTA DE STOCK BAJO
    println!("   5️⃣ Verificando alerta de stock bajo...");
    let bajo = listar_stock_bajo_logic(&conn).unwrap();
    assert!(bajo.iter().any(|p| p.id_producto == id));
    println!("      ✅ Producto aparece en lista de stock bajo");

    // 6. VERIFICAR NOTIFICACIÓN
    println!("   6️⃣ Verificando notificaciones...");
    let notifs = listar_notificaciones_logic(&conn, true).unwrap();
    let notif = notifs.iter().find(|n| n.id_producto == id);
    assert!(notif.is_some());
    let id_notif = notif.unwrap().id_notificacion;
    println!("      ✅ Notificación encontrada (estado: no leída)");

    // 7. GESTIONAR NOTIFICACIÓN
    println!("   7️⃣ Marcando notificación como atendida...");
    marcar_notificacion_logic(&conn, id_notif, 2).unwrap();
    let no_leidas = listar_notificaciones_logic(&conn, true).unwrap();
    assert!(!no_leidas.iter().any(|n| n.id_notificacion == id_notif));
    println!("      ✅ Notificación atendida y excluida del listado");

    // 8. VERIFICAR MOVIMIENTOS
    println!("   8️⃣ Verificando historial de movimientos...");
    let movs = movimientos_por_producto_logic(&conn, id).unwrap();
    assert_eq!(movs.len(), 2);
    println!("      ✅ 2 movimientos de entrada registrados");

    println!("\n   ✅ FLUJO COMPLETO DE STOCK EXITOSO");

    limpiar_producto(&conn, &nombre);
}
