use serde::{Serialize, Deserialize};

/// Struct movimiento de entrada tal cual está en la BD
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovimientoEntrada {
    pub id_movimiento: i32,
    pub id_producto: i32,
    pub cantidad: i32,
    pub fecha: String,
    pub id_usuario: i32,
}

/// Struct movimiento con nombre del producto y del usuario (JOIN)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovimientoEntradaDetalle {
    pub id_movimiento: i32,
    pub id_producto: i32,
    pub nombre_producto: String,
    pub tipo_producto: String,
    pub cantidad: i32,
    pub fecha: String,
    pub id_usuario: i32,
    pub nombre_usuario: String,
}

/// Struct para registrar un nuevo movimiento de entrada
#[derive(Debug, Deserialize)]
pub struct NuevoMovimientoEntrada {
    pub id_producto: i32,
    pub cantidad: i32,
    pub id_usuario: i32,
}
