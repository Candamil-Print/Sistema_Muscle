use serde::{Deserialize, Serialize};

/// Estados de una notificación
pub const ESTADO_NO_LEIDA: i32 = 0;
pub const ESTADO_LEIDA: i32 = 1;
pub const ESTADO_ATENDIDA: i32 = 2;

/// Notificación de stock bajo tal cual está en la BD (con nombre de producto en listados)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notificacion {
    pub id_notificacion: i32,
    pub id_producto: i32,
    pub nombre_producto: String,
    pub mensaje: Option<String>,
    pub stock_actual: Option<i32>,
    pub stock_minimo: Option<i32>,
    pub fecha: String,
    pub estado: i32,
}
