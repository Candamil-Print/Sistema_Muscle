import { useState, useEffect } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import {
  LayoutDashboard,
  Package,
  Boxes,
  ArrowLeftRight,
  ShoppingCart,
  BarChart3,
  History,
  Users,
  Bell,
  LogOut,
} from "lucide-react";
import "./sidebar.css";

const Sidebar = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const [active, setActive] = useState("Dashboard");

  const menu = [
    { name: "Dashboard", icon: LayoutDashboard, path: "/dashboard" },
    { name: "Productos", icon: Package, path: "/productos" },
    { name: "Stock", icon: Boxes, path: "/stock" },
    { name: "Movimientos", icon: ArrowLeftRight, path: "/Movimientos" },
    { name: "Ventas", icon: ShoppingCart, path: "/Ventas" },
    { name: "Reportes", icon: BarChart3, path: "/reportes" },
    { name: "Historial", icon: History, path: "/historial" },
    { name: "Usuarios", icon: Users, path: "/usuarios" },
    { name: "Notificaciones", icon: Bell, path: "/notificaciones" },
  ];

  // Sincronizar el estado active con la ruta actual
  useEffect(() => {
    const currentPath = location.pathname;
    const activeItem = menu.find(item => item.path === currentPath);
    if (activeItem) {
      setActive(activeItem.name);
    }
  }, [location.pathname]); // Se ejecuta cada vez que cambia la ruta

  const handleNavigation = (item) => {
    setActive(item.name);
    navigate(item.path);
  };

  const handleLogout = () => {
    navigate("/");
  };

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <h1 className="sidebar-title">SYSTEM MUSCLE</h1>
      </div>

      <nav className="sidebar-nav">
        {menu.map((item) => {
          const Icon = item.icon;
          return (
            <button
              key={item.name}
              onClick={() => handleNavigation(item)}
              className={`sidebar-nav-item ${active === item.name ? 'active' : ''}`}
            >
              <Icon size={18} className="sidebar-nav-icon" />
              <span>{item.name}</span>
            </button>
          );
        })}
      </nav>

      <div className="sidebar-footer">
        <button onClick={handleLogout} className="sidebar-logout">
          <LogOut size={18} />
          <span>Cerrar Sesión</span>
        </button>
      </div>
    </aside>
  );
};

export default Sidebar;