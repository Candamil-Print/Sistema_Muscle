// backend/src/routes/venta.routes.js
const router = require('express').Router();
const ventaController = require('../controllers/venta.controller');
const validationMiddleware = require('../middlewares/validation.middleware');
const ventaValidation = require('../validations/venta.validation');

// Rutas para ventas
router.get(
  '/',
  validationMiddleware.validate(ventaValidation.getAll, 'query'),
  ventaController.getAll
);

router.get(
  '/dia',
  ventaController.getVentasDelDia
);

router.get(
  '/estadisticas',
  ventaController.getEstadisticas
);

// NUEVA RUTA: Ventas por caja
router.get(
  '/caja/:id_caja',
  ventaController.getVentasPorCaja
);

router.get(
  '/:id',
  validationMiddleware.validate(ventaValidation.getById, 'params'),
  ventaController.getById
);

router.post(
  '/',
  validationMiddleware.validate(ventaValidation.create),
  ventaController.create
);

router.delete(
  '/:id/anular',
  validationMiddleware.validate(ventaValidation.getById, 'params'),
  ventaController.anularVenta
);

module.exports = router;