use serde::{Deserialize, Serialize};

/// Resumen diario de ventas (vista `vista_resumen_ventas_diario`)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumenVentasDiario {
    pub fecha: String,
    pub numero_ventas: i32,
    pub total_efectivo: f64,
    pub total_transferencia: f64,
    pub total_general: f64,
}

/// Totales de ventas en un rango de fechas
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumenVentasRango {
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub numero_ventas: i32,
    pub total_efectivo: f64,
    pub total_transferencia: f64,
    pub total_general: f64,
}

/// Producto con más unidades vendidas en un período
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductoMasVendido {
    pub id_producto: i32,
    pub nombre_producto: String,
    pub tipo_producto: String,
    pub cantidad_vendida: i32,
    pub total_ventas: f64,
}

/// Producto con stock bajo (vista `vista_productos_stock_bajo`)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReporteStockBajo {
    pub id_producto: i32,
    pub nombre: String,
    pub tipo_producto: String,
    pub stock_actual: i32,
    pub stock_maximo: i32,
    pub stock_minimo: i32,
    pub porcentaje_stock: f64,
}

/// Snapshot de inventario actual
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReporteInventario {
    pub id_producto: i32,
    pub nombre: String,
    pub tipo_producto: String,
    pub stock_actual: i32,
    pub stock_maximo: i32,
    pub stock_minimo: i32,
    pub precio_costo: f64,
    pub precio_sugerido: f64,
}

/// Entradas de inventario por producto en un rango
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReporteEntradasProducto {
    pub id_producto: i32,
    pub nombre_producto: String,
    pub cantidad_ingresada: i32,
    pub numero_movimientos: i32,
}

/// Ventas agrupadas por usuario en un rango
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VentasPorUsuario {
    pub id_usuario: i32,
    pub nombre_usuario: String,
    pub numero_ventas: i32,
    pub total_vendido: f64,
}

/// Ventas agrupadas por método de pago en un rango
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VentasPorMetodoPago {
    pub id_metodo: i32,
    pub nombre_metodo: String,
    pub cantidad_lineas: i32,
    pub total: f64,
}

/// Resumen de cajas cerradas/abiertas en un rango
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReporteCaja {
    pub id_caja: i32,
    pub fecha_apertura: String,
    pub fecha_cierre: Option<String>,
    pub estado: String,
    pub monto_apertura: f64,
    pub monto_cierre: Option<f64>,
    pub total_efectivo: f64,
    pub total_transferencia: f64,
    pub nombre_usuario_apertura: String,
}

/// KPIs generales del día actual
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DashboardResumen {
    pub fecha: String,
    pub ventas_hoy: i32,
    pub total_vendido_hoy: f64,
    pub productos_stock_bajo: i32,
    pub notificaciones_pendientes: i32,
    pub entradas_hoy: i32,
}
