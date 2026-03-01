// backend/src/validations/usuario.validation.js
const { body, param, query } = require('express-validator');

const usuarioValidation = {
  create: [
    body('nombre_completo')
      .notEmpty().withMessage('Full name is required')
      .isLength({ max: 150 }).withMessage('Maximum 150 characters'),
    
    body('tipo_documento')
      .notEmpty().withMessage('Document type is required')
      .isIn(['CC', 'TI', 'CE', 'PASAPORTE']).withMessage('Invalid document type'),
    
    body('numero_documento')
      .notEmpty().withMessage('Document number is required')
      .isLength({ max: 50 }).withMessage('Maximum 50 characters')
      .matches(/^[0-9]+$/).withMessage('Only numbers allowed'),
    
    body('direccion')
      .optional()
      .isLength({ max: 200 }).withMessage('Maximum 200 characters'),
    
    body('tipo_sangre')
      .optional()
      .isIn(['O+', 'O-', 'A+', 'A-', 'B+', 'B-', 'AB+', 'AB-'])
      .withMessage('Invalid blood type'),
    
    body('eps')
      .optional()
      .isIn(['SURA', 'SANITAS', 'COMPENSAR', 'NUEVA_EPS', 'FAMISANAR', 'COOMEVA', 'SALUD_TOTAL'])
      .withMessage('Invalid EPS'),
    
    body('genero')
      .optional()
      .isIn(['MASCULINO', 'FEMENINO', 'OTRO', 'PREFIERE_NO_DECIR'])
      .withMessage('Invalid gender'),
    
    body('correo')
      .optional()
      .isEmail().withMessage('Invalid email')
      .isLength({ max: 150 }).withMessage('Maximum 150 characters'),
    
    body('telefono')
      .optional()
      .isLength({ max: 50 }).withMessage('Maximum 50 characters'),
    
    body('password_hash')
      .notEmpty().withMessage('Password is required')
      .isLength({ min: 6 }).withMessage('Password must be at least 6 characters'),
    
    body('id_rol')
      .notEmpty().withMessage('Role is required')
      .isInt().withMessage('Role must be a number')
  ],

  update: [
    param('id')
      .isInt().withMessage('Invalid ID'),
    
    body('nombre_completo')
      .optional()
      .isLength({ max: 150 }).withMessage('Maximum 150 characters'),
    
    body('tipo_documento')
      .optional()
      .isIn(['CC', 'TI', 'CE', 'PASAPORTE']).withMessage('Invalid document type'),
    
    body('numero_documento')
      .optional()
      .isLength({ max: 50 }).withMessage('Maximum 50 characters')
      .matches(/^[0-9]+$/).withMessage('Only numbers allowed'),
    
    body('direccion')
      .optional()
      .isLength({ max: 200 }).withMessage('Maximum 200 characters'),
    
    body('tipo_sangre')
      .optional()
      .isIn(['O+', 'O-', 'A+', 'A-', 'B+', 'B-', 'AB+', 'AB-'])
      .withMessage('Invalid blood type'),
    
    body('eps')
      .optional()
      .isIn(['SURA', 'SANITAS', 'COMPENSAR', 'NUEVA_EPS', 'FAMISANAR', 'COOMEVA', 'SALUD_TOTAL'])
      .withMessage('Invalid EPS'),
    
    body('genero')
      .optional()
      .isIn(['MASCULINO', 'FEMENINO', 'OTRO', 'PREFIERE_NO_DECIR'])
      .withMessage('Invalid gender'),
    
    body('correo')
      .optional()
      .isEmail().withMessage('Invalid email')
      .isLength({ max: 150 }).withMessage('Maximum 150 characters'),
    
    body('telefono')
      .optional()
      .isLength({ max: 50 }).withMessage('Maximum 50 characters'),
    
    body('password_hash')
      .optional()
      .isLength({ min: 6 }).withMessage('Password must be at least 6 characters'),
    
    body('id_rol')
      .optional()
      .isInt().withMessage('Role must be a number')
  ],

  cambiarPassword: [
    param('id')
      .isInt().withMessage('Invalid ID'),
    
    body('password_actual')
      .notEmpty().withMessage('Current password is required'),
    
    body('password_nueva')
      .notEmpty().withMessage('New password is required')
      .isLength({ min: 6 }).withMessage('New password must be at least 6 characters')
  ],

  getById: [
    param('id')
      .isInt().withMessage('Invalid ID')
  ],

  getByRol: [
    param('id_rol')
      .isInt().withMessage('Invalid role ID')
  ],

  getAll: [
    query('page')
      .optional()
      .isInt({ min: 1 }).withMessage('Page must be a positive number'),
    query('limit')
      .optional()
      .isInt({ min: 1, max: 100 }).withMessage('Limit must be between 1 and 100'),
    query('estado')
      .optional()
      .isBoolean().withMessage('Status must be true or false'),
    query('id_rol')
      .optional()
      .isInt().withMessage('Role must be a number')
  ]
};

module.exports = usuarioValidation;