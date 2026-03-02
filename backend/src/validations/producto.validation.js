// backend/src/validations/producto.validation.js
const { body, param, query } = require('express-validator');

const productoValidation = {
  create: [
    body('nombre')
      .notEmpty().withMessage('El nombre es requerido')
      .isLength({ max: 150 }).withMessage('Máximo 150 caracteres'),
    
    body('tipo_producto')
      .notEmpty().withMessage('El tipo de producto es requerido')
      .isIn(['SNACKS', 'SUPLEMENTOS', 'BEBIDAS'])
      .withMessage('Tipo no válido. Debe ser: SNACKS, SUPLEMENTOS o BEBIDAS'),
    
    body('precio_costo')
      .notEmpty().withMessage('El precio de costo es requerido')
      .isDecimal({ min: 0.01 }).withMessage('Debe ser un número mayor a 0'),
    
    body('precio_sugerido')
      .notEmpty().withMessage('El precio sugerido es requerido')
      .isDecimal({ min: 0.01 }).withMessage('Debe ser un número mayor a 0')
      .custom((value, { req }) => {
        if (parseFloat(value) <= parseFloat(req.body.precio_costo)) {
          throw new Error('El precio sugerido debe ser mayor al precio de costo');
        }
        return true;
      }),
    
    body('imagen_url')
      .optional()
      .isURL().withMessage('Debe ser una URL válida')
  ],

  update: [
    param('id')
      .isInt().withMessage('ID inválido'),
    
    body('nombre')
      .optional()
      .isLength({ max: 150 }).withMessage('Máximo 150 caracteres'),
    
    body('tipo_producto')
      .optional()
      .isIn(['SNACKS', 'SUPLEMENTOS', 'BEBIDAS'])
      .withMessage('Tipo no válido'),
    
    body('precio_costo')
      .optional()
      .isDecimal({ min: 0.01 }).withMessage('Debe ser un número mayor a 0'),
    
    body('precio_sugerido')
      .optional()
      .isDecimal({ min: 0.01 }).withMessage('Debe ser un número mayor a 0')
      .custom((value, { req }) => {
        const precioCosto = req.body.precio_costo;
        if (precioCosto && parseFloat(value) <= parseFloat(precioCosto)) {
          throw new Error('El precio sugerido debe ser mayor al precio de costo');
        }
        return true;
      }),
    
    body('imagen_url')
      .optional()
      .isURL().withMessage('Debe ser una URL válida')
  ],

  actualizarPrecios: [
    body('porcentaje')
      .notEmpty().withMessage('El porcentaje es requerido')
      .isFloat().withMessage('Debe ser un número válido'),
    
    body('tipo')
      .optional()
      .isIn(['SNACKS', 'SUPLEMENTOS', 'BEBIDAS'])
      .withMessage('Tipo no válido')
  ],

  getByTipo: [
    param('tipo')
      .isIn(['SNACKS', 'SUPLEMENTOS', 'BEBIDAS'])
      .withMessage('Tipo no válido')
  ],

  getAll: [
    query('page')
      .optional()
      .isInt({ min: 1 }).withMessage('La página debe ser un número positivo'),
    query('limit')
      .optional()
      .isInt({ min: 1, max: 100 }).withMessage('El límite debe ser entre 1 y 100'),
    query('tipo')
      .optional()
      .isIn(['SNACKS', 'SUPLEMENTOS', 'BEBIDAS']).withMessage('Tipo no válido'),
    query('precio_min')
      .optional()
      .isFloat({ min: 0 }).withMessage('Precio mínimo inválido'),
    query('precio_max')
      .optional()
      .isFloat({ min: 0 }).withMessage('Precio máximo inválido')
  ]
};

module.exports = productoValidation;