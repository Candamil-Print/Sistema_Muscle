import Sidebar from "../../components/sidebar/sidebar";
import "./dashboard.css";

const Dashboard = () => {
  return (
    <div className="dashboard-container">
      <Sidebar />
      
      <main className="dashboard-main">
        {/* Header */}
        <header className="dashboard-header">
          <h1 className="dashboard-title">Bienvenido, Administrador</h1>
          <p className="dashboard-subtitle">Resumen general del sistema</p>
        </header>

        {/* Stats Cards */}
        <div className="stats-container">
          {/* Productos Activos */}
          <div className="stat-card">
            <h2 className="stat-card-title">Productos Activos</h2>
            <p className="stat-card-number">3</p>
            <p className="stat-card-subtitle">3 total registrados</p>
          </div>

          {/* Ventas Hoy */}
          <div className="stat-card">
            <h2 className="stat-card-title">Ventas Hoy</h2>
            <p className="stat-card-number">0</p>
            <p className="stat-card-subtitle">$ 0</p>
          </div>

          {/* Stock Bajo */}
          <div className="stat-card">
            <h2 className="stat-card-title">Stock Bajo</h2>
            <p className="stat-card-number">0</p>
            <p className="stat-card-subtitle">Productos por reabastecer</p>
          </div>

          {/* Usuarios Activos */}
          <div className="stat-card">
            <h2 className="stat-card-title">Usuarios Activos</h2>
            <p className="stat-card-number">1</p>
            <p className="stat-card-subtitle">En el sistema</p>
          </div>
        </div>

        {/* Separator */}
        <hr className="dashboard-divider" />

        {/* Recent Activity */}
        <div className="activity-container">
          {/* Ventas Recientes */}
          <div className="activity-card">
            <h3 className="activity-card-title">Ventas Recientes</h3>
            <p className="activity-card-empty">No hay ventas registradas</p>
          </div>

          {/* Entradas Recientes */}
          <div className="activity-card">
            <h3 className="activity-card-title">Entradas Recientes</h3>
            <p className="activity-card-empty">No hay movimientos registrados</p>
          </div>
        </div>
      </main>
    </div>
  );
};

export default Dashboard;