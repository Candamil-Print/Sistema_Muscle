// backend/src/controllers/turno.controller.js
const turnoService = require('../services/turno.service');

class TurnoController {
  // Obtener todos los turnos
  async getAll(req, res, next) {
    try {
      const result = await turnoService.findAll(req.query);
      res.json({
        success: true,
        message: 'Turnos obtenidos correctamente',
        ...result
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener turno por ID
  async getById(req, res, next) {
    try {
      const turno = await turnoService.findById(req.params.id);
      res.json({
        success: true,
        message: 'Turno obtenido correctamente',
        data: turno
      });
    } catch (error) {
      next(error);
    }
  }

  // Abrir turno
  async abrirTurno(req, res, next) {
    try {
      const turno = await turnoService.abrirTurno(req.body);
      res.status(201).json({
        success: true,
        message: 'Turno abierto correctamente',
        data: turno
      });
    } catch (error) {
      next(error);
    }
  }

  // Cerrar turno
  async cerrarTurno(req, res, next) {
    try {
      const turno = await turnoService.cerrarTurno(req.params.id);
      res.json({
        success: true,
        message: 'Turno cerrado correctamente',
        data: turno
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener turno activo de un usuario
  async getTurnoActivo(req, res, next) {
    try {
      const turno = await turnoService.getTurnoActivo(req.params.id_usuario);
      res.json({
        success: true,
        message: turno ? 'Turno activo encontrado' : 'No hay turno activo',
        data: turno || null
      });
    } catch (error) {
      next(error);
    }
  }

  // Turnos por fecha
  async getTurnosPorFecha(req, res, next) {
    try {
      const turnos = await turnoService.getTurnosPorFecha(req.params.fecha);
      res.json({
        success: true,
        message: 'Turnos obtenidos correctamente',
        data: turnos
      });
    } catch (error) {
      next(error);
    }
  }

  // Turnos por usuario
  async getTurnosPorUsuario(req, res, next) {
    try {
      const turnos = await turnoService.getTurnosPorUsuario(
        req.params.id_usuario, 
        req.query
      );
      res.json({
        success: true,
        message: 'Turnos del usuario obtenidos',
        data: turnos
      });
    } catch (error) {
      next(error);
    }
  }

  // Estadísticas
  async getEstadisticas(req, res, next) {
    try {
      const estadisticas = await turnoService.getEstadisticas(req.query);
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

module.exports = new TurnoController();