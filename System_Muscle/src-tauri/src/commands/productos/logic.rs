use rusqlite::Connection;
use crate::models::productos::producto::{
    Producto, ProductoConStock, NuevoProducto, ModificarProducto,
};

/// Crea un producto y su registro de stock inicial.
/// Retorna el id_producto generado.
pub fn crear_producto_logic(conn: &Connection, nuevo: &NuevoProducto) -> Result<i32, String> {
    // Validar que no exista un producto activo con el mismo nombre
    let mut stmt = conn
        .prepare("SELECT 1 FROM productos WHERE nombre = ?1 AND activo = 1")
        .map_err(|e| e.to_string())?;

    let existe = stmt
        .exists([&nuevo.nombre])
        .map_err(|e| e.to_string())?;

    if existe {
        return Err("Ya existe un producto activo con ese nombre".to_string());
    }

    // Insertar en productos
    conn.execute(
        r#"INSERT INTO productos (nombre, tipo_producto, precio_costo, precio_sugerido, imagen_url, activo)
           VALUES (?1, ?2, ?3, ?4, ?5, 1)"#,
        rusqlite::params![
            &nuevo.nombre,
            &nuevo.tipo_producto,
            nuevo.precio_costo,
            nuevo.precio_sugerido,
            nuevo.imagen_url.as_deref().unwrap_or(""),
        ],
    )
    .map_err(|e| e.to_string())?;

    let id_producto = conn.last_insert_rowid() as i32;

    // Crear registro de stock vinculado al producto
    conn.execute(
        r#"INSERT INTO stock (id_producto, stock_actual, stock_maximo)
           VALUES (?1, 0, ?2)"#,
        rusqlite::params![id_producto, nuevo.stock_maximo],
    )
    .map_err(|e| e.to_string())?;

    Ok(id_producto)
}

