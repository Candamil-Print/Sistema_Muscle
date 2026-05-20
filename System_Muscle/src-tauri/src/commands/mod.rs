// Modulo de usuarios
pub mod usuarios;

pub use usuarios::logic::{
    crear_usuario_logic,
    modificar_usuario_logic,
    obtener_usuario_logic,
    listar_usuarios_logic,
    habilitar_usuario_logic,
    deshabilitar_usuario_logic,
    login_logic,
};

// Modulo de productos
pub mod productos;

pub use productos::logic::{
    crear_producto_logic,
    modificar_producto_logic,
    obtener_producto_logic,
    obtener_producto_simple_logic,
    listar_productos_logic,
    buscar_productos_logic,
    activar_producto_logic,
    desactivar_producto_logic,
};

// Modulo de stock
pub mod stock;

pub use stock::logic::{
    obtener_stock_por_producto_logic,
    listar_stock_logic,
    ajustar_stock_logic,
    listar_stock_bajo_logic,
};

// Modulo de notificaciones
pub mod notificaciones;

pub use notificaciones::logic::{
    obtener_notificacion_logic,
    listar_notificaciones_logic,
    notificaciones_por_producto_logic,
    notificaciones_por_estado_logic,
    contar_no_leidas_logic,
    marcar_notificacion_logic,
    marcar_todas_leidas_logic,
};

// Modulo de movimientos de entrada
pub mod movimientos_entrada;

pub use movimientos_entrada::logic::{
    registrar_entrada_logic,
    obtener_movimiento_logic,
    listar_movimientos_entrada_logic,
    movimientos_por_producto_logic,
    movimientos_por_usuario_logic,
    movimientos_por_rango_fechas_logic,
    total_entradas_por_producto_logic,
};

// Modulo de ventas
pub mod ventas;

pub use ventas::logic::{
    registrar_venta_logic,
    obtener_venta_logic,
    listar_detalle_venta_logic,
    listar_ventas_logic,
    ventas_por_usuario_logic,
    ventas_por_caja_logic,
    ventas_por_rango_fechas_logic,
    resumen_ventas_diario_logic,
    total_ventas_por_producto_logic,
};

// Modulo de caja
pub mod caja;

pub use caja::logic::{
    abrir_caja_logic,
    cerrar_caja_logic,
    obtener_caja_logic,
    obtener_caja_activa_logic,
    listar_cajas_logic,
};

// Modulo de historial
pub mod historial;

pub use historial::logic::{
    registrar_accion_logic,
    listar_historial_logic,
    obtener_accion_logic,
    historial_por_usuario_logic,
    historial_por_turno_logic,
    historial_por_accion_logic,
    historial_por_rango_fechas_logic,
    listar_historial_detalle_logic,
    ultimos_historial_logic,
};

// Comandos de Tauri
use tauri::State;
use crate::services::db::connection::DbState;
use crate::models::usuarios::usuario::{NuevoUsuario, UsuarioModificacion};
use crate::models::productos::producto::{NuevoProducto, ModificarProducto};
use crate::models::stock::stock::AjusteStock;
use crate::models::movimientos_entrada::movimiento_entrada::NuevoMovimientoEntrada;
use crate::models::ventas::venta::NuevaVenta;
use crate::models::caja::caja::{NuevaCaja, CierreCaja};
use crate::models::historial::historial::{NuevaAccion, FiltroHistorial};

// Comandos de utilidad
#[tauri::command]
pub fn test_db_connection() -> Result<String, String> {
    match rusqlite::Connection::open("system_muscle.db") {
        Ok(conn) => {
            match conn.query_row("SELECT 'Conectado exitosamente'", [], |row| {
                row.get::<_, String>(0)
            }) {
                Ok(mensaje) => Ok(mensaje),
                Err(e) => Err(format!("Error en consulta: {}", e)),
            }
        }
        Err(e) => Err(format!("Error al abrir DB: {}", e)),
    }
}

// Comandos de usuarios
#[tauri::command]
pub fn crear_usuario(
    state: State<'_, DbState>,
    nuevo: NuevoUsuario,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    crear_usuario_logic(&conn, &nuevo)
}

#[tauri::command]
pub fn modificar_usuario(
    state: State<'_, DbState>,
    id: i32,
    modificacion: UsuarioModificacion,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    modificar_usuario_logic(&conn, id, &modificacion)
}

