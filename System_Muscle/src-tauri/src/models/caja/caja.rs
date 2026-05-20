use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Caja {
    pub id_caja: i32,
    pub fecha_apertura: String,
    pub fecha_cierre: Option<String>,
    pub monto_apertura: f64,
    pub monto_cierre: Option<f64>,
    pub total_efectivo: f64,
    pub total_transferencia: f64,
    pub estado: String,
    pub id_usuario_apertura: i32,
    pub id_usuario_cierre: Option<i32>,
    pub id_turno: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct NuevaCaja {
    pub monto_apertura: f64,
    pub id_usuario_apertura: i32,
    pub id_turno: i32,
}

#[derive(Debug, Deserialize)]
pub struct CierreCaja {
    pub id_caja: i32,
    pub monto_cierre: f64,
    pub total_efectivo: f64,
    pub total_transferencia: f64,
    pub id_usuario_cierre: i32,
}