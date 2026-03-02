// backend/src/routes/turno.routes.js
const router = require('express').Router();
const turnoController = require('../controllers/turno.controller');
const validationMiddleware = require('../middlewares/validation.middleware');
const turnoValidation = require('../validations/turno.validation');

// Rutas para turnos
router.get(
  '/',
  validationMiddleware.validate(turnoValidation.getAll, 'query'),
  turnoController.getAll
);

router.get(
  '/estadisticas',
  turnoController.getEstadisticas
);

router.get(
  '/usuario/:id_usuario/activo',
  validationMiddleware.validate(turnoValidation.getTurnoActivo, 'params'),
  turnoController.getTurnoActivo
);

router.get(
  '/usuario/:id_usuario',
  validationMiddleware.validate(turnoValidation.getTurnosPorUsuario, 'params'),
  turnoController.getTurnosPorUsuario
);

router.get(
  '/fecha/:fecha',
  validationMiddleware.validate(turnoValidation.getTurnosPorFecha, 'params'),
  turnoController.getTurnosPorFecha
);

router.get(
  '/:id',
  validationMiddleware.validate(turnoValidation.getById, 'params'),
  turnoController.getById
);

router.post(
  '/abrir',
  validationMiddleware.validate(turnoValidation.abrirTurno),
  turnoController.abrirTurno
);

router.put(
  '/:id/cerrar',
  validationMiddleware.validate(turnoValidation.getById, 'params'),
  turnoController.cerrarTurno
);

module.exports = router;