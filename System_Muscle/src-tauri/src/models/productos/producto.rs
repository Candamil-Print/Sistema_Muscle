use serde::{Serialize, Deserialize};

/// Struct producto tal cual está en la BD
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Producto {
    pub id_producto: i32,
    pub nombre: String,
    pub tipo_producto: String,
    pub precio_costo: f64,
    pub precio_sugerido: f64,
    pub imagen_url: Option<String>,
    pub fecha_creacion: String,
    pub activo: i32,
}

/// Struct producto con datos de su stock
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductoConStock {
    pub id_producto: i32,
    pub nombre: String,
    pub tipo_producto: String,
    pub precio_costo: f64,
    pub precio_sugerido: f64,
    pub imagen_url: Option<String>,
    pub fecha_creacion: String,
    pub activo: i32,
    // Datos de stock (pueden ser None si aún no se creó el registro)
    pub stock_actual: Option<i32>,
    pub stock_maximo: Option<i32>,
    pub stock_minimo: Option<i32>,
    pub fecha_actualizacion_stock: Option<String>,
}

/// Struct de creación
#[derive(Debug, Deserialize)]
pub struct NuevoProducto {
    pub nombre: String,
    pub tipo_producto: String,   // 'SNACKS', 'SUPLEMENTOS', 'BEBIDAS'
    pub precio_costo: f64,
    pub precio_sugerido: f64,
    pub imagen_url: Option<String>,
    pub stock_maximo: i32,       // requerido al crear: define el stock máximo inicial
}

/// Struct de modificación
#[derive(Debug, Deserialize)]
pub struct ModificarProducto {
    pub nombre: Option<String>,
    pub tipo_producto: Option<String>,
    pub precio_costo: Option<f64>,
    pub precio_sugerido: Option<f64>,
    pub imagen_url: Option<String>,
    pub stock_maximo: Option<i32>,  // si se cambia, actualiza también la tabla stock
}
