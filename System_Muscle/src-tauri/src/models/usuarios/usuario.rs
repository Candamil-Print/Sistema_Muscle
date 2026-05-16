use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usuario {
    pub id_usuario: i32,
    pub nombre_completo: String,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub direccion: Option<String>,
    pub tipo_sangre: Option<String>,
    pub eps: Option<String>,
    pub genero: Option<String>,
    pub correo: Option<String>,
    pub telefono: Option<String>,
    pub estado: i32,
    pub id_rol: i32,
    pub fecha_creacion: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevoUsuario {
    pub nombre_completo: String,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub direccion: Option<String>,
    pub tipo_sangre: Option<String>,
    pub eps: Option<String>,
    pub genero: Option<String>,
    pub correo: Option<String>,
    pub telefono: Option<String>,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UsuarioModificacion {
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub correo: Option<String>,
}