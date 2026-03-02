// backend/src/routes/index.js
const router = require('express').Router();
const usuarioRoutes = require('./usuario.routes');
const productoRoutes = require('./producto.routes');
const turnoRoutes = require('./turno.routes');
const cajaRoutes = require('./caja.routes'); // <-- NUEVO

router.use('/usuarios', usuarioRoutes);
router.use('/productos', productoRoutes);
router.use('/turnos', turnoRoutes);
router.use('/cajas', cajaRoutes); // <-- NUEVO

router.get('/health', (req, res) => {
  res.json({ 
    success: true, 
    message: 'API funcionando correctamente',
    timestamp: new Date()
  });
});

module.exports = router;