// backend/src/middlewares/auth.middleware.js
const jwt = require('jsonwebtoken');
const env = require('../../config/environment');
const { Usuario } = require('../models');

const authMiddleware = {
  // Verificar token JWT
  async verifyToken(req, res, next) {
    try {
      const token = req.headers.authorization?.split(' ')[1];
      
      if (!token) {
        return res.status(401).json({
          success: false,
          message: 'No authentication token was provided'
        });
      }

      const decoded = jwt.verify(token, env.JWT_SECRET);
      
      // Verificar que el usuario existe y está activo
      const usuario = await Usuario.findByPk(decoded.id, {
        attributes: { exclude: ['password_hash'] }
      });

      if (!usuario) {
        return res.status(401).json({
          success: false,
          message: 'User not found'
        });
      }

      if (!usuario.estado) {
        return res.status(401).json({
          success: false,
          message: 'User inactive'
        });
      }

      req.usuario = usuario;
      next();
    } catch (error) {
      if (error.name === 'JsonWebTokenError') {
        return res.status(401).json({
          success: false,
          message: 'Invalide token'
        });
      }
      if (error.name === 'TokenExpiredError') {
        return res.status(401).json({
          success: false,
          message: 'Expired token'
        });
      }
      next(error);
    }
  },

  // Verificar si es administrador
  async verifyAdmin(req, res, next) {
    try {
      if (!req.usuario) {
        return res.status(401).json({
          success: false,
          message: 'unauthenticated'
        });
      }

      // Asumiendo que id_rol=1 es ADMINISTRADOR
      if (req.usuario.id_rol !== 1) {
        return res.status(403).json({
          success: false,
          message: 'Access denied, Administrator permissions required'
        });
      }

      next();
    } catch (error) {
      next(error);
    }
  },

  // Verificar si es recepcionista
  async verifyRecepcionista(req, res, next) {
    try {
      if (!req.usuario) {
        return res.status(401).json({
          success: false,
          message: 'Unauthenticated'
        });
      }

      // Asumiendo que id_rol=2 es RECEPCIONISTA
      if (req.usuario.id_rol !== 2) {
        return res.status(403).json({
          success: false,
          message: 'Access denied, Receptionist permissions required'
        });
      }

      next();
    } catch (error) {
      next(error);
    }
  },

  // Verificar si es administrador o el mismo usuario
  async verifyOwnUser(req, res, next) {
    try {
      if (!req.usuario) {
        return res.status(401).json({
          success: false,
          message: 'Unauthenticated'
        });
      }

      // Si es admin, puede acceder
      if (req.usuario.id_rol === 1) {
        return next();
      }

      // Si no es admin, verificar que sea el mismo usuario
      if (req.usuario.id_usuario !== parseInt(req.params.id)) {
        return res.status(403).json({
          success: false,
          message: 'You do not have permissions to access this resource'
        });
      }

      next();
    } catch (error) {
      next(error);
    }
  }
};

module.exports = authMiddleware;