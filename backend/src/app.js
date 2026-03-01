// backend/src/app.js
const express = require('express');
const cors = require('cors');
const helmet = require('helmet');

const app = express();

// Middlewares
app.use(helmet());
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.get('/', (req, res) => {
  res.json({ message: 'API Working' });
});

// Space to import routes later

module.exports = app; // <-- ESTO ES CRUCIAL