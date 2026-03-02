// backend/src/models/Caja.js
const { DataTypes } = require('sequelize');
const { sequelize } = require('../../config/db');

const Caja = sequelize.define('Caja', {
  id_caja: {
    type: DataTypes.INTEGER,
    primaryKey: true,
    autoIncrement: true
  },
  fecha_apertura: {
    type: DataTypes.DATE,
    allowNull: false,
    defaultValue: DataTypes.NOW
  },
  fecha_cierre: {
    type: DataTypes.DATE,
    allowNull: true
  },
  monto_apertura: {
    type: DataTypes.DECIMAL(10, 2),
    allowNull: false,
    validate: {
      min: { args: [0], msg: 'El monto de apertura no puede ser negativo' }
    }
  },
  monto_cierre: {
    type: DataTypes.DECIMAL(10, 2),
    allowNull: true
  },
  total_efectivo: {
    type: DataTypes.DECIMAL(10, 2),
    defaultValue: 0
  },
  total_transferencia: {
    type: DataTypes.DECIMAL(10, 2),
    defaultValue: 0
  },
  estado: {
    type: DataTypes.ENUM('ABIERTA', 'CERRADA'),
    allowNull: false,
    defaultValue: 'ABIERTA'
  },
  id_usuario_apertura: {
    type: DataTypes.INTEGER,
    allowNull: false,
    references: {
      model: 'usuarios',
      key: 'id_usuario'
    }
  },
  id_usuario_cierre: {
    type: DataTypes.INTEGER,
    allowNull: true,
    references: {
      model: 'usuarios',
      key: 'id_usuario'
    }
  },
  id_turno: {
    type: DataTypes.INTEGER,
    allowNull: true,
    references: {
      model: 'turnos',
      key: 'id_turno'
    }
  }
}, {
  tableName: 'caja',
  timestamps: false
});

module.exports = Caja;