// backend/src/routes/index.js
const router = require('express').Router();
const usuarioRoutes = require('./usuario.routes');

// Rutas de usuarios
router.use('/usuarios', usuarioRoutes);

// Ruta de salud
router.get('/health', (req, res) => {
  res.json({ 
    success: true, 
    message: 'API funcionando correctamente',
    timestamp: new Date()
  });
});

module.exports = router;