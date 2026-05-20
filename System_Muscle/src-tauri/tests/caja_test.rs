//! Tests del módulo de caja
//! Ejecutar con: cargo test --test caja_test -- --nocapture

use system_muscle_lib::services::db::connection::get_db_connection;
use system_muscle_lib::commands::caja::logic::*;
use system_muscle_lib::models::caja::caja::{NuevaCaja, CierreCaja};

/// Limpiar cajas de prueba (solo las que no sean la primera)
fn limpiar_cajas_prueba(conn: &rusqlite::Connection) {
    let _ = conn.execute("DELETE FROM caja WHERE id_caja > 1", []);
}

/// Obtener un turno activo o crear uno de prueba
fn obtener_turno_prueba(conn: &rusqlite::Connection) -> i32 {
    // Intentar obtener un turno activo
    let mut stmt = conn.prepare("SELECT id_turno FROM turnos WHERE estado = 'ABIERTO' LIMIT 1")
        .unwrap();
    let turno: Result<i32, _> = stmt.query_row([], |row| row.get(0));
    
    if let Ok(id) = turno {
        return id;
    }
    
    // Si no hay turno activo, crear uno de prueba
    conn.execute(
        "INSERT INTO turnos (id_usuario, id_tipo_turno, fecha_inicio, estado) 
         VALUES (1, 1, CURRENT_TIMESTAMP, 'ABIERTO')",
        []
    ).unwrap();
    
    conn.last_insert_rowid() as i32
}

// ==============================================
// TESTS DE CAJA
// ==============================================

#[test]
fn test_abrir_caja() {
    println!("\n💰 TEST: Abrir caja");
    let conn = get_db_connection().unwrap();
    
    limpiar_cajas_prueba(&conn);
    let id_turno = obtener_turno_prueba(&conn);
    
    let nueva = NuevaCaja {
        monto_apertura: 500000.0,
        id_usuario_apertura: 1,
        id_turno,
    };
    
    let resultado = abrir_caja_logic(&conn, &nueva);
    
    match resultado {
        Ok(id) => {
            println!("   ✅ Caja abierta con ID: {}", id);
            assert!(id > 0);
        }
        Err(e) => panic!("❌ Error al abrir caja: {}", e),
    }
}

#[test]
fn test_obtener_caja_activa() {
    println!("\n🔍 TEST: Obtener caja activa");
    let conn = get_db_connection().unwrap();
    
    // Asegurar que hay una caja abierta
    limpiar_cajas_prueba(&conn);
    let id_turno = obtener_turno_prueba(&conn);
    
    let nueva = NuevaCaja {
        monto_apertura: 500000.0,
        id_usuario_apertura: 1,
        id_turno,
    };
    let _ = abrir_caja_logic(&conn, &nueva);
    
    let resultado = obtener_caja_activa_logic(&conn);
    
    match resultado {
        Ok(Some(caja)) => {
            println!("   ✅ Caja activa encontrada:");
            println!("      ID: {}", caja.id_caja);
            println!("      Monto apertura: ${}", caja.monto_apertura);
            println!("      Estado: {}", caja.estado);
            assert_eq!(caja.estado, "ABIERTA");
        }
        Ok(None) => panic!("❌ No se encontró caja activa"),
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_no_abrir_doble_caja() {
    println!("\n🚫 TEST: No abrir segunda caja si ya hay una abierta");
    let conn = get_db_connection().unwrap();
    
    limpiar_cajas_prueba(&conn);
    let id_turno = obtener_turno_prueba(&conn);
    
    // Abrir primera caja
    let primera = NuevaCaja {
        monto_apertura: 500000.0,
        id_usuario_apertura: 1,
        id_turno,
    };
    let _ = abrir_caja_logic(&conn, &primera).unwrap();
    
    // Intentar abrir segunda
    let segunda = NuevaCaja {
        monto_apertura: 300000.0,
        id_usuario_apertura: 1,
        id_turno,
    };
    let resultado = abrir_caja_logic(&conn, &segunda);
    
    match resultado {
        Err(e) => {
            println!("   ✅ Error esperado: {}", e);
            assert!(e.contains("Ya existe una caja abierta"));
        }
        Ok(_) => panic!("❌ No debería permitir abrir segunda caja"),
    }
}

#[test]
fn test_listar_cajas() {
    println!("\n📋 TEST: Listar cajas");
    let conn = get_db_connection().unwrap();
    
    let resultado = listar_cajas_logic(&conn, false);
    
    match resultado {
        Ok(cajas) => {
            println!("   📊 Total cajas: {}", cajas.len());
            for caja in &cajas {
                println!("      ID: {} | Estado: {} | Apertura: ${}", 
                    caja.id_caja, caja.estado, caja.monto_apertura);
            }
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_cerrar_caja() {
    println!("\n🔒 TEST: Cerrar caja");
    let conn = get_db_connection().unwrap();
    
    limpiar_cajas_prueba(&conn);
    let id_turno = obtener_turno_prueba(&conn);
    
    // Abrir caja
    let nueva = NuevaCaja {
        monto_apertura: 500000.0,
        id_usuario_apertura: 1,
        id_turno,
    };
    let id_caja = abrir_caja_logic(&conn, &nueva).unwrap();
    println!("   📌 Caja abierta con ID: {}", id_caja);
    
    // Cerrar caja
    let cierre = CierreCaja {
        id_caja,
        monto_cierre: 750000.0,
        total_efectivo: 650000.0,
        total_transferencia: 100000.0,
        id_usuario_cierre: 1,
    };
    
    let resultado = cerrar_caja_logic(&conn, &cierre);
    
    match resultado {
        Ok(()) => {
            println!("   ✅ Caja cerrada correctamente");
            
            // Verificar estado
            let caja = obtener_caja_logic(&conn, id_caja).unwrap();
            assert_eq!(caja.estado, "CERRADA");
            assert_eq!(caja.monto_cierre, Some(750000.0));
            assert_eq!(caja.total_efectivo, 650000.0);
            assert_eq!(caja.total_transferencia, 100000.0);
            println!("   ✅ Verificación: Estado = {}", caja.estado);
        }
        Err(e) => panic!("❌ Error al cerrar caja: {}", e),
    }
}