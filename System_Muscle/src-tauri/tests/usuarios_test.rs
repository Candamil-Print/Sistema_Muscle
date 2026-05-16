//! Tests del módulo de usuarios
//! Ejecutar con: cargo test --test usuarios -- --nocapture

use system_muscle_lib::services::db::connection::get_db_connection;
use system_muscle_lib::commands::*;
use system_muscle_lib::models::usuarios::usuario::{NuevoUsuario, UsuarioModificacion};
use std::time::{SystemTime, UNIX_EPOCH};

/// Helper para limpiar usuarios de prueba
fn limpiar_usuario_prueba(conn: &rusqlite::Connection, documento: &str) {
    let _ = conn.execute(
        "DELETE FROM usuarios WHERE numero_documento = ?1",
        [documento]
    );
}

/// Helper para crear usuario de prueba y devolver su ID
fn crear_usuario_prueba(conn: &rusqlite::Connection) -> i32 {
    // Limpiar primero para evitar UNIQUE constraint
    let _ = conn.execute("DELETE FROM usuarios WHERE numero_documento = 'TEST999'", []);
    
    let nuevo = NuevoUsuario {
        nombre_completo: "Usuario Test".to_string(),
        tipo_documento: "CC".to_string(),
        numero_documento: "TEST999".to_string(),
        direccion: Some("Calle Test 123".to_string()),
        tipo_sangre: Some("O+".to_string()),
        eps: Some("SURA".to_string()),
        genero: Some("MASCULINO".to_string()),
        correo: Some("test@test.com".to_string()),
        telefono: Some("3001234567".to_string()),
        password: "test123".to_string(),
    };
    
    crear_usuario_logic(conn, &nuevo).unwrap()
}

// ==============================================
// TESTS DE LOGIN
// ==============================================

#[test]
fn test_login_con_admin_exitoso() {
    println!("\n🔐 TEST: Login con administrador");
    let conn = get_db_connection().unwrap();
    
    // Usar la contraseña real que está hasheada en la BD
    let resultado = login_logic(&conn, "1000000000", "adminsystemmuscle");
    
    match resultado {
        Ok(Some(usuario)) => {
            println!("   ✅ Login exitoso");
            println!("   👤 Usuario: {}", usuario.nombre_completo);
            println!("   📌 Rol: {}", usuario.id_rol);
            assert_eq!(usuario.id_rol, 1);
            assert_eq!(usuario.estado, 1);
        }
        Ok(None) => panic!("❌ Login falló - Credenciales inválidas"),
        Err(e) => panic!("❌ Error en login: {}", e),
    }
}

#[test]
fn test_login_con_credenciales_incorrectas() {
    println!("\n🔐 TEST: Login con credenciales incorrectas");
    let conn = get_db_connection().unwrap();
    
    let resultado = login_logic(&conn, "1000000000", "password_falso");
    
    match resultado {
        Ok(None) => println!("   ✅ Login rechazado correctamente"),
        Ok(Some(_)) => panic!("❌ Debería haber fallado el login"),
        Err(e) => panic!("❌ Error en login: {}", e),
    }
}

#[test]
fn test_login_con_usuario_inexistente() {
    println!("\n🔐 TEST: Login con usuario inexistente");
    let conn = get_db_connection().unwrap();
    
    let resultado = login_logic(&conn, "999999999", "cualquier");
    
    match resultado {
        Ok(None) => println!("   ✅ Usuario no encontrado correctamente"),
        Ok(Some(_)) => panic!("❌ No debería encontrar un usuario inexistente"),
        Err(e) => panic!("❌ Error en login: {}", e),
    }
}

// ==============================================
// TESTS DE CREAR USUARIO
// ==============================================

#[test]
fn test_crear_usuario_exitoso() {
    println!("\n➕ TEST: Crear usuario exitosamente");
    let conn = get_db_connection().unwrap();
    
    limpiar_usuario_prueba(&conn, "123456789");
    
    let nuevo = NuevoUsuario {
        nombre_completo: "María González".to_string(),
        tipo_documento: "CC".to_string(),
        numero_documento: "123456789".to_string(),
        direccion: Some("Carrera 45 #23-12".to_string()),
        tipo_sangre: Some("A+".to_string()),
        eps: Some("SANITAS".to_string()),
        genero: Some("FEMENINO".to_string()),
        correo: Some("maria@test.com".to_string()),
        telefono: Some("3112223344".to_string()),
        password: "maria123".to_string(),
    };
    
    let resultado = crear_usuario_logic(&conn, &nuevo);
    
    match resultado {
        Ok(id) => {
            println!("   ✅ Usuario creado con ID: {}", id);
            assert!(id > 0);
            
            let usuario = obtener_usuario_logic(&conn, id).unwrap();
            assert_eq!(usuario.nombre_completo, "María González");
            assert_eq!(usuario.id_rol, 2);
            assert_eq!(usuario.estado, 1);
            
            limpiar_usuario_prueba(&conn, "123456789");
        }
        Err(e) => panic!("❌ Error al crear usuario: {}", e),
    }
}

