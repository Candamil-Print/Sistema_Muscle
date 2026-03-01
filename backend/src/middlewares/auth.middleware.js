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
          message: 'No se proporcionó token de autenticación'
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
          message: 'Usuario no encontrado'
        });
      }

      if (!usuario.estado) {
        return res.status(401).json({
          success: false,
          message: 'Usuario inactivo'
        });
      }

      req.usuario = usuario;
      next();
    } catch (error) {
      if (error.name === 'JsonWebTokenError') {
        return res.status(401).json({
          success: false,
          message: 'Token inválido'
        });
      }
      if (error.name === 'TokenExpiredError') {
        return res.status(401).json({
          success: false,
          message: 'Token expirado'
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
          message: 'No autenticado'
        });
      }

      // Asumiendo que id_rol=1 es ADMINISTRADOR
      if (req.usuario.id_rol !== 1) {
        return res.status(403).json({
          success: false,
          message: 'Acceso denegado. Se requieren permisos de administrador'
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
          message: 'No autenticado'
        });
      }

      // Asumiendo que id_rol=2 es RECEPCIONISTA
      if (req.usuario.id_rol !== 2) {
        return res.status(403).json({
          success: false,
          message: 'Acceso denegado. Se requieren permisos de recepcionista'
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
          message: 'No autenticado'
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
          message: 'No tienes permiso para acceder a este recurso'
        });
      }

      next();
    } catch (error) {
      next(error);
    }
  }
};

module.exports = authMiddleware;