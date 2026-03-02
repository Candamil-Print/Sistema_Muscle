// backend/src/routes/producto.routes.js
const router = require('express').Router();
const productoController = require('../controllers/producto.controller');
const validationMiddleware = require('../middlewares/validation.middleware');
const productoValidation = require('../validations/producto.validation');
const authMiddleware = require('../middlewares/auth.middleware');

// router.use(authMiddleware.verifyToken);

// Rutas públicas (con autenticación comentada)
router.get(
  '/',
  validationMiddleware.validate(productoValidation.getAll, 'query'),
  productoController.getAll
);

router.get(
  '/estadisticas',
  productoController.getEstadisticas
);

router.get(
  '/tipo/:tipo',
  validationMiddleware.validate(productoValidation.getByTipo, 'params'),
  productoController.getByTipo
);

router.get(
  '/:id',
  validationMiddleware.validate(productoValidation.getById, 'params'),
  productoController.getById
);

// Rutas que modifican datos (requieren autenticación - descomentar después)
router.post(
  '/',
  validationMiddleware.validate(productoValidation.create),
  productoController.create
);

router.put(
  '/:id',
  validationMiddleware.validate(productoValidation.update),
  productoController.update
);

router.delete(
  '/:id',
  validationMiddleware.validate(productoValidation.getById, 'params'),
  productoController.delete
);

router.put(
  '/precios/actualizar',
  validationMiddleware.validate(productoValidation.actualizarPrecios),
  productoController.actualizarPrecios
);

module.exports = router;