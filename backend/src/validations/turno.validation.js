// backend/src/validations/turno.validation.js
const { body, param, query } = require('express-validator');

const turnoValidation = {
  abrirTurno: [
    body('id_usuario')
      .notEmpty().withMessage('El usuario es requerido')
      .isInt().withMessage('ID de usuario inválido'),
    
    body('id_tipo_turno')
      .notEmpty().withMessage('El tipo de turno es requerido')
      .isInt().withMessage('ID de tipo de turno inválido')
  ],

  getById: [
    param('id')
      .isInt().withMessage('ID de turno inválido')
  ],

  getTurnoActivo: [
    param('id_usuario')
      .isInt().withMessage('ID de usuario inválido')
  ],

  getTurnosPorFecha: [
    param('fecha')
      .isISO8601().withMessage('Fecha inválida (use YYYY-MM-DD)')
  ],

  getTurnosPorUsuario: [
    param('id_usuario')
      .isInt().withMessage('ID de usuario inválido')
  ],

  getAll: [
    query('page')
      .optional()
      .isInt({ min: 1 }).withMessage('Página inválida'),
    query('limit')
      .optional()
      .isInt({ min: 1, max: 100 }).withMessage('Límite inválido'),
    query('fecha')
      .optional()
      .isISO8601().withMessage('Fecha inválida (use YYYY-MM-DD)'),
    query('estado')
      .optional()
      .isIn(['ABIERTO', 'CERRADO']).withMessage('Estado inválido')
  ]
};

module.exports = turnoValidation;