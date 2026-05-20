//! Tests del módulo de productos
//! Ejecutar con: cargo test --test productos_test -- --nocapture

use system_muscle_lib::services::db::connection::get_db_connection;
use system_muscle_lib::commands::*;
use system_muscle_lib::models::productos::producto::{NuevoProducto, ModificarProducto};
use std::time::{SystemTime, UNIX_EPOCH};


/// Limpia un producto de prueba (y su stock por CASCADE) dado su nombre
fn limpiar_producto_prueba(conn: &rusqlite::Connection, nombre: &str) {
    let _ = conn.execute(
        "DELETE FROM productos WHERE nombre = ?1",
        [nombre],
    );
}

/// Crea un producto de prueba y devuelve su id_producto
fn crear_producto_prueba(conn: &rusqlite::Connection) -> i32 {
    // Limpiar primero para evitar UNIQUE constraint de nombre
    limpiar_producto_prueba(conn, "Producto Test");

    let nuevo = NuevoProducto {
        nombre: "Producto Test".to_string(),
        tipo_producto: "SNACKS".to_string(),
        precio_costo: 5000.0,
        precio_sugerido: 8000.0,
        imagen_url: None,
        stock_maximo: 100,
    };

    crear_producto_logic(conn, &nuevo).unwrap()
}

#[test]
fn test_crear_producto_exitoso() {
    println!("\n➕ TEST: Crear producto exitosamente");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Whey Protein Gold");

    let nuevo = NuevoProducto {
        nombre: "Whey Protein Gold".to_string(),
        tipo_producto: "SUPLEMENTOS".to_string(),
        precio_costo: 80000.0,
        precio_sugerido: 120000.0,
        imagen_url: Some("https://example.com/whey.jpg".to_string()),
        stock_maximo: 50,
    };

    let resultado = crear_producto_logic(&conn, &nuevo);

    match resultado {
        Ok(id) => {
            println!("   ✅ Producto creado con ID: {}", id);
            assert!(id > 0);

            // Verificar que el producto existe con stock
            let producto = obtener_producto_logic(&conn, id).unwrap();
            assert_eq!(producto.nombre, "Whey Protein Gold");
            assert_eq!(producto.tipo_producto, "SUPLEMENTOS");
            assert_eq!(producto.precio_costo, 80000.0);
            assert_eq!(producto.precio_sugerido, 120000.0);
            assert_eq!(producto.activo, 1);
            assert_eq!(producto.stock_actual, Some(0));
            assert_eq!(producto.stock_maximo, Some(50));
            println!("   ✅ Stock inicial creado correctamente");

            limpiar_producto_prueba(&conn, "Whey Protein Gold");
        }
        Err(e) => panic!("❌ Error al crear producto: {}", e),
    }
}

#[test]
fn test_crear_producto_nombre_duplicado() {
    println!("\n⚠️ TEST: Crear producto con nombre duplicado");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Duplicado Test");

    let nuevo1 = NuevoProducto {
        nombre: "Producto Duplicado Test".to_string(),
        tipo_producto: "BEBIDAS".to_string(),
        precio_costo: 3000.0,
        precio_sugerido: 5000.0,
        imagen_url: None,
        stock_maximo: 200,
    };
    let _ = crear_producto_logic(&conn, &nuevo1).unwrap();

    let nuevo2 = NuevoProducto {
        nombre: "Producto Duplicado Test".to_string(),
        tipo_producto: "SNACKS".to_string(),
        precio_costo: 4000.0,
        precio_sugerido: 6000.0,
        imagen_url: None,
        stock_maximo: 100,
    };

    let resultado = crear_producto_logic(&conn, &nuevo2);

    match resultado {
        Err(e) => {
            println!("   ✅ Error esperado: {}", e);
            assert!(e.contains("Ya existe"));
        }
        Ok(_) => panic!("❌ No debería permitir nombres duplicados"),
    }

    limpiar_producto_prueba(&conn, "Producto Duplicado Test");
}

