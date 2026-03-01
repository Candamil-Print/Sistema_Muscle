const app = require('./src/app');
const env = require('./config/environment');
const { testConnection } = require('./config/db');
const { syncDatabase } = require('./src/models');

const startServer = async () => {
  try {
    await testConnection();
    
    // Synchronized models
    if (env.NODE_ENV === 'development') {
      await syncDatabase();
    }
    
    // Start Server
    app.listen(env.PORT, () => {
      console.log(`
      Server running in port: ${env.PORT}
      In mode: ${env.NODE_ENV}
      API: ${env.API_PREFIX}
      `);
    });
  } catch (error) {
    console.error('Error starting server:', error);
    process.exit(1);
  }
};

startServer();