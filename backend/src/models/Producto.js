// backend/src/models/Producto.js
const { DataTypes } = require('sequelize');
const { sequelize } = require('../../config/db');

const Producto = sequelize.define('Producto', {
  id_producto: {
    type: DataTypes.INTEGER,
    primaryKey: true,
    autoIncrement: true
  },
  nombre: {
    type: DataTypes.STRING(150),
    allowNull: false,
    validate: {
      notEmpty: { msg: 'El nombre del producto es requerido' }
    }
  },
  tipo_producto: {
    type: DataTypes.ENUM('SNACKS', 'SUPLEMENTOS', 'BEBIDAS'),
    allowNull: false
  },
  precio_costo: {
    type: DataTypes.DECIMAL(10, 2),
    allowNull: false,
    validate: {
      isDecimal: { msg: 'El precio costo debe ser un número decimal' },
      min: { args: [0.01], msg: 'El precio costo debe ser mayor a 0' }
    }
  },
  precio_sugerido: {
    type: DataTypes.DECIMAL(10, 2),
    allowNull: false,
    validate: {
      isDecimal: { msg: 'El precio sugerido debe ser un número decimal' },
      min: { args: [0.01], msg: 'El precio sugerido debe ser mayor a 0' }
    }
  },
  imagen_url: {
    type: DataTypes.TEXT,
    allowNull: true
  },
  fecha_creacion: {
    type: DataTypes.DATE,
    defaultValue: DataTypes.NOW
  }
}, {
  tableName: 'productos',
  timestamps: false,
  hooks: {
    beforeValidate: (producto) => {
      // Convertir a mayúsculas si es necesario
      if (producto.tipo_producto) {
        producto.tipo_producto = producto.tipo_producto.toUpperCase();
      }
    }
  }
});

// Métodos estáticos
Producto.findByTipo = function(tipo) {
  return this.findAll({ where: { tipo_producto: tipo } });
};

Producto.findByNombre = function(nombre) {
  return this.findAll({ 
    where: { 
      nombre: { [Op.like]: `%${nombre}%` } 
    } 
  });
};

module.exports = Producto;