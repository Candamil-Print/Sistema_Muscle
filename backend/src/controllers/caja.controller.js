// backend/src/controllers/caja.controller.js
const cajaService = require('../services/caja.service');

class CajaController {
  // Obtener todas las cajas
  async getAll(req, res, next) {
    try {
      const result = await cajaService.findAll(req.query);
      res.json({
        success: true,
        message: 'Cajas obtenidas correctamente',
        ...result
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener caja por ID
  async getById(req, res, next) {
    try {
      const caja = await cajaService.findById(req.params.id);
      res.json({
        success: true,
        message: 'Caja obtenida correctamente',
        data: caja
      });
    } catch (error) {
      next(error);
    }
  }

  // Abrir caja
  async abrirCaja(req, res, next) {
    try {
      const caja = await cajaService.abrirCaja(req.body);
      res.status(201).json({
        success: true,
        message: 'Caja abierta correctamente',
        data: caja
      });
    } catch (error) {
      next(error);
    }
  }

  // Cerrar caja
  async cerrarCaja(req, res, next) {
    try {
      const caja = await cajaService.cerrarCaja(req.params.id, req.body);
      res.json({
        success: true,
        message: 'Caja cerrada correctamente',
        data: caja
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener caja activa por usuario
  async getCajaActivaPorUsuario(req, res, next) {
    try {
      const caja = await cajaService.getCajaActivaPorUsuario(req.params.id_usuario);
      res.json({
        success: true,
        message: caja ? 'Caja activa encontrada' : 'No hay caja activa',
        data: caja || null
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener caja activa por turno
  async getCajaActivaPorTurno(req, res, next) {
    try {
      const caja = await cajaService.getCajaActivaPorTurno(req.params.id_turno);
      res.json({
        success: true,
        message: caja ? 'Caja activa encontrada' : 'No hay caja activa',
        data: caja || null
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener resumen de caja
  async getResumenCaja(req, res, next) {
    try {
      const resumen = await cajaService.getResumenCaja(req.params.id);
      res.json({
        success: true,
        message: 'Resumen obtenido correctamente',
        data: resumen
      });
    } catch (error) {
      next(error);
    }
  }

  // Estadísticas
  async getEstadisticas(req, res, next) {
    try {
      const estadisticas = await cajaService.getEstadisticas(req.query);
      res.json({
        success: true,
        message: 'Estadísticas obtenidas',
        data: estadisticas
      });
    } catch (error) {
      next(error);
    }
  }
}

module.exports = new CajaController();