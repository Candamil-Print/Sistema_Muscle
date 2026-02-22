require('dotenv').config();
const app = require('./src/app');
const pool = require('./config/db');

async function connectDB() {
  try {
    const client = await pool.connect();
    await client.query('SELECT NOW()');
    console.log('Successfully connected with NEON');
    client.release();
  } catch (error) {
    console.error('Error connecting with NEON', error.message);
    process.exit(1);
  }
}

const PORT = process.env.PORT || 3000;

app.listen(PORT, async () => {
  console.log(`Server running on port ${PORT}`);
  await connectDB();
});
