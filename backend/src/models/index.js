// backend/src/models/index.js
const { sequelize } = require('../../config/db');
const Usuario = require('./Usuario');
const Rol = require('./Rol');
const Producto = require('./Producto');
const TipoTurno = require('./TipoTurno');
const Turno = require('./Turno');
const Caja = require('./caja'); // <-- NUEVO

// Relaciones existentes
Usuario.belongsTo(Rol, { foreignKey: 'id_rol', as: 'rol' });
Rol.hasMany(Usuario, { foreignKey: 'id_rol', as: 'usuarios' });

// Relaciones de Turnos
Turno.belongsTo(Usuario, { foreignKey: 'id_usuario', as: 'usuario' });
Usuario.hasMany(Turno, { foreignKey: 'id_usuario', as: 'turnos' });
Turno.belongsTo(TipoTurno, { foreignKey: 'id_tipo_turno', as: 'tipo_turno' });
TipoTurno.hasMany(Turno, { foreignKey: 'id_tipo_turno', as: 'turnos' });

// Relaciones de Caja <-- NUEVO
Caja.belongsTo(Usuario, { foreignKey: 'id_usuario_apertura', as: 'usuario_apertura' });
Caja.belongsTo(Usuario, { foreignKey: 'id_usuario_cierre', as: 'usuario_cierre' });
Caja.belongsTo(Turno, { foreignKey: 'id_turno', as: 'turno' });

Turno.hasOne(Caja, { foreignKey: 'id_turno', as: 'caja' });

const syncDatabase = async () => {
    try {
        console.log('✅ Modelos cargados correctamente');
    } catch (error) {
        console.error('❌ Error:', error);
    }
};

module.exports = {
    sequelize,
    Usuario,
    Rol,
    Producto,
    TipoTurno,
    Turno,
    Caja, // <-- NUEVO
    syncDatabase
};