#[test]
fn test_crear_usuario_con_documento_duplicado() {
    println!("\n⚠️ TEST: Crear usuario con documento duplicado");
    let conn = get_db_connection().unwrap();
    
    limpiar_usuario_prueba(&conn, "DUPLICADO123");
    
    let nuevo1 = NuevoUsuario {
        nombre_completo: "Usuario Original".to_string(),
        tipo_documento: "CC".to_string(),
        numero_documento: "DUPLICADO123".to_string(),
        direccion: None,
        tipo_sangre: Some("O+".to_string()),
        eps: Some("SURA".to_string()),
        genero: Some("MASCULINO".to_string()),
        correo: None,
        telefono: None,
        password: "pass123".to_string(),
    };
    
    let _ = crear_usuario_logic(&conn, &nuevo1).unwrap();
    
    let nuevo2 = NuevoUsuario {
        nombre_completo: "Usuario Duplicado".to_string(),
        tipo_documento: "CC".to_string(),
        numero_documento: "DUPLICADO123".to_string(),
        direccion: None,
        tipo_sangre: Some("A+".to_string()),
        eps: Some("SANITAS".to_string()),
        genero: Some("FEMENINO".to_string()),
        correo: None,
        telefono: None,
        password: "pass456".to_string(),
    };
    
    let resultado = crear_usuario_logic(&conn, &nuevo2);
    
    match resultado {
        Err(e) => {
            println!("   ✅ Error esperado: {}", e);
            assert!(e.contains("Ya existe") || e.contains("duplicado"));
        }
        Ok(_) => panic!("❌ No debería permitir documento duplicado"),
    }
    
    limpiar_usuario_prueba(&conn, "DUPLICADO123");
}

// ==============================================
// TESTS DE LISTAR USUARIOS
// ==============================================

#[test]
fn test_listar_usuarios_activos() {
    println!("\n📋 TEST: Listar usuarios activos");
    let conn = get_db_connection().unwrap();
    
    let usuarios = listar_usuarios_logic(&conn, true).unwrap();
    
    println!("   📊 Total usuarios activos: {}", usuarios.len());
    
    for user in &usuarios {
        println!("   👤 - {}", user.nombre_completo);
        assert_eq!(user.estado, 1);
    }
    
    assert!(usuarios.len() >= 1);
}

#[test]
fn test_listar_todos_usuarios() {
    println!("\n📋 TEST: Listar todos los usuarios");
    let conn = get_db_connection().unwrap();
    
    limpiar_usuario_prueba(&conn, "DESHAB999");
    
    let nuevo = NuevoUsuario {
        nombre_completo: "Usuario Para Deshabilitar".to_string(),
        tipo_documento: "CC".to_string(),
        numero_documento: "DESHAB999".to_string(),
        direccion: None,
        tipo_sangre: Some("O+".to_string()),
        eps: Some("SURA".to_string()),
        genero: Some("MASCULINO".to_string()),
        correo: None,
        telefono: None,
        password: "pass123".to_string(),
    };
    
    let id = crear_usuario_logic(&conn, &nuevo).unwrap();
    deshabilitar_usuario_logic(&conn, id).unwrap();
    
    let todos = listar_usuarios_logic(&conn, false).unwrap();
    
    let usuarios_activos: Vec<_> = todos.iter().filter(|u| u.estado == 1).collect();
    let usuarios_inactivos: Vec<_> = todos.iter().filter(|u| u.estado == 0).collect();
    
    println!("   ✅ Activos: {}", usuarios_activos.len());
    println!("   ⚪ Inactivos: {}", usuarios_inactivos.len());
    
    assert!(usuarios_inactivos.len() >= 1);
    
    limpiar_usuario_prueba(&conn, "DESHAB999");
}

