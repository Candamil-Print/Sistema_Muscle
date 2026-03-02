// backend/src/models/TipoTurno.js
const { DataTypes } = require('sequelize');
const { sequelize } = require('../../config/db');

const TipoTurno = sequelize.define('TipoTurno', {
  id_tipo_turno: {
    type: DataTypes.INTEGER,
    primaryKey: true,
    autoIncrement: true
  },
  nombre: {
    type: DataTypes.STRING(50),
    allowNull: false,
    validate: {
      notEmpty: { msg: 'El nombre del turno es requerido' }
    }
  },
  hora_inicio: {
    type: DataTypes.TIME,
    allowNull: false
  },
  hora_fin: {
    type: DataTypes.TIME,
    allowNull: false
  }
}, {
  tableName: 'tipos_turno',
  timestamps: false
});

module.exports = TipoTurno;