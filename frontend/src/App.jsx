import { BrowserRouter, Routes, Route } from "react-router-dom";
import Dashboard from "./pages/dashboard/dashboard.jsx";
import Login from "./pages/login/login.jsx";
import Productos from "./pages/productos/productos.jsx";
import Ventas from "./pages/PuntoVenta/PuntoVenta.jsx";
import Movimientos from "./pages/Movimientos/movimientos.jsx";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Login/>} />
        <Route path="/dashboard" element={<Dashboard />} />
        <Route path="/productos" element={<Productos />} />
        <Route path="/Ventas" element={<Ventas />} />
        <Route path="/movimientos" element={<Movimientos />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;