// ==============================================
// TESTS DE OBTENER USUARIO
// ==============================================

#[test]
fn test_obtener_usuario_existente() {
    println!("\n🔍 TEST: Obtener usuario existente");
    let conn = get_db_connection().unwrap();
    
    let resultado = obtener_usuario_logic(&conn, 1);
    
    match resultado {
        Ok(usuario) => {
            println!("   ✅ Usuario encontrado: {}", usuario.nombre_completo);
            assert_eq!(usuario.id_usuario, 1);
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_obtener_usuario_inexistente() {
    println!("\n🔍 TEST: Obtener usuario inexistente");
    let conn = get_db_connection().unwrap();
    
    let resultado = obtener_usuario_logic(&conn, 99999);
    
    match resultado {
        Err(e) => {
            println!("   ✅ Error esperado: {}", e);
        }
        Ok(_) => panic!("❌ No debería encontrar el usuario"),
    }
}

// ==============================================
// TESTS DE MODIFICAR USUARIO
// ==============================================

#[test]
fn test_modificar_usuario_exitoso() {
    println!("\n✏️ TEST: Modificar usuario");
    let conn = get_db_connection().unwrap();
    
    limpiar_usuario_prueba(&conn, "TEST999");
    let id = crear_usuario_prueba(&conn);
    
    let modificacion = UsuarioModificacion {
        direccion: Some("Nueva Dirección".to_string()),
        telefono: Some("999888777".to_string()),
        correo: Some("nuevo@test.com".to_string()),
    };
    
    let resultado = modificar_usuario_logic(&conn, id, &modificacion);
    
    match resultado {
        Ok(()) => {
            let modificado = obtener_usuario_logic(&conn, id).unwrap();
            assert_eq!(modificado.direccion, Some("Nueva Dirección".to_string()));
            assert_eq!(modificado.telefono, Some("999888777".to_string()));
            assert_eq!(modificado.correo, Some("nuevo@test.com".to_string()));
            println!("   ✅ Usuario modificado correctamente");
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
    
    limpiar_usuario_prueba(&conn, "TEST999");
}

#[test]
fn test_modificar_usuario_campos_parciales() {
    println!("\n✏️ TEST: Modificar solo teléfono");
    let conn = get_db_connection().unwrap();
    
    limpiar_usuario_prueba(&conn, "TEST999");
    let id = crear_usuario_prueba(&conn);
    let original = obtener_usuario_logic(&conn, id).unwrap();
    
    let modificacion = UsuarioModificacion {
        direccion: None,
        telefono: Some("SOLO_TELEFONO".to_string()),
        correo: None,
    };
    
    modificar_usuario_logic(&conn, id, &modificacion).unwrap();
    
    let modificado = obtener_usuario_logic(&conn, id).unwrap();
    
    assert_eq!(modificado.direccion, original.direccion);
    assert_eq!(modificado.telefono, Some("SOLO_TELEFONO".to_string()));
    assert_eq!(modificado.correo, original.correo);
    
    println!("   ✅ Solo se modificó el teléfono");
    
    limpiar_usuario_prueba(&conn, "TEST999");
}

// ==============================================
// TESTS DE HABILITAR/DESHABILITAR
// ==============================================

#[test]
fn test_deshabilitar_usuario() {
    println!("\n🔴 TEST: Deshabilitar usuario");
    let conn = get_db_connection().unwrap();
    
    limpiar_usuario_prueba(&conn, "TEST999");
    let id = crear_usuario_prueba(&conn);
    
    let usuario = obtener_usuario_logic(&conn, id).unwrap();
    assert_eq!(usuario.estado, 1);
    println!("   ✅ Usuario inicialmente ACTIVO");
    
    deshabilitar_usuario_logic(&conn, id).unwrap();
    
    let deshabilitado = obtener_usuario_logic(&conn, id).unwrap();
    assert_eq!(deshabilitado.estado, 0);
    println!("   ✅ Usuario deshabilitado");
    
    limpiar_usuario_prueba(&conn, "TEST999");
}

#[test]
fn test_habilitar_usuario() {
    println!("\n🟢 TEST: Habilitar usuario");
    let conn = get_db_connection().unwrap();
    
    limpiar_usuario_prueba(&conn, "TEST999");
    let id = crear_usuario_prueba(&conn);
    deshabilitar_usuario_logic(&conn, id).unwrap();
    println!("   ⚪ Usuario deshabilitado primero");
    
    habilitar_usuario_logic(&conn, id).unwrap();
    
    let habilitado = obtener_usuario_logic(&conn, id).unwrap();
    assert_eq!(habilitado.estado, 1);
    println!("   ✅ Usuario habilitado");
    
    limpiar_usuario_prueba(&conn, "TEST999");
}

#[test]
fn test_login_usuario_deshabilitado() {
    println!("\n🚫 TEST: Login con usuario deshabilitado");
    let conn = get_db_connection().unwrap();
    
    limpiar_usuario_prueba(&conn, "BLOQUEADO123");
    
    let nuevo = NuevoUsuario {
        nombre_completo: "Usuario Bloqueado".to_string(),
        tipo_documento: "CC".to_string(),
        numero_documento: "BLOQUEADO123".to_string(),
        direccion: None,
        tipo_sangre: Some("O+".to_string()),
        eps: Some("SURA".to_string()),
        genero: Some("MASCULINO".to_string()),
        correo: None,
        telefono: None,
        password: "pass123".to_string(),
    };
    
    let id = crear_usuario_logic(&conn, &nuevo).unwrap();
    deshabilitar_usuario_logic(&conn, id).unwrap();
    
    let resultado = login_logic(&conn, "BLOQUEADO123", "pass123");
    
    match resultado {
        Ok(None) => println!("   ✅ Login rechazado (usuario deshabilitado)"),
        Ok(Some(_)) => panic!("❌ No debería permitir login"),
        Err(e) => panic!("❌ Error: {}", e),
    }
    
    limpiar_usuario_prueba(&conn, "BLOQUEADO123");
}

// ==============================================
// TEST DE INTEGRACIÓN - FLUJO COMPLETO
// ==============================================

#[test]
fn test_flujo_completo_usuario() {
    println!("\n🔄 TEST: Flujo completo de usuario");
    let conn = get_db_connection().unwrap();
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let documento_unico = format!("FLUJO{}", timestamp);
    
    // 1. CREAR
    println!("   1️⃣ Creando usuario...");
    let nuevo = NuevoUsuario {
        nombre_completo: "Flujo Completo".to_string(),
        tipo_documento: "CC".to_string(),
        numero_documento: documento_unico.clone(),
        direccion: Some("Dirección Original".to_string()),
        tipo_sangre: Some("B+".to_string()),
        eps: Some("COOMEVA".to_string()),
        genero: Some("OTRO".to_string()),
        correo: Some("original@test.com".to_string()),
        telefono: Some("111111111".to_string()),
        password: "flujo123".to_string(),
    };
    
    let id = crear_usuario_logic(&conn, &nuevo).unwrap();
    println!("      ✅ Creado con ID: {}", id);
    
    // 2. MODIFICAR
    println!("   2️⃣ Modificando usuario...");
    let modificacion = UsuarioModificacion {
        direccion: Some("Dirección Modificada".to_string()),
        telefono: Some("999999999".to_string()),
        correo: Some("modificado@test.com".to_string()),
    };
    modificar_usuario_logic(&conn, id, &modificacion).unwrap();
    println!("      ✅ Modificado");
    
    // 3. DESHABILITAR
    println!("   3️⃣ Deshabilitando usuario...");
    deshabilitar_usuario_logic(&conn, id).unwrap();
    println!("      ✅ Deshabilitado");
    
    // 4. HABILITAR
    println!("   4️⃣ Habilitando usuario...");
    habilitar_usuario_logic(&conn, id).unwrap();
    println!("      ✅ Habilitado");
    
    // 5. VERIFICAR
    let final_usuario = obtener_usuario_logic(&conn, id).unwrap();
    println!("   5️⃣ Verificando estado final:");
    println!("      📛 Nombre: {}", final_usuario.nombre_completo);
    println!("      📍 Dirección: {:?}", final_usuario.direccion);
    println!("      📧 Correo: {:?}", final_usuario.correo);
    println!("      📞 Teléfono: {:?}", final_usuario.telefono);
    println!("      🔘 Estado: {}", if final_usuario.estado == 1 { "ACTIVO" } else { "INACTIVO" });
    
    assert_eq!(final_usuario.direccion, Some("Dirección Modificada".to_string()));
    assert_eq!(final_usuario.estado, 1);
    
    println!("\n   ✅ FLUJO COMPLETO EXITOSO");
    
    limpiar_usuario_prueba(&conn, &documento_unico);
}