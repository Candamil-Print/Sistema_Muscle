use serde::{Serialize, Deserialize};

/// Struct stock tal cual está en la BD
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stock {
    pub id_stock: i32,
    pub id_producto: i32,
    pub stock_actual: i32,
    pub stock_maximo: i32,
    pub stock_minimo: i32,
    pub fecha_actualizacion: String,
}

/// Struct stock con nombre del producto (JOIN con productos)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockConProducto {
    pub id_stock: i32,
    pub id_producto: i32,
    pub nombre_producto: String,
    pub tipo_producto: String,
    pub stock_actual: i32,
    pub stock_maximo: i32,
    pub stock_minimo: i32,
    pub fecha_actualizacion: String,
}

/// Struct para ajuste directo del stock_actual
#[derive(Debug, Deserialize)]
pub struct AjusteStock {
    pub stock_actual: i32,
    pub stock_maximo: Option<i32>,
}

/// Struct movimiento de entrada tal cual está en la BD
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovimientoEntrada {
    pub id_movimiento: i32,
    pub id_producto: i32,
    pub cantidad: i32,
    pub fecha: String,
    pub id_usuario: i32,
}

/// Struct movimiento de entrada con nombre del producto y usuario
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovimientoEntradaDetalle {
    pub id_movimiento: i32,
    pub id_producto: i32,
    pub nombre_producto: String,
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

/// Struct notificación de stock bajo
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notificacion {
    pub id_notificacion: i32,
    pub id_producto: i32,
    pub nombre_producto: String,
    pub mensaje: Option<String>,
    pub stock_actual: Option<i32>,
    pub stock_minimo: Option<i32>,
    pub fecha: String,
    pub estado: i32,   // 0: no leída, 1: leída, 2: atendida
}

/// Struct para el resultado de la vista vista_productos_stock_bajo
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductoStockBajo {
    pub id_producto: i32,
    pub nombre: String,
    pub tipo_producto: String,
    pub stock_actual: i32,
    pub stock_maximo: i32,
    pub stock_minimo: i32,
    pub porcentaje_stock: f64,
}
