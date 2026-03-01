// backend/src/models/Usuario.js
const { DataTypes } = require('sequelize');
const { sequelize } = require('../../config/db');
const bcrypt = require('bcryptjs');

const Usuario = sequelize.define('Usuario', {
  id_usuario: {
    type: DataTypes.INTEGER,
    primaryKey: true,
    autoIncrement: true
  },
  nombre_completo: {
    type: DataTypes.STRING(150),
    allowNull: false,
    validate: {
      notEmpty: { msg: 'Full name is required' }
    }
  },
  tipo_documento: {
    type: DataTypes.ENUM('CC', 'TI', 'CE', 'PASAPORTE'),
    allowNull: false
  },
  numero_documento: {
    type: DataTypes.STRING(50),
    allowNull: false,
    unique: true,
    validate: {
      notEmpty: { msg: 'The document number ir required' }
    }
  },
  direccion: {
    type: DataTypes.STRING(200),
    allowNull: true
  },
  tipo_sangre: {
    type: DataTypes.ENUM('O+', 'O-', 'A+', 'A-', 'B+', 'B-', 'AB+', 'AB-'),
    allowNull: true
  },
  eps: {
    type: DataTypes.ENUM('SURA', 'SANITAS', 'COMPENSAR', 'NUEVA_EPS', 'FAMISANAR', 'COOMEVA', 'SALUD_TOTAL'),
    allowNull: true
  },
  genero: {
    type: DataTypes.ENUM('MASCULINO', 'FEMENINO', 'OTRO', 'PREFIERE_NO_DECIR'),
    allowNull: true
  },
  correo: {
    type: DataTypes.STRING(150),
    allowNull: true,
    validate: {
      isEmail: { msg: 'It must be a valid email address' }
    }
  },
  telefono: {
    type: DataTypes.STRING(50),
    allowNull: true
  },
  password_hash: {
    type: DataTypes.STRING(255),
    allowNull: false
  },
  estado: {
    type: DataTypes.BOOLEAN,
    defaultValue: true
  },
  fecha_creacion: {
    type: DataTypes.DATE,
    defaultValue: DataTypes.NOW
  },
  id_rol: {
    type: DataTypes.INTEGER,
    allowNull: false,
    references: {
      model: 'roles',
      key: 'id_rol'
    }
  }
}, {
  tableName: 'usuarios',
  timestamps: false, // Porque usamos fecha_creacion manual
  hooks: {
    beforeCreate: async (usuario) => {
      if (usuario.password_hash) {
        usuario.password_hash = await bcrypt.hash(usuario.password_hash, 10);
      }
    },
    beforeUpdate: async (usuario) => {
      if (usuario.changed('password_hash')) {
        usuario.password_hash = await bcrypt.hash(usuario.password_hash, 10);
      }
    }
  }
});

// Métodos de instancia
Usuario.prototype.validarPassword = async function(password) {
  return await bcrypt.compare(password, this.password_hash);
};

// Métodos estáticos
Usuario.findByDocumento = function(numero_documento) {
  return this.findOne({ where: { numero_documento } });
};

Usuario.findByEmail = function(correo) {
  return this.findOne({ where: { correo } });
};

module.exports = Usuario;