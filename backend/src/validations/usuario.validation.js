// backend/src/validations/usuario.validation.js
const { body, param, query } = require('express-validator');

const usuarioValidation = {
  create: [
    body('nombre_completo')
      .notEmpty().withMessage('El nombre completo es requerido')
      .isLength({ max: 150 }).withMessage('Máximo 150 caracteres'),
    
    body('tipo_documento')
      .notEmpty().withMessage('El tipo de documento es requerido')
      .isIn(['CC', 'TI', 'CE', 'PASAPORTE']).withMessage('Tipo de documento no válido'),
    
    body('numero_documento')
      .notEmpty().withMessage('El número de documento es requerido')
      .isLength({ max: 50 }).withMessage('Máximo 50 caracteres')
      .matches(/^[0-9]+$/).withMessage('Solo números permitidos'),
    
    body('direccion')
      .optional()
      .isLength({ max: 200 }).withMessage('Máximo 200 caracteres'),
    
    body('tipo_sangre')
      .optional()
      .isIn(['O+', 'O-', 'A+', 'A-', 'B+', 'B-', 'AB+', 'AB-'])
      .withMessage('Tipo de sangre no válido'),
    
    body('eps')
      .optional()
      .isIn(['SURA', 'SANITAS', 'COMPENSAR', 'NUEVA_EPS', 'FAMISANAR', 'COOMEVA', 'SALUD_TOTAL'])
      .withMessage('EPS no válida'),
    
    body('genero')
      .optional()
      .isIn(['MASCULINO', 'FEMENINO', 'OTRO', 'PREFIERE_NO_DECIR'])
      .withMessage('Género no válido'),
    
    body('correo')
      .optional()
      .isEmail().withMessage('Correo electrónico no válido')
      .isLength({ max: 150 }).withMessage('Máximo 150 caracteres'),
    
    body('telefono')
      .optional()
      .isLength({ max: 50 }).withMessage('Máximo 50 caracteres'),
    
    body('password_hash')
      .notEmpty().withMessage('La contraseña es requerida')
      .isLength({ min: 6 }).withMessage('La contraseña debe tener al menos 6 caracteres'),
    
    body('id_rol')
      .notEmpty().withMessage('El rol es requerido')
      .isInt().withMessage('El rol debe ser un número')
  ],

  update: [
    param('id')
      .isInt().withMessage('ID inválido'),
    
    body('nombre_completo')
      .optional()
      .isLength({ max: 150 }).withMessage('Máximo 150 caracteres'),
    
    body('tipo_documento')
      .optional()
      .isIn(['CC', 'TI', 'CE', 'PASAPORTE']).withMessage('Tipo de documento no válido'),
    
    body('numero_documento')
      .optional()
      .isLength({ max: 50 }).withMessage('Máximo 50 caracteres')
      .matches(/^[0-9]+$/).withMessage('Solo números permitidos'),
    
    body('direccion')
      .optional()
      .isLength({ max: 200 }).withMessage('Máximo 200 caracteres'),
    
    body('tipo_sangre')
      .optional()
      .isIn(['O+', 'O-', 'A+', 'A-', 'B+', 'B-', 'AB+', 'AB-'])
      .withMessage('Tipo de sangre no válido'),
    
    body('eps')
      .optional()
      .isIn(['SURA', 'SANITAS', 'COMPENSAR', 'NUEVA_EPS', 'FAMISANAR', 'COOMEVA', 'SALUD_TOTAL'])
      .withMessage('EPS no válida'),
    
    body('genero')
      .optional()
      .isIn(['MASCULINO', 'FEMENINO', 'OTRO', 'PREFIERE_NO_DECIR'])
      .withMessage('Género no válido'),
    
    body('correo')
      .optional()
      .isEmail().withMessage('Correo electrónico no válido')
      .isLength({ max: 150 }).withMessage('Máximo 150 caracteres'),
    
    body('telefono')
      .optional()
      .isLength({ max: 50 }).withMessage('Máximo 50 caracteres'),
    
    body('password_hash')
      .optional()
      .isLength({ min: 6 }).withMessage('La contraseña debe tener al menos 6 caracteres'),
    
    body('id_rol')
      .optional()
      .isInt().withMessage('El rol debe ser un número')
  ],

  cambiarPassword: [
    param('id')
      .isInt().withMessage('ID inválido'),
    
    body('password_actual')
      .notEmpty().withMessage('La contraseña actual es requerida'),
    
    body('password_nueva')
      .notEmpty().withMessage('La nueva contraseña es requerida')
      .isLength({ min: 6 }).withMessage('La nueva contraseña debe tener al menos 6 caracteres')
  ],

  getById: [
    param('id')
      .isInt().withMessage('ID inválido')
  ],

  getByRol: [
    param('id_rol')
      .isInt().withMessage('ID de rol inválido')
  ],

  getAll: [
    query('page')
      .optional()
      .isInt({ min: 1 }).withMessage('La página debe ser un número positivo'),
    query('limit')
      .optional()
      .isInt({ min: 1, max: 100 }).withMessage('El límite debe ser entre 1 y 100'),
    query('estado')
      .optional()
      .isBoolean().withMessage('El estado debe ser true o false'),
    query('id_rol')
      .optional()
      .isInt().withMessage('El rol debe ser un número')
  ]
};

module.exports = usuarioValidation;