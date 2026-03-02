// backend/src/services/producto.service.js
const { Producto } = require('../models');
const { Op } = require('sequelize');

class ProductoService {
  // Obtener todos los productos con paginación y filtros
  async findAll(query = {}) {
    const { 
      page = 1, 
      limit = 10, 
      search = '', 
      tipo,
      precio_min,
      precio_max,
      sortBy = 'id_producto',
      order = 'DESC'
    } = query;

    const offset = (page - 1) * limit;
    
    // Construir filtros
    const where = {};
    
    if (search) {
      where.nombre = { [Op.like]: `%${search}%` };
    }
    
    if (tipo) {
      where.tipo_producto = tipo;
    }
    
    if (precio_min || precio_max) {
      where.precio_sugerido = {};
      if (precio_min) where.precio_sugerido[Op.gte] = parseFloat(precio_min);
      if (precio_max) where.precio_sugerido[Op.lte] = parseFloat(precio_max);
    }

    const { count, rows } = await Producto.findAndCountAll({
      where,
      limit: parseInt(limit),
      offset: parseInt(offset),
      order: [[sortBy, order]]
    });

    return {
      total: count,
      page: parseInt(page),
      limit: parseInt(limit),
      totalPages: Math.ceil(count / limit),
      data: rows
    };
  }

  // Obtener producto por ID
  async findById(id) {
    const producto = await Producto.findByPk(id);
    if (!producto) {
      throw new Error('Producto no encontrado');
    }
    return producto;
  }

  // Crear producto
  async create(data) {
    // Validar que el precio sugerido sea mayor al precio costo
    if (parseFloat(data.precio_sugerido) <= parseFloat(data.precio_costo)) {
      throw new Error('El precio sugerido debe ser mayor al precio de costo');
    }

    const producto = await Producto.create(data);
    return producto;
  }

  // Actualizar producto
  async update(id, data) {
    const producto = await Producto.findByPk(id);
    
    if (!producto) {
      throw new Error('Producto no encontrado');
    }

    // Validar precios si se están actualizando
    const precioCosto = data.precio_costo || producto.precio_costo;
    const precioSugerido = data.precio_sugerido || producto.precio_sugerido;
    
    if (parseFloat(precioSugerido) <= parseFloat(precioCosto)) {
      throw new Error('El precio sugerido debe ser mayor al precio de costo');
    }

    await producto.update(data);
    return producto;
  }

  // Eliminar producto
  async delete(id) {
    const producto = await Producto.findByPk(id);
    
    if (!producto) {
      throw new Error('Producto no encontrado');
    }

    await producto.destroy();
    return { message: 'Producto eliminado correctamente' };
  }

  // Obtener productos por tipo
  async findByTipo(tipo) {
    const productos = await Producto.findAll({
      where: { tipo_producto: tipo }
    });
    return productos;
  }

  // Actualizar precios en lote (ej: aumentar 10%)
  async actualizarPrecios(porcentaje, tipo = null) {
    const where = tipo ? { tipo_producto: tipo } : {};
    
    const productos = await Producto.findAll({ where });
    
    for (const producto of productos) {
      const nuevoPrecio = parseFloat(producto.precio_sugerido) * (1 + porcentaje / 100);
      await producto.update({ 
        precio_sugerido: nuevoPrecio.toFixed(2) 
      });
    }
    
    return { 
      message: `Precios actualizados ${porcentaje > 0 ? '+' : ''}${porcentaje}%`,
      productos_actualizados: productos.length 
    };
  }

  // Obtener estadísticas de productos
  async getEstadisticas() {
    const total = await Producto.count();
    
    const porTipo = await Producto.findAll({
      attributes: [
        'tipo_producto',
        [sequelize.fn('COUNT', sequelize.col('tipo_producto')), 'cantidad'],
        [sequelize.fn('AVG', sequelize.col('precio_sugerido')), 'precio_promedio'],
        [sequelize.fn('MIN', sequelize.col('precio_sugerido')), 'precio_minimo'],
        [sequelize.fn('MAX', sequelize.col('precio_sugerido')), 'precio_maximo']
      ],
      group: ['tipo_producto']
    });

    return {
      total_productos: total,
      tipos: porTipo
    };
  }
}

module.exports = new ProductoService();