#[test]
fn test_crear_producto_todos_los_tipos() {
    println!("\n➕ TEST: Crear un producto de cada tipo");
    let conn = get_db_connection().unwrap();

    let tipos = vec![
        ("Snack de Prueba", "SNACKS", 2000.0, 3500.0, 80),
        ("Suplemento de Prueba", "SUPLEMENTOS", 50000.0, 75000.0, 30),
        ("Bebida de Prueba", "BEBIDAS", 1500.0, 2500.0, 150),
    ];

    for (nombre, tipo, costo, sugerido, stock_max) in &tipos {
        limpiar_producto_prueba(&conn, nombre);

        let nuevo = NuevoProducto {
            nombre: nombre.to_string(),
            tipo_producto: tipo.to_string(),
            precio_costo: *costo,
            precio_sugerido: *sugerido,
            imagen_url: None,
            stock_maximo: *stock_max,
        };

        let id = crear_producto_logic(&conn, &nuevo).unwrap();
        println!("   ✅ Creado {} con ID: {}", nombre, id);
        assert!(id > 0);

        limpiar_producto_prueba(&conn, nombre);
    }
}

//TEST obtener producto
#[test]
fn test_obtener_producto_existente() {
    println!("\n🔍 TEST: Obtener producto existente");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Test");
    let id = crear_producto_prueba(&conn);

    let resultado = obtener_producto_logic(&conn, id);

    match resultado {
        Ok(producto) => {
            println!("   ✅ Producto encontrado: {}", producto.nombre);
            assert_eq!(producto.id_producto, id);
            assert_eq!(producto.nombre, "Producto Test");
            assert!(producto.stock_actual.is_some());
            println!("   📦 Stock actual: {:?}", producto.stock_actual);
        }
        Err(e) => panic!("❌ Error: {}", e),
    }

    limpiar_producto_prueba(&conn, "Producto Test");
}

#[test]
fn test_obtener_producto_inexistente() {
    println!("\n🔍 TEST: Obtener producto inexistente");
    let conn = get_db_connection().unwrap();

    let resultado = obtener_producto_logic(&conn, 999999);

    match resultado {
        Err(e) => println!("   ✅ Error esperado: {}", e),
        Ok(_) => panic!("❌ No debería encontrar el producto"),
    }
}

//TEST Listar productos
#[test]
fn test_listar_productos_activos() {
    println!("\n📋 TEST: Listar productos activos");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Test");
    crear_producto_prueba(&conn);

    let productos = listar_productos_logic(&conn, true).unwrap();

    println!("   📊 Total productos activos: {}", productos.len());
    for p in &productos {
        println!("   🏷️  - {} [{}]", p.nombre, p.tipo_producto);
        assert_eq!(p.activo, 1);
    }

    assert!(productos.len() >= 1);

    limpiar_producto_prueba(&conn, "Producto Test");
}

#[test]
fn test_listar_todos_los_productos() {
    println!("\n📋 TEST: Listar todos los productos (activos e inactivos)");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Test");
    let id = crear_producto_prueba(&conn);
    desactivar_producto_logic(&conn, id).unwrap();

    let todos = listar_productos_logic(&conn, false).unwrap();
    let activos: Vec<_> = todos.iter().filter(|p| p.activo == 1).collect();
    let inactivos: Vec<_> = todos.iter().filter(|p| p.activo == 0).collect();

    println!("   ✅ Activos: {}", activos.len());
    println!("   ⚪ Inactivos: {}", inactivos.len());
    assert!(inactivos.len() >= 1);

    limpiar_producto_prueba(&conn, "Producto Test");
}

//TEST Buscar productos
#[test]
fn test_buscar_productos_por_nombre() {
    println!("\n🔎 TEST: Buscar productos por nombre");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Creatina Monohidrato");

    let nuevo = NuevoProducto {
        nombre: "Creatina Monohidrato".to_string(),
        tipo_producto: "SUPLEMENTOS".to_string(),
        precio_costo: 45000.0,
        precio_sugerido: 65000.0,
        imagen_url: None,
        stock_maximo: 40,
    };
    crear_producto_logic(&conn, &nuevo).unwrap();

    let resultados = buscar_productos_logic(&conn, "Creatina", true).unwrap();

    println!("   📊 Encontrados: {}", resultados.len());
    assert!(resultados.iter().any(|p| p.nombre == "Creatina Monohidrato"));
    println!("   ✅ Producto encontrado en la búsqueda");

    limpiar_producto_prueba(&conn, "Creatina Monohidrato");
}

#[test]
fn test_buscar_productos_por_tipo() {
    println!("\n🔎 TEST: Buscar productos por tipo");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Test");
    crear_producto_prueba(&conn); // tipo: SNACKS

    let resultados = buscar_productos_logic(&conn, "SNACKS", true).unwrap();

    println!("   📊 Encontrados con tipo SNACKS: {}", resultados.len());
    assert!(resultados.iter().all(|p| p.tipo_producto == "SNACKS"));
    println!("   ✅ Búsqueda por tipo correcta");

    limpiar_producto_prueba(&conn, "Producto Test");
}