#[tauri::command]
pub fn obtener_usuario(
    state: State<'_, DbState>,
    id: i32,
) -> Result<crate::models::usuarios::usuario::Usuario, String> {
    let conn = state.conn.lock().unwrap();
    obtener_usuario_logic(&conn, id)
}

#[tauri::command]
pub fn listar_usuarios(
    state: State<'_, DbState>,
    solo_activos: bool,
) -> Result<Vec<crate::models::usuarios::usuario::Usuario>, String> {
    let conn = state.conn.lock().unwrap();
    listar_usuarios_logic(&conn, solo_activos)
}

#[tauri::command]
pub fn habilitar_usuario(
    state: State<'_, DbState>,
    id: i32,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    habilitar_usuario_logic(&conn, id)
}

#[tauri::command]
pub fn deshabilitar_usuario(
    state: State<'_, DbState>,
    id: i32,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    deshabilitar_usuario_logic(&conn, id)
}

#[tauri::command]
pub fn login(
    state: State<'_, DbState>,
    documento: String,
    password: String,
) -> Result<Option<crate::models::usuarios::usuario::Usuario>, String> {
    let conn = state.conn.lock().unwrap();
    login_logic(&conn, &documento, &password)
}

// Comandos de productos
#[tauri::command]
pub fn crear_producto(
    state: State<'_, DbState>,
    nuevo: NuevoProducto,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    crear_producto_logic(&conn, &nuevo)
}

#[tauri::command]
pub fn modificar_producto(
    state: State<'_, DbState>,
    id: i32,
    datos: ModificarProducto,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    modificar_producto_logic(&conn, id, &datos)
}

#[tauri::command]
pub fn obtener_producto(
    state: State<'_, DbState>,
    id: i32,
) -> Result<crate::models::productos::producto::ProductoConStock, String> {
    let conn = state.conn.lock().unwrap();
    obtener_producto_logic(&conn, id)
}

#[tauri::command]
pub fn listar_productos(
    state: State<'_, DbState>,
    solo_activos: bool,
) -> Result<Vec<crate::models::productos::producto::ProductoConStock>, String> {
    let conn = state.conn.lock().unwrap();
    listar_productos_logic(&conn, solo_activos)
}

#[tauri::command]
pub fn buscar_productos(
    state: State<'_, DbState>,
    termino: String,
    solo_activos: bool,
) -> Result<Vec<crate::models::productos::producto::ProductoConStock>, String> {
    let conn = state.conn.lock().unwrap();
    buscar_productos_logic(&conn, &termino, solo_activos)
}

#[tauri::command]
pub fn activar_producto(
    state: State<'_, DbState>,
    id: i32,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    activar_producto_logic(&conn, id)
}

#[tauri::command]
pub fn desactivar_producto(
    state: State<'_, DbState>,
    id: i32,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    desactivar_producto_logic(&conn, id)
}

// Comandos de stock
#[tauri::command]
pub fn obtener_stock_por_producto(
    state: State<'_, DbState>,
    id_producto: i32,
) -> Result<crate::models::stock::stock::Stock, String> {
    let conn = state.conn.lock().unwrap();
    obtener_stock_por_producto_logic(&conn, id_producto)
}

#[tauri::command]
pub fn listar_stock(
    state: State<'_, DbState>,
) -> Result<Vec<crate::models::stock::stock::StockConProducto>, String> {
    let conn = state.conn.lock().unwrap();
    listar_stock_logic(&conn)
}

#[tauri::command]
pub fn ajustar_stock(
    state: State<'_, DbState>,
    id_producto: i32,
    ajuste: AjusteStock,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    ajustar_stock_logic(&conn, id_producto, &ajuste)
}

#[tauri::command]
pub fn listar_stock_bajo(
    state: State<'_, DbState>,
) -> Result<Vec<crate::models::stock::stock::ProductoStockBajo>, String> {
    let conn = state.conn.lock().unwrap();
    listar_stock_bajo_logic(&conn)
}

// Comandos de notificaciones
#[tauri::command]
pub fn obtener_notificacion(
    state: State<'_, DbState>,
    id_notificacion: i32,
) -> Result<crate::models::notificaciones::notificacion::Notificacion, String> {
    let conn = state.conn.lock().unwrap();
    obtener_notificacion_logic(&conn, id_notificacion)
}

