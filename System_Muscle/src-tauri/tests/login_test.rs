//! Tests del módulo de login
//! Ejecutar con: cargo test --test login_test -- --nocapture

use system_muscle_lib::commands::login::logic::{login_logic, verificar_sesion_logic};
use system_muscle_lib::commands::usuarios::logic::{
    crear_usuario_logic, deshabilitar_usuario_logic,
};
use system_muscle_lib::models::login::login::CredencialesLogin;
use system_muscle_lib::models::usuarios::usuario::NuevoUsuario;
use system_muscle_lib::services::db::connection::get_db_connection;

fn limpiar_usuario_prueba(conn: &rusqlite::Connection, documento: &str) {
    let _ = conn.execute(
        "DELETE FROM usuarios WHERE numero_documento = ?1",
        [documento],
    );
}

#[test]
fn test_login_admin_exitoso() {
    println!("\n🔐 TEST: Login administrador");
    let conn = get_db_connection().unwrap();

    let credenciales = CredencialesLogin {
        documento: "1000000000".to_string(),
        password: "adminsystemmuscle".to_string(),
    };

    let resultado = login_logic(&conn, &credenciales);

    match resultado {
        Ok(Some(sesion)) => {
            println!("   ✅ Login exitoso: {}", sesion.nombre_completo);
            println!("   📌 Rol: {} ({})", sesion.nombre_rol, sesion.id_rol);
            assert_eq!(sesion.id_rol, 1);
            assert_eq!(sesion.estado, 1);
            assert!(!sesion.nombre_rol.is_empty());
        }
        Ok(None) => panic!("❌ Credenciales inválidas"),
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_login_credenciales_incorrectas() {
    println!("\n🔐 TEST: Contraseña incorrecta");
    let conn = get_db_connection().unwrap();

    let credenciales = CredencialesLogin {
        documento: "1000000000".to_string(),
        password: "password_falso".to_string(),
    };

    match login_logic(&conn, &credenciales) {
        Ok(None) => println!("   ✅ Login rechazado"),
        Ok(Some(_)) => panic!("❌ No debería autenticar"),
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_login_usuario_inexistente() {
    println!("\n🔐 TEST: Usuario inexistente");
    let conn = get_db_connection().unwrap();

    let credenciales = CredencialesLogin {
        documento: "999999999".to_string(),
        password: "cualquier".to_string(),
    };

    match login_logic(&conn, &credenciales) {
        Ok(None) => println!("   ✅ Usuario no encontrado"),
        Ok(Some(_)) => panic!("❌ No debería encontrar usuario"),
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_login_documento_vacio() {
    println!("\n🔐 TEST: Documento vacío");
    let conn = get_db_connection().unwrap();

    let credenciales = CredencialesLogin {
        documento: "   ".to_string(),
        password: "algo".to_string(),
    };

    match login_logic(&conn, &credenciales) {
        Err(e) => {
            println!("   ✅ Error esperado: {}", e);
            assert!(e.contains("documento"));
        }
        Ok(_) => panic!("❌ Debería validar documento vacío"),
    }
}

#[test]
fn test_login_usuario_deshabilitado() {
    println!("\n🚫 TEST: Usuario deshabilitado");
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

    let credenciales = CredencialesLogin {
        documento: "BLOQUEADO123".to_string(),
        password: "pass123".to_string(),
    };

    match login_logic(&conn, &credenciales) {
        Ok(None) => println!("   ✅ Login rechazado"),
        Ok(Some(_)) => panic!("❌ No debería permitir login"),
        Err(e) => panic!("❌ Error: {}", e),
    }

    limpiar_usuario_prueba(&conn, "BLOQUEADO123");
}

#[test]
fn test_verificar_sesion_activa() {
    println!("\n🔍 TEST: Verificar sesión activa");
    let conn = get_db_connection().unwrap();

    match verificar_sesion_logic(&conn, 1) {
        Ok(Some(sesion)) => {
            println!("   ✅ Sesión válida: {}", sesion.nombre_completo);
            assert_eq!(sesion.id_usuario, 1);
        }
        Ok(None) => panic!("❌ Admin debería estar activo"),
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_verificar_sesion_inexistente() {
    println!("\n🔍 TEST: Verificar sesión inexistente");
    let conn = get_db_connection().unwrap();

    match verificar_sesion_logic(&conn, 99999) {
        Ok(None) => println!("   ✅ Sin sesión"),
        Ok(Some(_)) => panic!("❌ No debería existir"),
        Err(e) => panic!("❌ Error: {}", e),
    }
}
