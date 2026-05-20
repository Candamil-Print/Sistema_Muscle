use serde::{Deserialize, Serialize};

/// Credenciales de acceso al sistema
#[derive(Debug, Deserialize)]
pub struct CredencialesLogin {
    pub documento: String,
    pub password: String,
}

/// Usuario autenticado con datos de sesión (sin contraseña)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SesionUsuario {
    pub id_usuario: i32,
    pub nombre_completo: String,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub correo: Option<String>,
    pub telefono: Option<String>,
    pub id_rol: i32,
    pub nombre_rol: String,
    pub estado: i32,
}
