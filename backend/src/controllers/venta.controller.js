// backend/src/controllers/venta.controller.js
const ventaService = require('../services/venta.service');

class VentaController {
  // ... (todos los métodos existentes) ...

  // Nuevo método: Ventas por caja
  async getVentasPorCaja(req, res, next) {
    try {
      const ventas = await ventaService.getVentasPorCaja(req.params.id_caja, req.query);
      res.json({
        success: true,
        message: 'Ventas de la caja obtenidas',
        data: ventas
      });
    } catch (error) {
      next(error);
    }
  }
}

module.exports = new VentaController();