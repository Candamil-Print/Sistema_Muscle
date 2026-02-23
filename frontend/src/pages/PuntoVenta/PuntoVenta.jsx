import { useState } from "react";
import Sidebar from "../../components/sidebar/sidebar";
import { Search, Minus, Plus, ShoppingCart, X } from "lucide-react";
import "./PuntoVenta.css";

const productsData = [
  {
    id: 1,
    name: "Barra de Proteína",
    stock: 20,
    price: 5000,
  },
  {
    id: 2,
    name: "Whey Protein Gold",
    stock: 12,
    price: 120000,
  },
  {
    id: 3,
    name: "Agua Mineral 600ml",
    stock: 40,
    price: 2000,
  },
];

const PuntoVenta = () => {
  const [searchQuery, setSearchQuery] = useState("");
  const [cart, setCart] = useState([]);

  const addToCart = (product) => {
    setCart(prevCart => {
      const existingItem = prevCart.find(item => item.id === product.id);
      
      if (existingItem) {
        if (existingItem.quantity < product.stock) {
          return prevCart.map(item =>
            item.id === product.id
              ? { ...item, quantity: item.quantity + 1 }
              : item
          );
        }
        return prevCart;
      } else {
        return [...prevCart, { ...product, quantity: 1 }];
      }
    });
  };

  const removeFromCart = (productId) => {
    setCart(cart.filter(item => item.id !== productId));
  };

  const updateQuantity = (productId, newQuantity) => {
    if (newQuantity <= 0) {
      removeFromCart(productId);
    } else {
      setCart(cart.map(item =>
        item.id === productId
          ? { ...item, quantity: newQuantity }
          : item
      ));
    }
  };

  const filteredProducts = productsData.filter(product =>
    product.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const totalVenta = cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);

  return (
    <div className="puntoventa-container">
      <Sidebar />
      
      <main className="puntoventa-main">
        {/* Header */}
        <div className="puntoventa-header">
          <h1 className="puntoventa-title">Punto de Venta</h1>
          <p className="puntoventa-subtitle">Registro ventas de productos.</p>
        </div>

        {/* Buscador */}
        <div className="puntoventa-search">
          <Search size={18} className="search-icon" />
          <input
            type="text"
            placeholder="Buscar producto..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>

        {/* Grid de productos */}
        <div className="puntoventa-products-grid">
          {filteredProducts.map((product) => {
            const cartItem = cart.find(item => item.id === product.id);
            const currentQuantity = cartItem?.quantity || 0;
            
            return (
              <div 
                key={product.id} 
                className={`product-card ${currentQuantity > 0 ? 'in-cart' : ''}`}
                onClick={() => addToCart(product)}
              >
                <h3 className="product-name">{product.name}</h3>
                
                <div className="product-details">
                  <p className="product-stock">
                    <span>Stock:</span> {product.stock}
                  </p>
                  <p className="product-price">
                    <span>Price:</span> ${product.price.toLocaleString()}
                  </p>
                </div>

                {currentQuantity > 0 && (
                  <div className="product-cart-indicator">
                    <span>{currentQuantity} en carrito</span>
                  </div>
                )}
              </div>
            );
          })}
        </div>

        {/* Carrito de compras */}
        {cart.length > 0 && (
          <div className="puntoventa-cart">
            <div className="cart-header">
              <ShoppingCart size={20} />
              <h2>Carrito de Ventas</h2>
            </div>

            <div className="cart-items">
              {cart.map((item) => (
                <div key={item.id} className="cart-item">
                  <div className="cart-item-info">
                    <h4>{item.name}</h4>
                    <p>${item.price.toLocaleString()} x {item.quantity}</p>
                  </div>
                  
                  <div className="cart-item-actions">
                    <div className="cart-quantity">
                      <button 
                        onClick={(e) => {
                          e.stopPropagation();
                          updateQuantity(item.id, item.quantity - 1);
                        }}
                        className="cart-quantity-btn"
                      ><span>-</span>
                        <Minus size={14} />
                      </button>
                      <span>{item.quantity}</span>
                      <button 
                        onClick={(e) => {
                          e.stopPropagation();
                          updateQuantity(item.id, item.quantity + 1);
                        }}
                        className="cart-quantity-btn"
                        
                      > <span>+</span>
                        <Plus size={14} />
                      </button>
                    </div>
                    <button 
                      onClick={(e) => {
                        e.stopPropagation();
                        removeFromCart(item.id);
                      }}
                      className="cart-remove-btn"
                    ><span>X</span>
                      <X size={16} />
                    </button>
                  </div>
                </div>
              ))}
            </div>

            <div className="cart-total">
              <span>Total:</span>
              <span>${totalVenta.toLocaleString()}</span>
            </div>

            <button className="checkout-btn">
              Finalizar Venta
            </button>
          </div>
        )}
      </main>
    </div>
  );
};

export default PuntoVenta;