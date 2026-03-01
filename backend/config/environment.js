require('dotenv').config();

module.exports = {
  PORT: process.env.PORT || 3000,
  NODE_ENV: process.env.NODE_ENV || 'development',
  API_PREFIX: process.env.API_PREFIX || '/api/v1',
  
  // Security for connection data
  DB_NAME: process.env.DB_NAME,
  DB_USER: process.env.DB_USER,
  DB_PASSWORD: process.env.DB_PASSWORD,
  DB_HOST: process.env.DB_HOST || 'localhost',
  DB_PORT: process.env.DB_PORT || 5432,
  DB_DIALECT: 'postgres',
  
  // // JWT
  // JWT_SECRET: process.env.JWT_SECRET,
  // JWT_EXPIRES_IN: process.env.JWT_EXPIRES_IN || '7d'
};