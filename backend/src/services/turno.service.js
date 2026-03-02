// backend/src/services/turno.service.js
const { Turno, Usuario, TipoTurno } = require('../models');
const { Op } = require('sequelize');

class TurnoService {
  // Obtener todos los turnos con paginación y filtros
  async findAll(query = {}) {
    const { 
      page = 1, 
      limit = 10, 
      fecha,
      id_usuario,
      estado,
      sortBy = 'fecha_inicio',
      order = 'DESC'
    } = query;

    const offset = (page - 1) * limit;
    
    // Construir filtros
    const where = {};
    
    if (fecha) {
      const fechaInicio = new Date(fecha);
      fechaInicio.setHours(0, 0, 0, 0);
      const fechaFin = new Date(fecha);
      fechaFin.setHours(23, 59, 59, 999);
      
      where.fecha_inicio = {
        [Op.between]: [fechaInicio, fechaFin]
      };
    }
    
    if (id_usuario) where.id_usuario = id_usuario;
    if (estado) where.estado = estado;

    const { count, rows } = await Turno.findAndCountAll({
      where,
      include: [
        {
          model: Usuario,
          as: 'usuario',
          attributes: ['id_usuario', 'nombre_completo', 'numero_documento']
        },
        {
          model: TipoTurno,
          as: 'tipo_turno',
          attributes: ['id_tipo_turno', 'nombre', 'hora_inicio', 'hora_fin']
        }
      ],
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

  // Obtener turno por ID
  async findById(id) {
    const turno = await Turno.findByPk(id, {
      include: [
        {
          model: Usuario,
          as: 'usuario',
          attributes: ['id_usuario', 'nombre_completo', 'numero_documento', 'correo']
        },
        {
          model: TipoTurno,
          as: 'tipo_turno'
        }
      ]
    });

    if (!turno) {
      throw new Error('Turno no encontrado');
    }

    return turno;
  }

  // Obtener turno activo de un usuario
  async getTurnoActivo(id_usuario) {
    const turno = await Turno.findOne({
      where: {
        id_usuario,
        estado: 'ABIERTO'
      },
      include: [
        {
          model: Usuario,
          as: 'usuario'
        },
        {
          model: TipoTurno,
          as: 'tipo_turno'
        }
      ]
    });

    return turno;
  }

  // Abrir turno
  async abrirTurno(data) {
    const { id_usuario, id_tipo_turno } = data;

    // Verificar si ya tiene un turno abierto
    const turnoActivo = await this.getTurnoActivo(id_usuario);
    if (turnoActivo) {
      throw new Error('El usuario ya tiene un turno abierto');
    }

    // Verificar que el usuario existe y está activo
    const usuario = await Usuario.findByPk(id_usuario);
    if (!usuario) {
      throw new Error('Usuario no encontrado');
    }
    if (!usuario.estado) {
      throw new Error('Usuario inactivo');
    }

    // Verificar que el tipo de turno existe
    const tipoTurno = await TipoTurno.findByPk(id_tipo_turno);
    if (!tipoTurno) {
      throw new Error('Tipo de turno no encontrado');
    }

    // Crear turno
    const turno = await Turno.create({
      id_usuario,
      id_tipo_turno,
      fecha_inicio: new Date(),
      estado: 'ABIERTO'
    });

    return this.findById(turno.id_turno);
  }

  // Cerrar turno
  async cerrarTurno(id) {
    const turno = await Turno.findByPk(id);
    
    if (!turno) {
      throw new Error('Turno no encontrado');
    }

    if (turno.estado === 'CERRADO') {
      throw new Error('El turno ya está cerrado');
    }

    await turno.update({
      fecha_fin: new Date(),
      estado: 'CERRADO'
    });

    return this.findById(id);
  }

  // Obtener turnos por fecha
  async getTurnosPorFecha(fecha) {
    const fechaInicio = new Date(fecha);
    fechaInicio.setHours(0, 0, 0, 0);
    const fechaFin = new Date(fecha);
    fechaFin.setHours(23, 59, 59, 999);

    const turnos = await Turno.findAll({
      where: {
        fecha_inicio: {
          [Op.between]: [fechaInicio, fechaFin]
        }
      },
      include: [
        {
          model: Usuario,
          as: 'usuario',
          attributes: ['id_usuario', 'nombre_completo']
        },
        {
          model: TipoTurno,
          as: 'tipo_turno'
        }
      ],
      order: [['fecha_inicio', 'ASC']]
    });

    return turnos;
  }

  // Obtener turnos por usuario
  async getTurnosPorUsuario(id_usuario, { limite = 10 }) {
    const turnos = await Turno.findAll({
      where: { id_usuario },
      include: [
        {
          model: TipoTurno,
          as: 'tipo_turno'
        }
      ],
      limit: limite,
      order: [['fecha_inicio', 'DESC']]
    });

    return turnos;
  }

  // Estadísticas de turnos
  async getEstadisticas({ fecha_desde, fecha_hasta }) {
    const where = {};
    
    if (fecha_desde || fecha_hasta) {
      where.fecha_inicio = {};
      if (fecha_desde) where.fecha_inicio[Op.gte] = new Date(fecha_desde);
      if (fecha_hasta) where.fecha_inicio[Op.lte] = new Date(fecha_hasta);
    }

    const turnos = await Turno.findAll({
      where,
      include: [
        {
          model: Usuario,
          as: 'usuario'
        },
        {
          model: TipoTurno,
          as: 'tipo_turno'
        }
      ]
    });

    const totalTurnos = turnos.length;
    const turnosAbiertos = turnos.filter(t => t.estado === 'ABIERTO').length;
    const turnosCerrados = turnos.filter(t => t.estado === 'CERRADO').length;

    // Turnos por tipo
    const porTipo = {};
    turnos.forEach(turno => {
      const tipo = turno.tipo_turno.nombre;
      porTipo[tipo] = (porTipo[tipo] || 0) + 1;
    });

    // Duración promedio de turnos cerrados (en horas)
    const turnosCerradosList = turnos.filter(t => t.estado === 'CERRADO' && t.fecha_fin);
    let duracionPromedio = 0;
    
    if (turnosCerradosList.length > 0) {
      const sumaDuracion = turnosCerradosList.reduce((sum, t) => {
        const inicio = new Date(t.fecha_inicio);
        const fin = new Date(t.fecha_fin);
        const horas = (fin - inicio) / (1000 * 60 * 60);
        return sum + horas;
      }, 0);
      duracionPromedio = sumaDuracion / turnosCerradosList.length;
    }

    return {
      periodo: {
        desde: fecha_desde || 'siempre',
        hasta: fecha_hasta || 'ahora'
      },
      total_turnos,
      turnos_abiertos: turnosAbiertos,
      turnos_cerrados: turnosCerrados,
      duracion_promedio_horas: duracionPromedio.toFixed(2),
      turnos_por_tipo: porTipo
    };
  }
}

module.exports = new TurnoService();