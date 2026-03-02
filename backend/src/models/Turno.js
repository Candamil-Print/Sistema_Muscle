// backend/src/models/Turno.js
const { DataTypes } = require('sequelize');
const { sequelize } = require('../../config/db');

const Turno = sequelize.define('Turno', {
  id_turno: {
    type: DataTypes.INTEGER,
    primaryKey: true,
    autoIncrement: true
  },
  id_usuario: {
    type: DataTypes.INTEGER,
    allowNull: false,
    references: {
      model: 'usuarios',
      key: 'id_usuario'
    }
  },
  id_tipo_turno: {
    type: DataTypes.INTEGER,
    allowNull: false,
    references: {
      model: 'tipos_turno',
      key: 'id_tipo_turno'
    }
  },
  fecha_inicio: {
    type: DataTypes.DATE,
    defaultValue: DataTypes.NOW
  },
  fecha_fin: {
    type: DataTypes.DATE,
    allowNull: true
  },
  estado: {
    type: DataTypes.ENUM('ABIERTO', 'CERRADO'),
    defaultValue: 'ABIERTO'
  }
}, {
  tableName: 'turnos',
  timestamps: false
});

module.exports = Turno;