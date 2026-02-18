const { Pool } = require('pg');

const pool = new Pool({
  connectionString: 'postgresql://neondb_owner:npg_4AtMKB7hjWYg@ep-rough-king-a847tf5l-pooler.eastus2.azure.neon.tech/neondb?sslmode=require&channel_binding=require',
  ssl: {
    rejectUnauthorized: false, // necesario en muchos servicios cloud
  },
});

async function testConnection() {
  try {
    const res = await pool.query('SELECT NOW()');
    console.log('Conexión exitosa:', res.rows[0]);
  } catch (err) {
    console.error('Error de conexión:', err);
  }
}

testConnection();
