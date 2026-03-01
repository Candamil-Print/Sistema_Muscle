// backend/src/routes/usuario.routes.js
const router = require('express').Router();
const usuarioController = require('../controllers/usuario.controller');
const validationMiddleware = require('../middlewares/validation.middleware');
const usuarioValidation = require('../validations/usuario.validation');
const authMiddleware = require('../middlewares/auth.middleware');

// Todas las rutas requieren autenticación
// router.use(authMiddleware.verifyToken);

// Rutas para obtener usuarios
router.get(
  '/',
  validationMiddleware.validate(usuarioValidation.getAll, 'query'),
  usuarioController.getAll
);

router.get(
  '/rol/:id_rol',
  validationMiddleware.validate(usuarioValidation.getByRol, 'params'),
  usuarioController.getByRol
);

router.get(
  '/:id',
  validationMiddleware.validate(usuarioValidation.getById, 'params'),
  usuarioController.getById
);

// Rutas para crear usuario
router.post(
  '/',
  validationMiddleware.validate(usuarioValidation.create),
  usuarioController.create
);

// Rutas para actualizar usuario
router.put(
  '/:id',
  validationMiddleware.validate(usuarioValidation.update),
  usuarioController.update
);

// Rutas para cambiar contraseña
router.put(
  '/:id/cambiar-password',
  validationMiddleware.validate(usuarioValidation.cambiarPassword),
  usuarioController.cambiarPassword
);

// Rutas para eliminar (soft delete)
router.delete(
  '/:id',
  validationMiddleware.validate(usuarioValidation.getById, 'params'),
  usuarioController.delete
);

// Ruta para eliminar permanentemente (solo admin)
router.delete(
  '/:id/hard',
  authMiddleware.verifyAdmin,
  validationMiddleware.validate(usuarioValidation.getById, 'params'),
  usuarioController.hardDelete
);

module.exports = router;