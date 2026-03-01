// backend/src/services/usuario.service.js
const { Usuario, Rol } = require('../models');
const { Op } = require('sequelize');

class UsuarioService {
  // Obtener todos los usuarios con paginación y filtros
  async findAll(query = {}) {
    const { 
      page = 1, 
      limit = 10, 
      search = '', 
      estado, 
      id_rol,
      sortBy = 'id_usuario',
      order = 'DESC'
    } = query;

    const offset = (page - 1) * limit;
    
    // Construir filtros
    const where = {};
    
    if (search) {
      where[Op.or] = [
        { nombre_completo: { [Op.like]: `%${search}%` } },
        { numero_documento: { [Op.like]: `%${search}%` } },
        { correo: { [Op.like]: `%${search}%` } }
      ];
    }
    
    if (estado !== undefined) {
      where.estado = estado === 'true' || estado === true;
    }
    
    if (id_rol) {
      where.id_rol = id_rol;
    }

    const { count, rows } = await Usuario.findAndCountAll({
      where,
      include: [{
        model: Rol,
        as: 'rol',
        attributes: ['id_rol', 'nombre']
      }],
      attributes: { exclude: ['password_hash'] },
      limit: parseInt(limit),
      offset: parseInt(offset),
      order: [[sortBy, order]]
    });

    return {
      total: count,
      page: parseInt(page),
      limit: parseInt(limit),
      totalPages: Math.ceil(count / limit),
      data: rows
    };
  }

  // Obtener usuario por ID
  async findById(id) {
    const usuario = await Usuario.findByPk(id, {
      include: [{
        model: Rol,
        as: 'rol',
        attributes: ['id_rol', 'nombre']
      }],
      attributes: { exclude: ['password_hash'] }
    });

    if (!usuario) {
      throw new Error('User not found');
    }

    return usuario;
  }

  // Crear usuario
  async create(data) {
    // Verificar si ya existe el documento
    const existeDocumento = await Usuario.findByDocumento(data.numero_documento);
    if (existeDocumento) {
      throw new Error(' A user with this document number already exists');
    }

    // Verift to email exist (si se proporciona)
    if (data.correo) {
      const existeEmail = await Usuario.findByEmail(data.correo);
      if (existeEmail) {
        throw new Error('There is alreay a user with this email address');
      }
    }

    // Verificar que el rol existe
    const rol = await Rol.findByPk(data.id_rol);
    if (!rol) {
      throw new Error('This rol expecificated not found');
    }

    const usuario = await Usuario.create(data);
    
    // Retornar usuario sin password
    return this.findById(usuario.id_usuario);
  }

  // Actualizar usuario
  async update(id, data) {
    const usuario = await Usuario.findByPk(id);
    
    if (!usuario) {
      throw new Error('User not found');
    }

    // Si se está actualizando el documento, verificar que no exista otro
    if (data.numero_documento && data.numero_documento !== usuario.numero_documento) {
      const existeDocumento = await Usuario.findOne({
        where: { 
          numero_documento: data.numero_documento,
          id_usuario: { [Op.ne]: id }
        }
      });
      if (existeDocumento) {
        throw new Error('There is alreay another user with this document number');
      }
    }

    // Si se está actualizando el email, verificar que no exista otro
    if (data.correo && data.correo !== usuario.correo) {
      const existeEmail = await Usuario.findOne({
        where: { 
          correo: data.correo,
          id_usuario: { [Op.ne]: id }
        }
      });
      if (existeEmail) {
        throw new Error('Thisi is already user with email');
      }
    }

    // Si se está actualizando el rol, verificar que existe
    if (data.id_rol && data.id_rol !== usuario.id_rol) {
      const rol = await Rol.findByPk(data.id_rol);
      if (!rol) {
        throw new Error('This rol especificated not found');
      }
    }

    await usuario.update(data);
    
    return this.findById(id);
  }

  // Eliminar usuario (soft delete - cambiar estado)
  async delete(id) {
    const usuario = await Usuario.findByPk(id);
    
    if (!usuario) {
      throw new Error('User not found');
    }

    // Soft delete: cambiar estado a false
    await usuario.update({ estado: false });
    
    return { message: 'User unenable successfully' };
  }

  // Eliminar usuario físicamente (solo para administradores)
  async hardDelete(id) {
    const usuario = await Usuario.findByPk(id);
    
    if (!usuario) {
      throw new Error('User not found');
    }

    await usuario.destroy();
    
    return { message: 'User deleted permanently' };
  }

  // Cambiar contraseña
  async cambiarPassword(id, passwordActual, passwordNueva) {
    const usuario = await Usuario.findByPk(id);
    
    if (!usuario) {
      throw new Error('User not found');
    }

    // Validar contraseña actual
    const esValida = await usuario.validarPassword(passwordActual);
    if (!esValida) {
      throw new Error('the current password is incorrect');
    }

    // Actualizar contraseña
    usuario.password_hash = passwordNueva;
    await usuario.save();

    return { message: 'Password update successfuly' };
  }

  // Obtener usuarios por rol
  async findByRol(id_rol) {
    return await Usuario.findAll({
      where: { id_rol, estado: true },
      include: [{
        model: Rol,
        as: 'rol',
        attributes: ['id_rol', 'nombre']
      }],
      attributes: { exclude: ['password_hash'] }
    });
  }
}

module.exports = new UsuarioService();