/// Modifica los campos opcionales de un producto (y stock_maximo si aplica).
pub fn modificar_producto_logic(
    conn: &Connection,
    id: i32,
    datos: &ModificarProducto,
) -> Result<(), String> {
    // Construir SET dinámico para la tabla productos
    let mut updates: Vec<String> = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(ref nombre) = datos.nombre {
        updates.push("nombre = ?".to_string());
        params.push(nombre.clone());
    }
    if let Some(ref tipo) = datos.tipo_producto {
        updates.push("tipo_producto = ?".to_string());
        params.push(tipo.clone());
    }
    if let Some(precio_costo) = datos.precio_costo {
        updates.push("precio_costo = ?".to_string());
        params.push(precio_costo.to_string());
    }
    if let Some(precio_sugerido) = datos.precio_sugerido {
        updates.push("precio_sugerido = ?".to_string());
        params.push(precio_sugerido.to_string());
    }
    if let Some(ref imagen) = datos.imagen_url {
        updates.push("imagen_url = ?".to_string());
        params.push(imagen.clone());
    }

    if !updates.is_empty() {
        params.push(id.to_string());
        let query = format!(
            "UPDATE productos SET {} WHERE id_producto = ?",
            updates.join(", ")
        );
        let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
        let params_slice: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
        stmt.execute(rusqlite::params_from_iter(params_slice))
            .map_err(|e| e.to_string())?;
    }

    // Actualizar stock_maximo si se proporcionó
    if let Some(stock_maximo) = datos.stock_maximo {
        conn.execute(
            "UPDATE stock SET stock_maximo = ?1, fecha_actualizacion = CURRENT_TIMESTAMP WHERE id_producto = ?2",
            rusqlite::params![stock_maximo, id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Obtiene un producto por su ID junto con sus datos de stock.
pub fn obtener_producto_logic(conn: &Connection, id: i32) -> Result<ProductoConStock, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT p.id_producto, p.nombre, p.tipo_producto, p.precio_costo, p.precio_sugerido,
                      p.imagen_url, p.fecha_creacion, p.activo,
                      s.stock_actual, s.stock_maximo, s.stock_minimo, s.fecha_actualizacion
               FROM productos p
               LEFT JOIN stock s ON p.id_producto = s.id_producto
               WHERE p.id_producto = ?1"#,
        )
        .map_err(|e| e.to_string())?;

    let producto = stmt
        .query_row([id], |row| {
            Ok(ProductoConStock {
                id_producto: row.get(0)?,
                nombre: row.get(1)?,
                tipo_producto: row.get(2)?,
                precio_costo: row.get(3)?,
                precio_sugerido: row.get(4)?,
                imagen_url: row.get(5)?,
                fecha_creacion: row.get(6)?,
                activo: row.get(7)?,
                stock_actual: row.get(8)?,
                stock_maximo: row.get(9)?,
                stock_minimo: row.get(10)?,
                fecha_actualizacion_stock: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(producto)
}

/// Lista todos los productos (con stock). Si `solo_activos` es true, filtra activo = 1.
pub fn listar_productos_logic(
    conn: &Connection,
    solo_activos: bool,
) -> Result<Vec<ProductoConStock>, String> {
    let query = if solo_activos {
        r#"SELECT p.id_producto, p.nombre, p.tipo_producto, p.precio_costo, p.precio_sugerido,
                  p.imagen_url, p.fecha_creacion, p.activo,
                  s.stock_actual, s.stock_maximo, s.stock_minimo, s.fecha_actualizacion
           FROM productos p
           LEFT JOIN stock s ON p.id_producto = s.id_producto
           WHERE p.activo = 1
           ORDER BY p.nombre"#
    } else {
        r#"SELECT p.id_producto, p.nombre, p.tipo_producto, p.precio_costo, p.precio_sugerido,
                  p.imagen_url, p.fecha_creacion, p.activo,
                  s.stock_actual, s.stock_maximo, s.stock_minimo, s.fecha_actualizacion
           FROM productos p
           LEFT JOIN stock s ON p.id_producto = s.id_producto
           ORDER BY p.nombre"#
    };

    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ProductoConStock {
                id_producto: row.get(0)?,
                nombre: row.get(1)?,
                tipo_producto: row.get(2)?,
                precio_costo: row.get(3)?,
                precio_sugerido: row.get(4)?,
                imagen_url: row.get(5)?,
                fecha_creacion: row.get(6)?,
                activo: row.get(7)?,
                stock_actual: row.get(8)?,
                stock_maximo: row.get(9)?,
                stock_minimo: row.get(10)?,
                fecha_actualizacion_stock: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut productos = Vec::new();
    for p in rows {
        productos.push(p.map_err(|e| e.to_string())?);
    }

    Ok(productos)
}

/// Busca productos por nombre o tipo_producto usando LIKE (case-insensitive).
pub fn buscar_productos_logic(
    conn: &Connection,
    termino: &str,
    solo_activos: bool,
) -> Result<Vec<ProductoConStock>, String> {
    let patron = format!("%{}%", termino);

    let query = if solo_activos {
        r#"SELECT p.id_producto, p.nombre, p.tipo_producto, p.precio_costo, p.precio_sugerido,
                  p.imagen_url, p.fecha_creacion, p.activo,
                  s.stock_actual, s.stock_maximo, s.stock_minimo, s.fecha_actualizacion
           FROM productos p
           LEFT JOIN stock s ON p.id_producto = s.id_producto
           WHERE p.activo = 1
             AND (UPPER(p.nombre) LIKE UPPER(?1) OR UPPER(p.tipo_producto) LIKE UPPER(?1))
           ORDER BY p.nombre"#
    } else {
        r#"SELECT p.id_producto, p.nombre, p.tipo_producto, p.precio_costo, p.precio_sugerido,
                  p.imagen_url, p.fecha_creacion, p.activo,
                  s.stock_actual, s.stock_maximo, s.stock_minimo, s.fecha_actualizacion
           FROM productos p
           LEFT JOIN stock s ON p.id_producto = s.id_producto
           WHERE UPPER(p.nombre) LIKE UPPER(?1) OR UPPER(p.tipo_producto) LIKE UPPER(?1)
           ORDER BY p.nombre"#
    };

    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([&patron], |row| {
            Ok(ProductoConStock {
                id_producto: row.get(0)?,
                nombre: row.get(1)?,
                tipo_producto: row.get(2)?,
                precio_costo: row.get(3)?,
                precio_sugerido: row.get(4)?,
                imagen_url: row.get(5)?,
                fecha_creacion: row.get(6)?,
                activo: row.get(7)?,
                stock_actual: row.get(8)?,
                stock_maximo: row.get(9)?,
                stock_minimo: row.get(10)?,
                fecha_actualizacion_stock: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut productos = Vec::new();
    for p in rows {
        productos.push(p.map_err(|e| e.to_string())?);
    }

    Ok(productos)
}

/// Activa un producto (activo = 1).
pub fn activar_producto_logic(conn: &Connection, id: i32) -> Result<(), String> {
    conn.execute(
        "UPDATE productos SET activo = 1 WHERE id_producto = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Desactiva un producto de forma lógica (activo = 0). No elimina el registro.
pub fn desactivar_producto_logic(conn: &Connection, id: i32) -> Result<(), String> {
    conn.execute(
        "UPDATE productos SET activo = 0 WHERE id_producto = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Obtiene solo los campos básicos de un producto (sin stock).
pub fn obtener_producto_simple_logic(conn: &Connection, id: i32) -> Result<Producto, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT id_producto, nombre, tipo_producto, precio_costo, precio_sugerido,
                      imagen_url, fecha_creacion, activo
               FROM productos WHERE id_producto = ?1"#,
        )
        .map_err(|e| e.to_string())?;

    let producto = stmt
        .query_row([id], |row| {
            Ok(Producto {
                id_producto: row.get(0)?,
                nombre: row.get(1)?,
                tipo_producto: row.get(2)?,
                precio_costo: row.get(3)?,
                precio_sugerido: row.get(4)?,
                imagen_url: row.get(5)?,
                fecha_creacion: row.get(6)?,
                activo: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(producto)
}
