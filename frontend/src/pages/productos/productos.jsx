import { useState } from "react";
import { useNavigate } from "react-router-dom";
import Sidebar from "../../components/sidebar/sidebar";
import {
  Plus,
  Search,
  Pencil,
  Trash2,
  Upload,
  X,
} from "lucide-react";
import "./productos.css";

const productsData = [
  {
    id: 1,
    name: "Barra de Proteina",
    type: "Snacks",
    costPrice: 3500,
    suggestedPrice: 5000,
    currentStock: 25,
    maxStock: 50,
    status: "Disponible",
  },
  {
    id: 2,
    name: "Whey Protein Gold",
    type: "Suplementos",
    costPrice: 85000,
    suggestedPrice: 120000,
    currentStock: 8,
    maxStock: 20,
    status: "Disponible",
  },
  {
    id: 3,
    name: "Agua Mineral 600ml",
    type: "Bebidas",
    costPrice: 1200,
    suggestedPrice: 2000,
    currentStock: 40,
    maxStock: 100,
    status: "Disponible",
  },
];

const Producto = () => {
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedType, setSelectedType] = useState("Todos");
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [formData, setFormData] = useState({
    nombre: "",
    tipo: "Snacks",
    stockMaximo: "",
    precioCosto: "",
    precioSugerido: "",
    imagen: null,
  });
  const [previewImage, setPreviewImage] = useState(null);

  const tipos = ["Snacks", "Suplementos", "Bebidas", "Proteínas", "Vitaminas"];

  const handleEdit = (id) => console.log("Editar:", id);
  const handleDelete = (id) => console.log("Eliminar:", id);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleImageChange = (e) => {
    const file = e.target.files[0];
    if (file) {
      setFormData((prev) => ({
        ...prev,
        imagen: file,
      }));
      setPreviewImage(URL.createObjectURL(file));
    }
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    console.log("Producto creado:", formData);
    setShowCreateForm(false);
    setFormData({
      nombre: "",
      tipo: "Snacks",
      stockMaximo: "",
      precioCosto: "",
      precioSugerido: "",
      imagen: null,
    });
    setPreviewImage(null);
  };

  const handleCancel = () => {
    setShowCreateForm(false);
    setFormData({
      nombre: "",
      tipo: "Snacks",
      stockMaximo: "",
      precioCosto: "",
      precioSugerido: "",
      imagen: null,
    });
    setPreviewImage(null);
  };

  const filteredProducts = productsData.filter((product) => {
    const matchName = product.name
      .toLowerCase()
      .includes(searchQuery.toLowerCase());
    const matchType =
      selectedType === "Todos" || product.type === selectedType;
    return matchName && matchType;
  });

  return (
    <div className="producto-container">
      <Sidebar />
      
      <main className="producto-main">
        {/* HEADER */}
        <div className="producto-header">
          <div>
            <h1 className="producto-title">Productos</h1>
            <p className="producto-subtitle">
              Gestiona el catálogo de productos
            </p>
          </div>

          <button 
            className="producto-new-btn"
            onClick={() => setShowCreateForm(true)}
          >
            <Plus size={18} />
            Nuevo Producto
          </button>
        </div>

        {/* FILTROS */}
        <div className="producto-filters">
          <div className="producto-search">
            <Search size={18} className="search-icon" />
            <input
              type="text"
              placeholder="Buscar por nombre..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
            />
          </div>

          <select
            className="producto-select"
            value={selectedType}
            onChange={(e) => setSelectedType(e.target.value)}
          >
            <option value="Todos">Todos</option>
            <option value="Snacks">Snacks</option>
            <option value="Suplementos">Suplementos</option>
            <option value="Bebidas">Bebidas</option>
          </select>
        </div>

        {/* FORMULARIO CREAR PRODUCTO */}
        {showCreateForm && (
          <div className="producto-form-overlay">
            <div className="producto-form-modal">
              <div className="form-modal-header">
                <h2>Crear Producto</h2>
                <button onClick={handleCancel} className="close-btn">
                  <X size={20} />
                </button>
              </div>

              <form onSubmit={handleSubmit} className="producto-create-form">
                {/* Nombre del producto */}
                <div className="form-group">
                  <label htmlFor="nombre">Nombre del producto</label>
                  <input
                    type="text"
                    id="nombre"
                    name="nombre"
                    value={formData.nombre}
                    onChange={handleChange}
                    placeholder="Ej: Barra de Proteína"
                    required
                  />
                </div>

                {/* Tipo */}
                <div className="form-group">
                  <label htmlFor="tipo">Tipo</label>
                  <select
                    id="tipo"
                    name="tipo"
                    value={formData.tipo}
                    onChange={handleChange}
                  >
                    {tipos.map((tipo) => (
                      <option key={tipo} value={tipo}>
                        {tipo}
                      </option>
                    ))}
                  </select>
                </div>

                {/* Stock Máximo */}
                <div className="form-group">
                  <label htmlFor="stockMaximo">Stock Máximo</label>
                  <input
                    type="number"
                    id="stockMaximo"
                    name="stockMaximo"
                    value={formData.stockMaximo}
                    onChange={handleChange}
                    placeholder="Cantidad máxima"
                    min="1"
                    required
                  />
                </div>

                {/* Precio Costo */}
                <div className="form-group">
                  <label htmlFor="precioCosto">Precio Costo (COP)</label>
                  <input
                    type="number"
                    id="precioCosto"
                    name="precioCosto"
                    value={formData.precioCosto}
                    onChange={handleChange}
                    placeholder="Ej: 3500"
                    min="0"
                    required
                  />
                </div>

                {/* Precio Sugerido */}
                <div className="form-group">
                  <label htmlFor="precioSugerido">Precio Sugerido (COP)</label>
                  <input
                    type="number"
                    id="precioSugerido"
                    name="precioSugerido"
                    value={formData.precioSugerido}
                    onChange={handleChange}
                    placeholder="Ej: 5000"
                    min="0"
                    required
                  />
                </div>

                {/* Fotografía */}
                <div className="form-group image-group">
                  <label>Fotografía del producto *</label>
                  
                  {!previewImage ? (
                    <div className="image-upload-area">
                      <input
                        type="file"
                        id="imagen"
                        accept="image/*"
                        onChange={handleImageChange}
                        className="image-input"
                      />
                      <label htmlFor="imagen" className="image-upload-label">
                        <Upload size={24} />
                        <span>Haz clic para seleccionar una imagen</span>
                      </label>
                    </div>
                  ) : (
                    <div className="image-preview">
                      <img src={previewImage} alt="Preview" />
                      <button
                        type="button"
                        onClick={() => {
                          setPreviewImage(null);
                          setFormData((prev) => ({ ...prev, imagen: null }));
                        }}
                        className="image-remove-btn"
                      >
                        <X size={16} />
                      </button>
                    </div>
                  )}
                </div>

                {/* Botones */}
                <div className="form-actions">
                  <button type="button" onClick={handleCancel} className="cancel-btn">
                    Cancelar
                  </button>
                  <button type="submit" className="submit-btn">
                    Crear Producto
                  </button>
                </div>
              </form>
            </div>
          </div>
        )}

        {/* TABLA */}
        <div className="producto-table-container">
          <table className="producto-table">
            <thead>
              <tr>
                <th>Nombre</th>
                <th>Tipo</th>
                <th>P. Costo</th>
                <th>P. Sugerido</th>
                <th>Stock</th>
                <th>Estado</th>
                <th>Acciones</th>
              </tr>
            </thead>

            <tbody>
              {filteredProducts.map((product) => {
                const percentage =
                  (product.currentStock / product.maxStock) * 100;

                return (
                  <tr key={product.id}>
                    <td>{product.name}</td>

                    <td>
                      <span className="producto-type-badge">
                        {product.type}
                      </span>
                    </td>

                    <td>${product.costPrice.toLocaleString()}</td>

                    <td>${product.suggestedPrice.toLocaleString()}</td>

                    <td>
                      <div className="producto-stock-bar">
                        <div
                          className="stock-bar-fill"
                          style={{ width: `${percentage}%` }}
                        />
                      </div>
                      <span className="producto-stock-text">
                        {product.currentStock}/{product.maxStock}
                      </span>
                    </td>

                    <td>
                      <span className="producto-status-badge">
                        {product.status}
                      </span>
                    </td>

                    <td>
                      <div className="producto-actions">
                        <button
                          onClick={() => handleEdit(product.id)}
                          className="action-btn edit"
                        >
                          <Pencil size={16} />
                        </button>

                        <button
                          onClick={() => handleDelete(product.id)}
                          className="action-btn delete"
                        >
                          <Trash2 size={16} />
                        </button>
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </main>
    </div>
  );
};

export default Producto;