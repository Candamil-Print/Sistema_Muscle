import { useState } from "react";
import Sidebar from "../../components/sidebar/sidebar";
import { Search, Plus } from "lucide-react";
import "./movimientos.css";

const movimientosData = [
  {
    id: 1,
    producto: "Whey Protein Gold",
    cantidad: 4,
    registradoPor: "Administrador System Muscle",
    fechaHora: "17/02/2026, 01:56 p. m.",
    estado: "Disponible",
  },
  {
    id: 2,
    producto: "Barra de Proteína",
    cantidad: 2,
    registradoPor: "Administrador System Muscle",
    fechaHora: "17/02/2026, 01:56 p. m.",
    estado: "Disponible",
  },
];

const MovimientosEntrada = () => {
  const [searchQuery, setSearchQuery] = useState("");

  const filteredMovimientos = movimientosData.filter(mov => 
    mov.producto.toLowerCase().includes(searchQuery.toLowerCase()) ||
    mov.registradoPor.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="movimientos-container">
      <Sidebar />
      
      <main className="movimientos-main">
        {/* Título de la sección */}
        <h2 className="section-title">Movimientos de Entrada</h2>
        <p className="section-subtitle">Registro de entradas de productos al inventario</p>

        {/* Buscador y botón en la misma línea */}
        <div className="search-section">
          <div className="search-box">
            <Search size={18} className="search-icon" />
            <input
              type="text"
              placeholder="Buscar por producto o registrado por..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
            />
          </div>
          
          <button className="new-movimiento-btn">
            <Plus size={18} />
            Nuevo Movimiento
          </button>
        </div>

        {/* Tabla */}
        <div className="table-container">
          <table>
            <thead>
              <tr>
                <th>Producto</th>
                <th>Cantidad</th>
                <th>Registrado por</th>
                <th>Fecha y Hora</th>
                <th>Estado</th>
              </tr>
            </thead>
            <tbody>
              {filteredMovimientos.map((mov) => (
                <tr key={mov.id}>
                  <td>{mov.producto}</td>
                  <td className="cantidad">+{mov.cantidad}</td>
                  <td>{mov.registradoPor}</td>
                  <td>{mov.fechaHora}</td>
                  <td>
                    <span className="estado">{mov.estado}</span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </main>
    </div>
  );
};

export default MovimientosEntrada;