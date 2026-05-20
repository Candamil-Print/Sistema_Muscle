use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistorialAccion {
    pub id_historial: i32,
    pub id_usuario: i32,
    pub accion: String,
    pub tabla_afectada: Option<String>,
    pub id_registro_afectado: Option<i32>,
    pub descripcion: Option<String>,
    pub fecha: String,
    pub hora: String,
    pub id_turno: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct NuevaAccion {
    pub id_usuario: i32,
    pub accion: String,
    pub tabla_afectada: Option<String>,
    pub id_registro_afectado: Option<i32>,
    pub descripcion: Option<String>,
    pub id_turno: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct FiltroHistorial {
    pub id_usuario: Option<i32>,
    pub accion: Option<String>,
    pub fecha_desde: Option<String>,
    pub fecha_hasta: Option<String>,
    pub id_turno: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistorialAccionDetalle {
    pub id_historial: i32,
    pub usuario: String,
    pub accion: String,
    pub tabla_afectada: Option<String>,
    pub id_registro_afectado: Option<i32>,
    pub descripcion: Option<String>,
    pub fecha: String,
    pub hora: String,
    pub turno: Option<String>,
}