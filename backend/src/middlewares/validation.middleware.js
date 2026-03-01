// backend/src/middlewares/validation.middleware.js
const { validationResult } = require('express-validator');

const validationMiddleware = {
  validate(validations, source = 'body') {
    return async (req, res, next) => {
      // Ejecutar validaciones
      for (let validation of validations) {
        if (source === 'body') {
          await validation.run(req);
        } else if (source === 'params') {
          await validation.run(req);
        } else if (source === 'query') {
          await validation.run(req);
        }
      }

      // Verificar errores
      const errors = validationResult(req);
      if (errors.isEmpty()) {
        return next();
      }

      // Formatear errores
      const formattedErrors = errors.array().map(error => ({
        field: error.path,
        message: error.msg
      }));

      return res.status(400).json({
        success: false,
        message: 'Error de validación',
        errors: formattedErrors
      });
    };
  }
};

module.exports = validationMiddleware;