#[test]
fn test_buscar_producto_sin_resultados() {
    println!("\n🔎 TEST: Buscar producto que no existe");
    let conn = get_db_connection().unwrap();

    let resultados = buscar_productos_logic(&conn, "XYZ_NO_EXISTE_NUNCA", true).unwrap();

    println!("   ✅ Resultados vacíos: {}", resultados.len());
    assert_eq!(resultados.len(), 0);
}

//TEST Modificar producto
#[test]
fn test_modificar_producto_exitoso() {
    println!("\n✏️ TEST: Modificar producto");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Test");
    let id = crear_producto_prueba(&conn);

    let datos = ModificarProducto {
        nombre: Some("Producto Test Modificado".to_string()),
        tipo_producto: Some("BEBIDAS".to_string()),
        precio_costo: Some(7000.0),
        precio_sugerido: Some(11000.0),
        imagen_url: Some("https://example.com/nuevo.jpg".to_string()),
        stock_maximo: Some(200),
    };

    let resultado = modificar_producto_logic(&conn, id, &datos);

    match resultado {
        Ok(()) => {
            let modificado = obtener_producto_logic(&conn, id).unwrap();
            assert_eq!(modificado.nombre, "Producto Test Modificado");
            assert_eq!(modificado.tipo_producto, "BEBIDAS");
            assert_eq!(modificado.precio_costo, 7000.0);
            assert_eq!(modificado.precio_sugerido, 11000.0);
            assert_eq!(modificado.stock_maximo, Some(200));
            println!("   ✅ Producto modificado correctamente");

            limpiar_producto_prueba(&conn, "Producto Test Modificado");
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_modificar_producto_campos_parciales() {
    println!("\n✏️ TEST: Modificar solo precio_sugerido");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Test");
    let id = crear_producto_prueba(&conn);
    let original = obtener_producto_logic(&conn, id).unwrap();

    let datos = ModificarProducto {
        nombre: None,
        tipo_producto: None,
        precio_costo: None,
        precio_sugerido: Some(9999.0),
        imagen_url: None,
        stock_maximo: None,
    };

    modificar_producto_logic(&conn, id, &datos).unwrap();

    let modificado = obtener_producto_logic(&conn, id).unwrap();
    assert_eq!(modificado.nombre, original.nombre);
    assert_eq!(modificado.tipo_producto, original.tipo_producto);
    assert_eq!(modificado.precio_costo, original.precio_costo);
    assert_eq!(modificado.precio_sugerido, 9999.0);
    println!("   ✅ Solo se modificó el precio sugerido");

    limpiar_producto_prueba(&conn, "Producto Test");
}

#[test]
fn test_modificar_stock_maximo() {
    println!("\n✏️ TEST: Modificar stock máximo");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Test");
    let id = crear_producto_prueba(&conn);

    // stock_maximo inicial = 100
    let antes = obtener_producto_logic(&conn, id).unwrap();
    assert_eq!(antes.stock_maximo, Some(100));

    let datos = ModificarProducto {
        nombre: None,
        tipo_producto: None,
        precio_costo: None,
        precio_sugerido: None,
        imagen_url: None,
        stock_maximo: Some(500),
    };

    modificar_producto_logic(&conn, id, &datos).unwrap();

    let despues = obtener_producto_logic(&conn, id).unwrap();
    assert_eq!(despues.stock_maximo, Some(500));
    println!("   ✅ Stock máximo actualizado de 100 a 500");

    limpiar_producto_prueba(&conn, "Producto Test");
}

//TEST Activar/Desactivar
#[test]
fn test_desactivar_producto() {
    println!("\n🔴 TEST: Desactivar producto");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Test");
    let id = crear_producto_prueba(&conn);

    let antes = obtener_producto_logic(&conn, id).unwrap();
    assert_eq!(antes.activo, 1);
    println!("   ✅ Producto inicialmente ACTIVO");

    desactivar_producto_logic(&conn, id).unwrap();

    let despues = obtener_producto_logic(&conn, id).unwrap();
    assert_eq!(despues.activo, 0);
    println!("   ✅ Producto desactivado correctamente");

    limpiar_producto_prueba(&conn, "Producto Test");
}

#[test]
fn test_activar_producto() {
    println!("\n🟢 TEST: Activar producto");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Test");
    let id = crear_producto_prueba(&conn);
    desactivar_producto_logic(&conn, id).unwrap();
    println!("   ⚪ Producto desactivado primero");

    activar_producto_logic(&conn, id).unwrap();

    let habilitado = obtener_producto_logic(&conn, id).unwrap();
    assert_eq!(habilitado.activo, 1);
    println!("   ✅ Producto activado correctamente");

    limpiar_producto_prueba(&conn, "Producto Test");
}

#[test]
fn test_producto_desactivado_no_aparece_en_activos() {
    println!("\n🚫 TEST: Producto desactivado no aparece en listado de activos");
    let conn = get_db_connection().unwrap();

    limpiar_producto_prueba(&conn, "Producto Test");
    let id = crear_producto_prueba(&conn);
    desactivar_producto_logic(&conn, id).unwrap();

    let activos = listar_productos_logic(&conn, true).unwrap();
    let encontrado = activos.iter().any(|p| p.id_producto == id);

    assert!(!encontrado, "❌ El producto desactivado no debería aparecer");
    println!("   ✅ Producto desactivado excluido correctamente del listado de activos");

    limpiar_producto_prueba(&conn, "Producto Test");
}

//TEST Integración Flujo completo
#[test]
fn test_flujo_completo_producto() {
    println!("\n🔄 TEST: Flujo completo de producto");
    let conn = get_db_connection().unwrap();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let nombre_unico = format!("Producto Flujo {}", timestamp);

    // 1. CREAR
    println!("   1️⃣ Creando producto...");
    let nuevo = NuevoProducto {
        nombre: nombre_unico.clone(),
        tipo_producto: "SUPLEMENTOS".to_string(),
        precio_costo: 60000.0,
        precio_sugerido: 90000.0,
        imagen_url: None,
        stock_maximo: 80,
    };
    let id = crear_producto_logic(&conn, &nuevo).unwrap();
    println!("      ✅ Creado con ID: {}", id);

    // 2. VERIFICAR STOCK INICIAL
    println!("   2️⃣ Verificando stock inicial...");
    let p = obtener_producto_logic(&conn, id).unwrap();
    assert_eq!(p.stock_actual, Some(0));
    assert_eq!(p.stock_maximo, Some(80));
    println!("      ✅ Stock inicial: 0 / 80");

    // 3. MODIFICAR PRECIOS
    println!("   3️⃣ Modificando precios...");
    let modificacion = ModificarProducto {
        nombre: None,
        tipo_producto: None,
        precio_costo: Some(65000.0),
        precio_sugerido: Some(95000.0),
        imagen_url: None,
        stock_maximo: Some(120),
    };
    modificar_producto_logic(&conn, id, &modificacion).unwrap();
    println!("      ✅ Precios y stock máximo modificados");

    // 4. DESACTIVAR
    println!("   4️⃣ Desactivando producto...");
    desactivar_producto_logic(&conn, id).unwrap();
    println!("      ✅ Desactivado");

    // 5. ACTIVAR
    println!("   5️⃣ Activando producto...");
    activar_producto_logic(&conn, id).unwrap();
    println!("      ✅ Activado");

    // 6. BUSCAR
    println!("   6️⃣ Buscando por nombre...");
    let busqueda = buscar_productos_logic(&conn, "Producto Flujo", true).unwrap();
    assert!(busqueda.iter().any(|p| p.id_producto == id));
    println!("      ✅ Encontrado en búsqueda");

    // 7. VERIFICAR ESTADO FINAL
    let final_p = obtener_producto_logic(&conn, id).unwrap();
    println!("   7️⃣ Estado final:");
    println!("      🏷️  Nombre: {}", final_p.nombre);
    println!("      💰 Precio costo: {}", final_p.precio_costo);
    println!("      💵 Precio sugerido: {}", final_p.precio_sugerido);
    println!("      📦 Stock máximo: {:?}", final_p.stock_maximo);
    println!("      🔘 Activo: {}", if final_p.activo == 1 { "SÍ" } else { "NO" });

    assert_eq!(final_p.precio_costo, 65000.0);
    assert_eq!(final_p.precio_sugerido, 95000.0);
    assert_eq!(final_p.stock_maximo, Some(120));
    assert_eq!(final_p.activo, 1);

    println!("\n   ✅ FLUJO COMPLETO EXITOSO");

    limpiar_producto_prueba(&conn, &nombre_unico);
}
