// backend/src/services/venta.service.js
const { Venta, DetalleVenta, Producto, Usuario, MetodoPago, Caja, Turno, sequelize } = require('../models');
const { Op } = require('sequelize');

class VentaService {
  // Obtener todas las ventas con paginación y filtros
  async findAll(query = {}) {
    const { 
      page = 1, 
      limit = 10, 
      fecha_desde, 
      fecha_hasta,
      id_usuario,
      id_caja,
      sortBy = 'fecha',
      order = 'DESC'
    } = query;

    const offset = (page - 1) * limit;
    
    // Construir filtros
    const where = {};
    
    if (fecha_desde || fecha_hasta) {
      where.fecha = {};
      if (fecha_desde) where.fecha[Op.gte] = new Date(fecha_desde);
      if (fecha_hasta) where.fecha[Op.lte] = new Date(fecha_hasta);
    }
    
    if (id_usuario) where.id_usuario = id_usuario;
    if (id_caja) where.id_caja = id_caja;

    const { count, rows } = await Venta.findAndCountAll({
      where,
      include: [
        {
          model: Usuario,
          as: 'usuario',
          attributes: ['id_usuario', 'nombre_completo']
        },
        {
          model: Caja,
          as: 'caja',
          attributes: ['id_caja', 'estado']
        },
        {
          model: DetalleVenta,
          as: 'detalles',
          include: [
            {
              model: Producto,
              as: 'producto',
              attributes: ['id_producto', 'nombre', 'tipo_producto']
            },
            {
              model: MetodoPago,
              as: 'metodo_pago_detalle',
              attributes: ['id_metodo', 'nombre']
            }
          ]
        }
      ],
      limit: parseInt(limit),
      offset: parseInt(offset),
      order: [[sortBy, order]]
    });

    // Calcular totales por venta
    const ventasConTotales = rows.map(venta => {
      const ventaJson = venta.toJSON();
      const total = ventaJson.detalles.reduce((sum, detalle) => 
        sum + parseFloat(detalle.subtotal), 0
      );
      return { ...ventaJson, total };
    });

    return {
      total: count,
      page: parseInt(page),
      limit: parseInt(limit),
      totalPages: Math.ceil(count / limit),
      data: ventasConTotales
    };
  }

  // Obtener venta por ID
  async findById(id) {
    const venta = await Venta.findByPk(id, {
      include: [
        {
          model: Usuario,
          as: 'usuario',
          attributes: ['id_usuario', 'nombre_completo']
        },
        {
          model: Caja,
          as: 'caja'
        },
        {
          model: DetalleVenta,
          as: 'detalles',
          include: [
            {
              model: Producto,
              as: 'producto',
              attributes: ['id_producto', 'nombre', 'precio_sugerido']
            },
            {
              model: MetodoPago,
              as: 'metodo_pago_detalle',
              attributes: ['id_metodo', 'nombre']
            }
          ]
        }
      ]
    });

    if (!venta) {
      throw new Error('Venta no encontrada');
    }

    const ventaJson = venta.toJSON();
    const total = ventaJson.detalles.reduce((sum, detalle) => 
      sum + parseFloat(detalle.subtotal), 0
    );

    return { ...ventaJson, total };
  }

  // Validar que existe caja activa
  async validarCajaActiva(id_caja) {
    const caja = await Caja.findByPk(id_caja);
    
    if (!caja) {
      throw new Error('Caja no encontrada');
    }
    
    if (caja.estado !== 'ABIERTA') {
      throw new Error('La caja debe estar abierta para registrar ventas');
    }
    
    return caja;
  }

