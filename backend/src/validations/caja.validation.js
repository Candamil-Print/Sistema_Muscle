// backend/src/validations/caja.validation.js
const { body, param, query } = require('express-validator');

const cajaValidation = {
  abrirCaja: [
    body('id_usuario_apertura')
      .notEmpty().withMessage('El usuario es requerido')
      .isInt().withMessage('ID de usuario inválido'),
    
    body('id_turno')
      .notEmpty().withMessage('El turno es requerido')
      .isInt().withMessage('ID de turno inválido'),
    
    body('monto_apertura')
      .notEmpty().withMessage('El monto de apertura es requerido')
      .isDecimal({ min: 0 }).withMessage('Monto inválido (debe ser ≥ 0)')
  ],

  cerrarCaja: [
    param('id')
      .isInt().withMessage('ID de caja inválido'),
    
    body('id_usuario_cierre')
      .notEmpty().withMessage('El usuario que cierra es requerido')
      .isInt().withMessage('ID de usuario inválido'),
    
    body('monto_cierre')
      .notEmpty().withMessage('El monto de cierre es requerido')
      .isDecimal({ min: 0 }).withMessage('Monto inválido'),
    
    body('total_efectivo')
      .optional()
      .isDecimal({ min: 0 }).withMessage('Total efectivo inválido'),
    
    body('total_transferencia')
      .optional()
      .isDecimal({ min: 0 }).withMessage('Total transferencia inválido')
  ],

  getById: [
    param('id')
      .isInt().withMessage('ID de caja inválido')
  ],

  getByUsuario: [
    param('id_usuario')
      .isInt().withMessage('ID de usuario inválido')
  ],

  getByTurno: [
    param('id_turno')
      .isInt().withMessage('ID de turno inválido')
  ],

  getAll: [
    query('page')
      .optional()
      .isInt({ min: 1 }).withMessage('Página inválida'),
    query('limit')
      .optional()
      .isInt({ min: 1, max: 100 }).withMessage('Límite inválido'),
    query('fecha_desde')
      .optional()
      .isISO8601().withMessage('Fecha desde inválida'),
    query('fecha_hasta')
      .optional()
      .isISO8601().withMessage('Fecha hasta inválida'),
    query('estado')
      .optional()
      .isIn(['ABIERTA', 'CERRADA']).withMessage('Estado inválido')
  ]
};

module.exports = cajaValidation;