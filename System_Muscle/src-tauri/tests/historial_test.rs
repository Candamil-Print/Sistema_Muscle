//! Tests del módulo de historial
//! Ejecutar con: cargo test --test historial_test -- --nocapture

use system_muscle_lib::services::db::connection::get_db_connection;
use system_muscle_lib::commands::historial::logic::*;
use system_muscle_lib::models::historial::historial::{NuevaAccion, FiltroHistorial};

/// Limpiar historial de prueba
fn limpiar_historial_prueba(conn: &rusqlite::Connection) {
    let _ = conn.execute("DELETE FROM historial_acciones WHERE id_historial > 1", []);
}

// ==============================================
// TESTS DE HISTORIAL
// ==============================================

#[test]
fn test_registrar_accion() {
    println!("\n📝 TEST: Registrar acción en historial");
    let conn = get_db_connection().unwrap();
    
    limpiar_historial_prueba(&conn);
    
    let nueva = NuevaAccion {
        id_usuario: 1,
        accion: "LOGIN".to_string(),
        tabla_afectada: Some("usuarios".to_string()),
        id_registro_afectado: Some(1),
        descripcion: Some("Usuario administrador inició sesión".to_string()),
        id_turno: Some(1),
    };
    
    let resultado = registrar_accion_logic(&conn, &nueva);
    
    match resultado {
        Ok(id) => {
            println!("   ✅ Acción registrada con ID: {}", id);
            assert!(id > 0);
        }
        Err(e) => panic!("❌ Error al registrar: {}", e),
    }
}

#[test]
fn test_obtener_accion() {
    println!("\n🔍 TEST: Obtener acción por ID");
    let conn = get_db_connection().unwrap();
    
    limpiar_historial_prueba(&conn);
    
    // Registrar una acción
    let nueva = NuevaAccion {
        id_usuario: 1,
        accion: "VENTA".to_string(),
        tabla_afectada: Some("ventas".to_string()),
        id_registro_afectado: Some(10),
        descripcion: Some("Venta de producto".to_string()),
        id_turno: Some(1),
    };
    let id = registrar_accion_logic(&conn, &nueva).unwrap();
    
    // Obtenerla
    let resultado = obtener_accion_logic(&conn, id);
    
    match resultado {
        Ok(accion) => {
            println!("   ✅ Acción encontrada:");
            println!("      ID: {}", accion.id_historial);
            println!("      Acción: {}", accion.accion);
            println!("      Descripción: {:?}", accion.descripcion);
            assert_eq!(accion.accion, "VENTA");
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_listar_historial() {
    println!("\n📋 TEST: Listar todo el historial");
    let conn = get_db_connection().unwrap();
    
    let filtro = FiltroHistorial {
        id_usuario: None,
        accion: None,
        fecha_desde: None,
        fecha_hasta: None,
        id_turno: None,
    };
    
    let resultado = listar_historial_logic(&conn, &filtro);
    
    match resultado {
        Ok(historial) => {
            println!("   📊 Total registros: {}", historial.len());
            for item in &historial {
                println!("      ID: {} | Acción: {} | Fecha: {}", 
                    item.id_historial, item.accion, item.fecha);
            }
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_historial_por_usuario() {
    println!("\n👤 TEST: Historial por usuario");
    let conn = get_db_connection().unwrap();
    
    let resultado = historial_por_usuario_logic(&conn, 1);
    
    match resultado {
        Ok(historial) => {
            println!("   📊 Usuario 1 tiene {} registros", historial.len());
            for item in &historial {
                println!("      Acción: {} | {}", item.accion, item.fecha);
            }
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_historial_por_accion() {
    println!("\n🏷️ TEST: Historial por tipo de acción");
    let conn = get_db_connection().unwrap();
    
    let resultado = historial_por_accion_logic(&conn, "LOGIN");
    
    match resultado {
        Ok(historial) => {
            println!("   📊 Acciones 'LOGIN': {} registros", historial.len());
            for item in &historial {
                println!("      ID: {} | Fecha: {}", item.id_historial, item.fecha);
            }
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_historial_por_rango_fechas() {
    println!("\n📅 TEST: Historial por rango de fechas");
    let conn = get_db_connection().unwrap();
    
    let resultado = historial_por_rango_fechas_logic(&conn, "2024-01-01", "2026-12-31");
    
    match resultado {
        Ok(historial) => {
            println!("   📊 Registros en el rango: {}", historial.len());
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_ultimos_historial() {
    println!("\n🆕 TEST: Últimos 5 registros del historial");
    let conn = get_db_connection().unwrap();
    
    let resultado = ultimos_historial_logic(&conn, 5);
    
    match resultado {
        Ok(historial) => {
            println!("   📊 Últimos {} registros:", historial.len());
            for item in &historial {
                println!("      [{}] {} - {}", 
                    item.fecha, item.usuario, item.accion);
            }
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_listar_historial_detalle() {
    println!("\n📋 TEST: Listar historial con detalles");
    let conn = get_db_connection().unwrap();
    
    let filtro = FiltroHistorial {
        id_usuario: None,
        accion: None,
        fecha_desde: None,
        fecha_hasta: None,
        id_turno: None,
    };
    
    let resultado = listar_historial_detalle_logic(&conn, &filtro);
    
    match resultado {
        Ok(historial) => {
            println!("   📊 Total registros con detalle: {}", historial.len());
            for item in &historial {
                println!("      ID: {} | Usuario: {} | Acción: {} | Turno: {:?}", 
                    item.id_historial, item.usuario, item.accion, item.turno);
            }
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}