  // Crear venta (con transacción y actualización de caja)
  async create(data) {
    const { id_usuario, id_caja, id_turno, productos } = data;
    
    // Validar que hay productos
    if (!productos || productos.length === 0) {
      throw new Error('Debe incluir al menos un producto');
    }

    // Validar que la caja existe y está activa
    const caja = await this.validarCajaActiva(id_caja);

    // Usar transacción para asegurar integridad
    const transaction = await sequelize.transaction();

    try {
      // 1. Crear la venta (cabecera)
      const venta = await Venta.create({
        id_usuario,
        id_caja,
        id_turno,
        fecha: new Date()
      }, { transaction });

      // Variables para acumular totales por método de pago
      let totalEfectivo = 0;
      let totalTransferencia = 0;
      
      // 2. Procesar cada producto
      const detalles = [];
      for (const item of productos) {
        const producto = await Producto.findByPk(item.id_producto, { transaction });
        
        if (!producto) {
          throw new Error(`Producto ${item.id_producto} no encontrado`);
        }

        const precio_unitario = item.precio_unitario || producto.precio_sugerido;
        const subtotal = precio_unitario * item.cantidad;

        // Acumular según método de pago
        if (item.metodo_pago === 1) { // EFECTIVO
          totalEfectivo += subtotal;
        } else if (item.metodo_pago === 2) { // TRANSFERENCIA
          totalTransferencia += subtotal;
        }

        // Crear detalle
        const detalle = await DetalleVenta.create({
          id_venta: venta.id_venta,
          id_producto: item.id_producto,
          cantidad: item.cantidad,
          precio_unitario,
          metodo_pago: item.metodo_pago,
          subtotal
        }, { transaction });

        detalles.push(detalle);
      }

      // 3. ACTUALIZAR CAJA: sumar los montos a los totales
      await caja.increment({
        total_efectivo: totalEfectivo,
        total_transferencia: totalTransferencia
      }, { transaction });

      await transaction.commit();

      // Retornar la venta completa
      return this.findById(venta.id_venta);

    } catch (error) {
      await transaction.rollback();
      throw error;
    }
  }

  // Obtener ventas por caja
  async getVentasPorCaja(id_caja, query = {}) {
    const { fecha_desde, fecha_hasta } = query;
    
    const where = { id_caja };
    
    if (fecha_desde || fecha_hasta) {
      where.fecha = {};
      if (fecha_desde) where.fecha[Op.gte] = new Date(fecha_desde);
      if (fecha_hasta) where.fecha[Op.lte] = new Date(fecha_hasta);
    }

    const ventas = await Venta.findAll({
      where,
      include: [
        {
          model: Usuario,
          as: 'usuario',
          attributes: ['nombre_completo']
        },
        {
          model: DetalleVenta,
          as: 'detalles',
          include: [{
            model: MetodoPago,
            as: 'metodo_pago_detalle'
          }]
        }
      ],
      order: [['fecha', 'ASC']]
    });

    let totalEfectivo = 0;
    let totalTransferencia = 0;
    let totalVentas = 0;

    const ventasDetalle = ventas.map(venta => {
      let ventaTotal = 0;
      venta.detalles.forEach(detalle => {
        ventaTotal += parseFloat(detalle.subtotal);
        
        if (detalle.metodo_pago_detalle.nombre === 'EFECTIVO') {
          totalEfectivo += parseFloat(detalle.subtotal);
        } else if (detalle.metodo_pago_detalle.nombre === 'TRANSFERENCIA') {
          totalTransferencia += parseFloat(detalle.subtotal);
        }
      });
      totalVentas += ventaTotal;
      
      return {
        id_venta: venta.id_venta,
        fecha: venta.fecha,
        total: ventaTotal,
        usuario: venta.usuario.nombre_completo
      };
    });

    return {
      caja_id: id_caja,
      total_ventas: ventas.length,
      montos: {
        total: totalVentas,
        efectivo: totalEfectivo,
        transferencia: totalTransferencia
      },
      ventas: ventasDetalle
    };
  }

