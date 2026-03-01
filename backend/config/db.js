const { Sequelize } = require('sequelize');
const env = require('./environment'); 

const sequelize = new Sequelize(
  env.DB_NAME,      
  env.DB_USER,      
  env.DB_PASSWORD,  
  {
    host: env.DB_HOST,
    port: env.DB_PORT,
    dialect: env.DB_DIALECT || 'postgres',
    logging: env.NODE_ENV === 'development' ? console.log : false,
    pool: {
      max: 5,
      min: 0,
      acquire: 30000,
      idle: 10000
    }
  }
);

//Database connection test
const testConnection = async () => {
  try {
    await sequelize.authenticate();
    console.log('Successful connection to PostgreSQL');
  } catch (error) {
    console.error('Error connecting to PostgreSQL:', error);
  }
};

module.exports = {
  sequelize,
  testConnection
};