// backend/src/controllers/producto.controller.js
const productoService = require('../services/producto.service');

class ProductoController {
  // Obtener todos los productos
  async getAll(req, res, next) {
    try {
      const result = await productoService.findAll(req.query);
      res.json({
        success: true,
        message: 'Productos obtenidos correctamente',
        ...result
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener producto por ID
  async getById(req, res, next) {
    try {
      const producto = await productoService.findById(req.params.id);
      res.json({
        success: true,
        message: 'Producto obtenido correctamente',
        data: producto
      });
    } catch (error) {
      next(error);
    }
  }

  // Crear producto
  async create(req, res, next) {
    try {
      const producto = await productoService.create(req.body);
      res.status(201).json({
        success: true,
        message: 'Producto creado correctamente',
        data: producto
      });
    } catch (error) {
      next(error);
    }
  }

  // Actualizar producto
  async update(req, res, next) {
    try {
      const producto = await productoService.update(req.params.id, req.body);
      res.json({
        success: true,
        message: 'Producto actualizado correctamente',
        data: producto
      });
    } catch (error) {
      next(error);
    }
  }

  // Eliminar producto
  async delete(req, res, next) {
    try {
      const result = await productoService.delete(req.params.id);
      res.json({
        success: true,
        message: result.message
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener productos por tipo
  async getByTipo(req, res, next) {
    try {
      const productos = await productoService.findByTipo(req.params.tipo);
      res.json({
        success: true,
        message: 'Productos obtenidos correctamente',
        data: productos
      });
    } catch (error) {
      next(error);
    }
  }

  // Actualizar precios en lote
  async actualizarPrecios(req, res, next) {
    try {
      const { porcentaje, tipo } = req.body;
      const result = await productoService.actualizarPrecios(porcentaje, tipo);
      res.json({
        success: true,
        message: result.message,
        data: result
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener estadísticas
  async getEstadisticas(req, res, next) {
    try {
      const estadisticas = await productoService.getEstadisticas();
      res.json({
        success: true,
        message: 'Estadísticas obtenidas correctamente',
        data: estadisticas
      });
    } catch (error) {
      next(error);
    }
  }
}

module.exports = new ProductoController();