  // Obtener ventas del día (con información de caja)
  async getVentasDelDia(fecha = new Date()) {
    const inicio = new Date(fecha);
    inicio.setHours(0, 0, 0, 0);
    
    const fin = new Date(fecha);
    fin.setHours(23, 59, 59, 999);

    const ventas = await Venta.findAll({
      where: {
        fecha: {
          [Op.between]: [inicio, fin]
        }
      },
      include: [
        {
          model: Usuario,
          as: 'usuario',
          attributes: ['nombre_completo']
        },
        {
          model: Caja,
          as: 'caja',
          attributes: ['id_caja', 'estado']
        },
        {
          model: DetalleVenta,
          as: 'detalles',
          include: [
            {
              model: Producto,
              as: 'producto',
              attributes: ['nombre']
            },
            {
              model: MetodoPago,
              as: 'metodo_pago_detalle',
              attributes: ['nombre']
            }
          ]
        }
      ]
    });

    // Calcular totales por método de pago
    const totales = {
      efectivo: 0,
      transferencia: 0,
      total: 0
    };

    const ventasPorCaja = {};

    ventas.forEach(venta => {
      // Agrupar por caja
      if (!ventasPorCaja[venta.id_caja]) {
        ventasPorCaja[venta.id_caja] = {
          caja_id: venta.id_caja,
          total: 0,
          efectivo: 0,
          transferencia: 0
        };
      }

      venta.detalles.forEach(detalle => {
        const monto = parseFloat(detalle.subtotal);
        totales.total += monto;
        ventasPorCaja[venta.id_caja].total += monto;
        
        if (detalle.metodo_pago_detalle.nombre === 'EFECTIVO') {
          totales.efectivo += monto;
          ventasPorCaja[venta.id_caja].efectivo += monto;
        } else if (detalle.metodo_pago_detalle.nombre === 'TRANSFERENCIA') {
          totales.transferencia += monto;
          ventasPorCaja[venta.id_caja].transferencia += monto;
        }
      });
    });

    return {
      fecha: inicio.toISOString().split('T')[0],
      total_ventas: ventas.length,
      montos: totales,
      ventas_por_caja: Object.values(ventasPorCaja),
      ventas
    };
  }

  // Anular venta (requiere revertir montos en caja)
  async anularVenta(id) {
    const venta = await Venta.findByPk(id, {
      include: [
        { 
          model: DetalleVenta, 
          as: 'detalles',
          include: [{
            model: MetodoPago,
            as: 'metodo_pago_detalle'
          }]
        },
        {
          model: Caja,
          as: 'caja'
        }
      ]
    });

    if (!venta) {
      throw new Error('Venta no encontrada');
    }

    // Verificar que la caja no esté cerrada
    if (venta.caja.estado === 'CERRADA') {
      throw new Error('No se puede anular una venta de una caja cerrada');
    }

    const transaction = await sequelize.transaction();

    try {
      // Calcular montos a revertir por método de pago
      let totalEfectivo = 0;
      let totalTransferencia = 0;

      venta.detalles.forEach(detalle => {
        if (detalle.metodo_pago_detalle.nombre === 'EFECTIVO') {
          totalEfectivo += parseFloat(detalle.subtotal);
        } else if (detalle.metodo_pago_detalle.nombre === 'TRANSFERENCIA') {
          totalTransferencia += parseFloat(detalle.subtotal);
        }
      });

      // Revertir montos en caja (restar)
      await venta.caja.decrement({
        total_efectivo: totalEfectivo,
        total_transferencia: totalTransferencia
      }, { transaction });

      // Eliminar detalles y venta
      await DetalleVenta.destroy({ 
        where: { id_venta: id }, 
        transaction 
      });
      
      await venta.destroy({ transaction });

      await transaction.commit();
      return { 
        message: 'Venta anulada correctamente',
        montos_revertidos: {
          efectivo: totalEfectivo,
          transferencia: totalTransferencia
        }
      };

    } catch (error) {
      await transaction.rollback();
      throw error;
    }
  }
}

module.exports = new VentaService();