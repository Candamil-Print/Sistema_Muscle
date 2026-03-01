// backend/src/models/index.js
const { sequelize } = require('../../config/db');

// Aquí importarás tus modelos cuando los crees
// const User = require('./User');

const syncDatabase = async () => {
  try {
    console.log('Synchronized database');
  } catch (error) {
    console.error('Error synchronized database:', error);
  }
};

module.exports = {
  sequelize,
  syncDatabase
  // Space to export the modules
};