// backend/src/models/index.js
const { sequelize } = require('../../config/db');
const Usuario = require('./Usuario');
const Rol = require('./Rol');

// Definir relaciones
Usuario.belongsTo(Rol, { 
    foreignKey: 'id_rol', 
    as: 'rol' 
});

Rol.hasMany(Usuario, { 
    foreignKey: 'id_rol', 
    as: 'usuarios' 
});

const syncDatabase = async () => {
    try {
        console.log('✅ Modelos cargados correctamente');
        // NO sincronizar automáticamente - comentado para evitar errores
        // await sequelize.sync({ alter: true });
    } catch (error) {
        console.error('❌ Error:', error);
    }
};

module.exports = {
    sequelize,
    Usuario,
    Rol,
    syncDatabase
};