// backend/src/routes/caja.routes.js
const router = require('express').Router();
const cajaController = require('../controllers/caja.controller');
const validationMiddleware = require('../middlewares/validation.middleware');
const cajaValidation = require('../validations/caja.validation');

// Rutas para caja
router.get(
  '/',
  validationMiddleware.validate(cajaValidation.getAll, 'query'),
  cajaController.getAll
);

router.get(
  '/estadisticas',
  cajaController.getEstadisticas
);

router.get(
  '/usuario/:id_usuario/activa',
  validationMiddleware.validate(cajaValidation.getByUsuario, 'params'),
  cajaController.getCajaActivaPorUsuario
);

router.get(
  '/turno/:id_turno/activa',
  validationMiddleware.validate(cajaValidation.getByTurno, 'params'),
  cajaController.getCajaActivaPorTurno
);

router.get(
  '/:id/resumen',
  validationMiddleware.validate(cajaValidation.getById, 'params'),
  cajaController.getResumenCaja
);

router.get(
  '/:id',
  validationMiddleware.validate(cajaValidation.getById, 'params'),
  cajaController.getById
);

router.post(
  '/abrir',
  validationMiddleware.validate(cajaValidation.abrirCaja),
  cajaController.abrirCaja
);

router.put(
  '/:id/cerrar',
  validationMiddleware.validate(cajaValidation.cerrarCaja),
  cajaController.cerrarCaja
);

module.exports = router;