#[tauri::command]
pub fn listar_notificaciones(
    state: State<'_, DbState>,
    solo_no_leidas: bool,
) -> Result<Vec<crate::models::notificaciones::notificacion::Notificacion>, String> {
    let conn = state.conn.lock().unwrap();
    listar_notificaciones_logic(&conn, solo_no_leidas)
}

#[tauri::command]
pub fn notificaciones_por_producto(
    state: State<'_, DbState>,
    id_producto: i32,
) -> Result<Vec<crate::models::notificaciones::notificacion::Notificacion>, String> {
    let conn = state.conn.lock().unwrap();
    notificaciones_por_producto_logic(&conn, id_producto)
}

#[tauri::command]
pub fn notificaciones_por_estado(
    state: State<'_, DbState>,
    estado: i32,
) -> Result<Vec<crate::models::notificaciones::notificacion::Notificacion>, String> {
    let conn = state.conn.lock().unwrap();
    notificaciones_por_estado_logic(&conn, estado)
}

#[tauri::command]
pub fn contar_notificaciones_no_leidas(
    state: State<'_, DbState>,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    contar_no_leidas_logic(&conn)
}

#[tauri::command]
pub fn marcar_notificacion(
    state: State<'_, DbState>,
    id_notificacion: i32,
    estado: i32,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    marcar_notificacion_logic(&conn, id_notificacion, estado)
}

#[tauri::command]
pub fn marcar_todas_notificaciones_leidas(
    state: State<'_, DbState>,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    marcar_todas_leidas_logic(&conn)
}

// Comandos de movimientos de entrada
#[tauri::command]
pub fn registrar_entrada(
    state: State<'_, DbState>,
    entrada: NuevoMovimientoEntrada,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    registrar_entrada_logic(&conn, &entrada)
}

#[tauri::command]
pub fn obtener_movimiento(
    state: State<'_, DbState>,
    id: i32,
) -> Result<crate::models::movimientos_entrada::movimiento_entrada::MovimientoEntrada, String> {
    let conn = state.conn.lock().unwrap();
    obtener_movimiento_logic(&conn, id)
}

#[tauri::command]
pub fn listar_movimientos_entrada(
    state: State<'_, DbState>,
) -> Result<Vec<crate::models::movimientos_entrada::movimiento_entrada::MovimientoEntradaDetalle>, String> {
    let conn = state.conn.lock().unwrap();
    listar_movimientos_entrada_logic(&conn)
}

#[tauri::command]
pub fn movimientos_por_producto(
    state: State<'_, DbState>,
    id_producto: i32,
) -> Result<Vec<crate::models::movimientos_entrada::movimiento_entrada::MovimientoEntradaDetalle>, String> {
    let conn = state.conn.lock().unwrap();
    movimientos_por_producto_logic(&conn, id_producto)
}

#[tauri::command]
pub fn movimientos_por_usuario(
    state: State<'_, DbState>,
    id_usuario: i32,
) -> Result<Vec<crate::models::movimientos_entrada::movimiento_entrada::MovimientoEntradaDetalle>, String> {
    let conn = state.conn.lock().unwrap();
    movimientos_por_usuario_logic(&conn, id_usuario)
}

