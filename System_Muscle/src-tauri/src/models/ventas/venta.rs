use serde::{Deserialize, Serialize};

/// Cabecera de venta tal cual está en la BD
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Venta {
    pub id_venta: i32,
    pub fecha: String,
    pub id_usuario: i32,
    pub id_caja: i32,
    pub id_turno: Option<i32>,
}

/// Línea de detalle de venta (subtotal calculado en BD)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetalleVenta {
    pub id_detalle: i32,
    pub id_venta: i32,
    pub id_producto: i32,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub metodo_pago: i32,
    pub subtotal: f64,
}

/// Línea de detalle con nombres de producto y método de pago (JOIN)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetalleVentaDetalle {
    pub id_detalle: i32,
    pub id_venta: i32,
    pub id_producto: i32,
    pub nombre_producto: String,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub metodo_pago: i32,
    pub nombre_metodo_pago: String,
    pub subtotal: f64,
}

/// Venta en listados con total y datos del usuario
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VentaResumen {
    pub id_venta: i32,
    pub fecha: String,
    pub id_usuario: i32,
    pub nombre_usuario: String,
    pub id_caja: i32,
    pub total: f64,
}

/// Resumen diario desde la vista `vista_resumen_ventas_diario`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumenVentasDiario {
    pub fecha: String,
    pub numero_ventas: i32,
    pub total_efectivo: f64,
    pub total_transferencia: f64,
    pub total_general: f64,
}

/// Línea para registrar una nueva venta
#[derive(Debug, Deserialize, Clone)]
pub struct LineaVenta {
    pub id_producto: i32,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub metodo_pago: i32,
}

/// Payload para registrar una venta (cabecera + líneas)
#[derive(Debug, Deserialize)]
pub struct NuevaVenta {
    pub id_usuario: i32,
    pub id_caja: i32,
    pub id_turno: Option<i32>,
    pub lineas: Vec<LineaVenta>,
}
