// backend/src/controllers/usuario.controller.js
const usuarioService = require('../services/usuario.service');

class UsuarioController {
  // Obtener todos los usuarios
  async getAll(req, res, next) {
    try {
      const result = await usuarioService.findAll(req.query);
      res.json({
        success: true,
        message: 'Usuarios obtenidos correctamente',
        ...result
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener usuario por ID
  async getById(req, res, next) {
    try {
      const usuario = await usuarioService.findById(req.params.id);
      res.json({
        success: true,
        message: 'Usuario obtenido correctamente',
        data: usuario
      });
    } catch (error) {
      next(error);
    }
  }

  // Crear usuario
  async create(req, res, next) {
    try {
      const usuario = await usuarioService.create(req.body);
      res.status(201).json({
        success: true,
        message: 'Usuario creado correctamente',
        data: usuario
      });
    } catch (error) {
      next(error);
    }
  }

  // Actualizar usuario
  async update(req, res, next) {
    try {
      const usuario = await usuarioService.update(req.params.id, req.body);
      res.json({
        success: true,
        message: 'Usuario actualizado correctamente',
        data: usuario
      });
    } catch (error) {
      next(error);
    }
  }

  // Eliminar usuario (soft delete)
  async delete(req, res, next) {
    try {
      const result = await usuarioService.delete(req.params.id);
      res.json({
        success: true,
        message: result.message
      });
    } catch (error) {
      next(error);
    }
  }

  // Eliminar usuario permanentemente
  async hardDelete(req, res, next) {
    try {
      const result = await usuarioService.hardDelete(req.params.id);
      res.json({
        success: true,
        message: result.message
      });
    } catch (error) {
      next(error);
    }
  }

  // Cambiar contraseña
  async cambiarPassword(req, res, next) {
    try {
      const { password_actual, password_nueva } = req.body;
      const result = await usuarioService.cambiarPassword(
        req.params.id, 
        password_actual, 
        password_nueva
      );
      res.json({
        success: true,
        message: result.message
      });
    } catch (error) {
      next(error);
    }
  }

  // Obtener usuarios por rol
  async getByRol(req, res, next) {
    try {
      const usuarios = await usuarioService.findByRol(req.params.id_rol);
      res.json({
        success: true,
        message: 'Usuarios obtenidos correctamente',
        data: usuarios
      });
    } catch (error) {
      next(error);
    }
  }
}

module.exports = new UsuarioController();