#[tauri::command]
pub fn movimientos_por_rango_fechas(
    state: State<'_, DbState>,
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<crate::models::movimientos_entrada::movimiento_entrada::MovimientoEntradaDetalle>, String> {
    let conn = state.conn.lock().unwrap();
    movimientos_por_rango_fechas_logic(&conn, &fecha_inicio, &fecha_fin)
}

#[tauri::command]
pub fn total_entradas_por_producto(
    state: State<'_, DbState>,
    id_producto: i32,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    total_entradas_por_producto_logic(&conn, id_producto)
}

// Comandos de ventas
#[tauri::command]
pub fn registrar_venta(
    state: State<'_, DbState>,
    venta: NuevaVenta,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    registrar_venta_logic(&conn, &venta)
}

#[tauri::command]
pub fn obtener_venta(
    state: State<'_, DbState>,
    id_venta: i32,
) -> Result<crate::models::ventas::venta::Venta, String> {
    let conn = state.conn.lock().unwrap();
    obtener_venta_logic(&conn, id_venta)
}

#[tauri::command]
pub fn listar_detalle_venta(
    state: State<'_, DbState>,
    id_venta: i32,
) -> Result<Vec<crate::models::ventas::venta::DetalleVentaDetalle>, String> {
    let conn = state.conn.lock().unwrap();
    listar_detalle_venta_logic(&conn, id_venta)
}

#[tauri::command]
pub fn listar_ventas(
    state: State<'_, DbState>,
) -> Result<Vec<crate::models::ventas::venta::VentaResumen>, String> {
    let conn = state.conn.lock().unwrap();
    listar_ventas_logic(&conn)
}

#[tauri::command]
pub fn ventas_por_usuario(
    state: State<'_, DbState>,
    id_usuario: i32,
) -> Result<Vec<crate::models::ventas::venta::VentaResumen>, String> {
    let conn = state.conn.lock().unwrap();
    ventas_por_usuario_logic(&conn, id_usuario)
}

#[tauri::command]
pub fn ventas_por_caja(
    state: State<'_, DbState>,
    id_caja: i32,
) -> Result<Vec<crate::models::ventas::venta::VentaResumen>, String> {
    let conn = state.conn.lock().unwrap();
    ventas_por_caja_logic(&conn, id_caja)
}

#[tauri::command]
pub fn ventas_por_rango_fechas(
    state: State<'_, DbState>,
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<crate::models::ventas::venta::VentaResumen>, String> {
    let conn = state.conn.lock().unwrap();
    ventas_por_rango_fechas_logic(&conn, &fecha_inicio, &fecha_fin)
}

#[tauri::command]
pub fn resumen_ventas_diario(
    state: State<'_, DbState>,
) -> Result<Vec<crate::models::ventas::venta::ResumenVentasDiario>, String> {
    let conn = state.conn.lock().unwrap();
    resumen_ventas_diario_logic(&conn)
}

#[tauri::command]
pub fn total_ventas_por_producto(
    state: State<'_, DbState>,
    id_producto: i32,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    total_ventas_por_producto_logic(&conn, id_producto)
}


// Comandos de caja
#[tauri::command]
pub fn abrir_caja(
    state: State<'_, DbState>,
    nueva: NuevaCaja,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    abrir_caja_logic(&conn, &nueva)
}

#[tauri::command]
pub fn cerrar_caja(
    state: State<'_, DbState>,
    cierre: CierreCaja,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    cerrar_caja_logic(&conn, &cierre)
}

#[tauri::command]
pub fn obtener_caja(
    state: State<'_, DbState>,
    id: i32,
) -> Result<crate::models::caja::caja::Caja, String> {
    let conn = state.conn.lock().unwrap();
    obtener_caja_logic(&conn, id)
}

#[tauri::command]
pub fn obtener_caja_activa(
    state: State<'_, DbState>,
) -> Result<Option<crate::models::caja::caja::Caja>, String> {
    let conn = state.conn.lock().unwrap();
    obtener_caja_activa_logic(&conn)
}

#[tauri::command]
pub fn listar_cajas(
    state: State<'_, DbState>,
    solo_abiertas: bool,
) -> Result<Vec<crate::models::caja::caja::Caja>, String> {
    let conn = state.conn.lock().unwrap();
    listar_cajas_logic(&conn, solo_abiertas)
}


// Comandos de historial
#[tauri::command]
pub fn registrar_accion(
    state: State<'_, DbState>,
    accion: NuevaAccion,
) -> Result<i32, String> {
    let conn = state.conn.lock().unwrap();
    registrar_accion_logic(&conn, &accion)
}

#[tauri::command]
pub fn listar_historial(
    state: State<'_, DbState>,
    filtro: FiltroHistorial,
) -> Result<Vec<crate::models::historial::historial::HistorialAccion>, String> {
    let conn = state.conn.lock().unwrap();
    listar_historial_logic(&conn, &filtro)
}

#[tauri::command]
pub fn obtener_accion(
    state: State<'_, DbState>,
    id: i32,
) -> Result<crate::models::historial::historial::HistorialAccion, String> {
    let conn = state.conn.lock().unwrap();
    obtener_accion_logic(&conn, id)
}

#[tauri::command]
pub fn historial_por_usuario(
    state: State<'_, DbState>,
    id_usuario: i32,
) -> Result<Vec<crate::models::historial::historial::HistorialAccion>, String> {
    let conn = state.conn.lock().unwrap();
    historial_por_usuario_logic(&conn, id_usuario)
}

#[tauri::command]
pub fn historial_por_turno(
    state: State<'_, DbState>,
    id_turno: i32,
) -> Result<Vec<crate::models::historial::historial::HistorialAccion>, String> {
    let conn = state.conn.lock().unwrap();
    historial_por_turno_logic(&conn, id_turno)
}