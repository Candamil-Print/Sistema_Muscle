// backend/src/services/caja.service.js
const { Caja, Usuario, Turno, TipoTurno, Venta, DetalleVenta, sequelize } = require('../models');
const { Op } = require('sequelize');

class CajaService {
  // Obtener todas las cajas con paginación y filtros
  async findAll(query = {}) {
    const { 
      page = 1, 
      limit = 10, 
      fecha_desde, 
      fecha_hasta,
      estado,
      id_usuario,
      sortBy = 'fecha_apertura',
      order = 'DESC'
    } = query;

    const offset = (page - 1) * limit;
    
    // Construir filtros
    const where = {};
    
    if (fecha_desde || fecha_hasta) {
      where.fecha_apertura = {};
      if (fecha_desde) where.fecha_apertura[Op.gte] = new Date(fecha_desde);
      if (fecha_hasta) where.fecha_apertura[Op.lte] = new Date(fecha_hasta);
    }
    
    if (estado) where.estado = estado;
    if (id_usuario) {
      where[Op.or] = [
        { id_usuario_apertura: id_usuario },
        { id_usuario_cierre: id_usuario }
      ];
    }

    const { count, rows } = await Caja.findAndCountAll({
      where,
      include: [
        {
          model: Usuario,
          as: 'usuario_apertura',
          attributes: ['id_usuario', 'nombre_completo']
        },
        {
          model: Usuario,
          as: 'usuario_cierre',
          attributes: ['id_usuario', 'nombre_completo']
        },
        {
          model: Turno,
          as: 'turno',
          include: [{
            model: TipoTurno,
            as: 'tipo_turno'
          }]
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

  // Obtener caja por ID
  async findById(id) {
    const caja = await Caja.findByPk(id, {
      include: [
        {
          model: Usuario,
          as: 'usuario_apertura',
          attributes: ['id_usuario', 'nombre_completo']
        },
        {
          model: Usuario,
          as: 'usuario_cierre',
          attributes: ['id_usuario', 'nombre_completo']
        },
        {
          model: Turno,
          as: 'turno',
          include: [{
            model: TipoTurno,
            as: 'tipo_turno'
          }]
        }
      ]
    });

    if (!caja) {
      throw new Error('Caja no encontrada');
    }

    return caja;
  }

  // Obtener caja activa por turno
  async getCajaActivaPorTurno(id_turno) {
    const caja = await Caja.findOne({
      where: {
        id_turno,
        estado: 'ABIERTA'
      },
      include: [
        {
          model: Usuario,
          as: 'usuario_apertura'
        }
      ]
    });

    return caja;
  }

  // Obtener caja activa por usuario
  async getCajaActivaPorUsuario(id_usuario) {
    const caja = await Caja.findOne({
      where: {
        id_usuario_apertura: id_usuario,
        estado: 'ABIERTA'
      },
      include: [
        {
          model: Turno,
          as: 'turno'
        }
      ]
    });

    return caja;
  }

  // Abrir caja
  async abrirCaja(data) {
    const { id_usuario_apertura, id_turno, monto_apertura } = data;

    // Verificar si ya tiene una caja abierta
    const cajaActiva = await this.getCajaActivaPorUsuario(id_usuario_apertura);
    if (cajaActiva) {
      throw new Error('El usuario ya tiene una caja abierta');
    }

    // Verificar que el turno existe y está abierto
    const turno = await Turno.findByPk(id_turno);
    if (!turno) {
      throw new Error('Turno no encontrado');
    }
    if (turno.estado !== 'ABIERTO') {
      throw new Error('El turno debe estar abierto para abrir caja');
    }

    // Verificar que no hay otra caja abierta para este turno
    const cajaPorTurno = await this.getCajaActivaPorTurno(id_turno);
    if (cajaPorTurno) {
      throw new Error('Ya hay una caja abierta para este turno');
    }

    // Verificar que el usuario existe y está activo
    const usuario = await Usuario.findByPk(id_usuario_apertura);
    if (!usuario || !usuario.estado) {
      throw new Error('Usuario no válido o inactivo');
    }

    // Crear caja
    const caja = await Caja.create({
      id_usuario_apertura,
      id_turno,
      monto_apertura,
      fecha_apertura: new Date(),
      estado: 'ABIERTA',
      total_efectivo: 0,
      total_transferencia: 0
    });

    return this.findById(caja.id_caja);
  }

  // Cerrar caja
  async cerrarCaja(id, data) {
    const { id_usuario_cierre, monto_cierre, total_efectivo, total_transferencia } = data;

    const caja = await Caja.findByPk(id);
    
    if (!caja) {
      throw new Error('Caja no encontrada');
    }

    if (caja.estado === 'CERRADA') {
      throw new Error('La caja ya está cerrada');
    }

    // Verificar que el usuario que cierra existe
    const usuario = await Usuario.findByPk(id_usuario_cierre);
    if (!usuario || !usuario.estado) {
      throw new Error('Usuario no válido para cierre');
    }

    // Obtener ventas de esta caja para verificar montos
    const ventas = await Venta.findAll({
      where: { id_caja: id },
      include: [{
        model: DetalleVenta,
        as: 'detalles',
        include: [{
          model: MetodoPago,
          as: 'metodo_pago_detalle'
        }]
      }]
    });

    // Calcular totales reales de ventas
    let totalEfectivoVentas = 0;
    let totalTransferenciaVentas = 0;

    ventas.forEach(venta => {
      venta.detalles.forEach(detalle => {
        const monto = parseFloat(detalle.subtotal);
        if (detalle.metodo_pago_detalle.nombre === 'EFECTIVO') {
          totalEfectivoVentas += monto;
        } else if (detalle.metodo_pago_detalle.nombre === 'TRANSFERENCIA') {
          totalTransferenciaVentas += monto;
        }
      });
    });

    // Validar que los montos coincidan (opcional - puede haber diferencia por gastos)
    const montoEsperado = parseFloat(caja.monto_apertura) + totalEfectivoVentas;
    if (Math.abs(parseFloat(monto_cierre) - montoEsperado) > 1) { // tolerancia de $1
      throw new Error(`El monto de cierre no coincide. Esperado: ${montoEsperado}, Ingresado: ${monto_cierre}`);
    }

    await caja.update({
      fecha_cierre: new Date(),
      monto_cierre,
      total_efectivo: total_efectivo || totalEfectivoVentas,
      total_transferencia: total_transferencia || totalTransferenciaVentas,
      id_usuario_cierre,
      estado: 'CERRADA'
    });

    return this.findById(id);
  }

  // Obtener resumen de caja (ventas del día asociadas a esta caja)
  async getResumenCaja(id) {
    const caja = await this.findById(id);
    
    const ventas = await Venta.findAll({
      where: { id_caja: id },
      include: [{
        model: DetalleVenta,
        as: 'detalles',
        include: [{
          model: Producto,
          as: 'producto',
          attributes: ['id_producto', 'nombre']
        }]
      }],
      order: [['fecha', 'ASC']]
    });

    let totalVentas = 0;
    let cantidadVentas = ventas.length;
    let ventasPorMetodo = {
      efectivo: 0,
      transferencia: 0
    };

    const ventasDetalle = ventas.map(venta => {
      let ventaTotal = 0;
      venta.detalles.forEach(detalle => {
        ventaTotal += parseFloat(detalle.subtotal);
      });
      totalVentas += ventaTotal;

      // Determinar método de pago (simplificado - asumiendo una venta con un solo método)
      if (venta.detalles.length > 0) {
        const metodo = venta.detalles[0].metodo_pago_detalle?.nombre;
        if (metodo === 'EFECTIVO') {
          ventasPorMetodo.efectivo += ventaTotal;
        } else if (metodo === 'TRANSFERENCIA') {
          ventasPorMetodo.transferencia += ventaTotal;
        }
      }

      return {
        id_venta: venta.id_venta,
        fecha: venta.fecha,
        total: ventaTotal,
        productos: venta.detalles.length
      };
    });

    return {
      caja: {
        id: caja.id_caja,
        fecha_apertura: caja.fecha_apertura,
        fecha_cierre: caja.fecha_cierre,
        estado: caja.estado,
        monto_apertura: caja.monto_apertura,
        monto_cierre: caja.monto_cierre
      },
      usuario: caja.usuario_apertura,
      turno: caja.turno,
      resumen: {
        cantidad_ventas,
        total_ventas,
        ventas_por_metodo: ventasPorMetodo,
        diferencia: caja.monto_cierre ? 
          parseFloat(caja.monto_cierre) - (parseFloat(caja.monto_apertura) + totalVentas) : 0
      },
      ventas: ventasDetalle
    };
  }

  // Obtener estadísticas de cajas
  async getEstadisticas({ fecha_desde, fecha_hasta }) {
    const where = {};
    
    if (fecha_desde || fecha_hasta) {
      where.fecha_apertura = {};
      if (fecha_desde) where.fecha_apertura[Op.gte] = new Date(fecha_desde);
      if (fecha_hasta) where.fecha_apertura[Op.lte] = new Date(fecha_hasta);
    }

    const cajas = await Caja.findAll({
      where,
      include: [{
        model: Usuario,
        as: 'usuario_apertura'
      }]
    });

    const totalCajas = cajas.length;
    const cajasAbiertas = cajas.filter(c => c.estado === 'ABIERTA').length;
    const cajasCerradas = cajas.filter(c => c.estado === 'CERRADA').length;

    let totalApertura = 0;
    let totalCierre = 0;
    let totalEfectivo = 0;
    let totalTransferencia = 0;

    cajas.forEach(caja => {
      totalApertura += parseFloat(caja.monto_apertura || 0);
      totalCierre += parseFloat(caja.monto_cierre || 0);
      totalEfectivo += parseFloat(caja.total_efectivo || 0);
      totalTransferencia += parseFloat(caja.total_transferencia || 0);
    });

    return {
      periodo: {
        desde: fecha_desde || 'siempre',
        hasta: fecha_hasta || 'ahora'
      },
      total_cajas,
      cajas_abiertas: cajasAbiertas,
      cajas_cerradas: cajasCerradas,
      montos: {
        total_apertura: totalApertura,
        total_cierre: totalCierre,
        total_efectivo,
        total_transferencia,
        diferencia_total: totalCierre - totalApertura
      }
    };
  }
}

module.exports = new CajaService();