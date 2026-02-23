import { useState } from "react";
import { Eye, EyeOff, IdCard } from "lucide-react";
import { useNavigate } from "react-router-dom";
import gymHero from "../../assets/imgLogin/imgLogin.png";
import "./login.css";

const Login = () => {
  const [documentNumber, setDocumentNumber] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);

  const navigate = useNavigate();

  const handleSubmit = (e) => {
    e.preventDefault();
    console.log("Login attempt with:", { documentNumber, password });
    navigate("/dashboard");
  };

  const togglePasswordVisibility = () => {
    setShowPassword(!showPassword);
  };

  return (
    <div className="login-container">
      {/* LADO IZQUIERDO - IMAGEN */}
      <div className="login-image-side">
        <img src={gymHero} alt="Gym" className="login-image" />
        <div className="login-image-overlay">
          <h2 className="login-image-title">SYSTEM MUSCLE</h2>
          <p className="login-image-subtitle">
            Sistema de Gestión Comercial e Inventario
          </p>
        </div>
      </div>

      {/* LADO DERECHO - FORMULARIO */}
      <div className="login-form-side">
        <div className="login-form-content">
          <h1 className="login-title">INICIAR SESIÓN</h1>
          <p className="login-subtitle">
            Ingresa tus credenciales para acceder al sistema
          </p>

          <form onSubmit={handleSubmit} className="login-form">
            {/* Número de Documento */}
            <div className="login-field">
              <label className="login-label">Número de Documento</label>
              <div className="login-input-wrapper">
                <IdCard size={18} className="login-input-icon" />
                <input
                  type="text"
                  value={documentNumber}
                  onChange={(e) => setDocumentNumber(e.target.value)}
                  placeholder="Ingresa tu número de documento"
                  className="login-input"
                  required
                />
              </div>
            </div>

            {/* Contraseña */}
            <div className="login-field">
              <label className="login-label">Contraseña</label>
              <div className="login-input-wrapper">
                <input
                  type={showPassword ? "text" : "password"}
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="Ingresa tu contraseña"
                  className="login-input login-input-password"
                  required
                />
                <button
                  type="button"
                  onClick={togglePasswordVisibility}
                  className="login-password-toggle"
                >
                  {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
                </button>
              </div>
            </div>

            {/* Botón Ingresar */}
            <button type="submit" className="login-button">
              Ingresar
            </button>
          </form>
        </div>
      </div>
    </div>
  );
};

export default Login;