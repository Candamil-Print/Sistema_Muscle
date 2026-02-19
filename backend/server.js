require('dotenv').config();
const express = require('express');
const pool = require('./config/db');

const app = express();
app.use(express.json());

app.get('/test-db', async (req, res) => {
  try {
    const result = await pool.query('SELECT NOW()');
    res.json({
      success: true,
      time: result.rows[0],
    });
  } catch (error) {
    console.error(error);
    res.status(500).json({ success: false });
  }
});


async function connectDB () {
    try {
        const client = await pool.connect();
        const result = await client.query('SELECT NOW()');
        console.log('Successful Conction');
        console.log('The server time is:', result.rows[0].now);
        client.release();
    } catch (error) {
        console.error('Error connecting to the database:', error.message);
        process.exits(1);
    } 
}

const PORT = process.env.PORT || 3000;

app.listen(PORT, async () => {
  console.log(`Servidor corriendo en puerto ${PORT}`);
  await connectDB();
});
