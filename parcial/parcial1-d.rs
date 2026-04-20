use std::collections::{HashMap, VecDeque};

// PARTE 1: Estructuras y Enums
#[derive(Debug, Clone, PartialEq)]
enum Categoria {
    Electronica,
    Ropa,
    Alimentos,
}

#[derive(Debug, Clone)]
struct Producto {
    id: u32,
    nombre: String,
    categoria: Categoria,
    precio_actual: f64,
    historial_precios: Vec<f64>, // Pila
}

impl Producto {
    fn nuevo(id: u32, nombre: String, categoria: Categoria, precio: f64) -> Self {
        Self {
            id,
            nombre,
            categoria,
            precio_actual: precio,
            historial_precios: Vec::new(),
        }
    }

    fn actualizar_precio(&mut self, nuevo_precio: f64) {
        // Lógica de Pila: Guardar el precio anterior
        self.historial_precios.push(self.precio_actual);

        // Limitar a los últimos 3 registros
        if self.historial_precios.len() > 3 {
            self.historial_precios.remove(0);
        }
        self.precio_actual = nuevo_precio;
    }

    fn obtener_ultimo_historial(&self) -> Result<f64, String> {
        self.historial_precios
            .last()
            .cloned()
            .ok_or_else(|| "Error: No hay precios anteriores en el historial".to_string())
    }
}

// PARTE 3: Orden de Envío
#[derive(Debug)]
struct OrdenEnvio {
    id_pedido: u32,
    id_producto: u32,
    cliente: String,
}

// PARTE 2 & 3: Almacén
struct Almacen {
    inventario: HashMap<u32, Producto>,
    cola_pedidos: VecDeque<OrdenEnvio>,
}

impl Almacen {
    fn nuevo() -> Self {
        Self {
            inventario: HashMap::new(),
            cola_pedidos: VecDeque::new(),
        }
    }

    fn agregar_producto(&mut self, p: Producto) -> Result<(), String> {
        if self.inventario.contains_key(&p.id) {
            return Err(format!("Error: El producto con ID {} ya existe", p.id));
        }
        self.inventario.insert(p.id, p);
        Ok(())
    }

    fn registrar_pedido(&mut self, orden: OrdenEnvio) {
        self.cola_pedidos.push_back(orden);
    }

    fn despachar_siguiente(&mut self) -> Result<OrdenEnvio, String> {
        self.cola_pedidos
            .pop_front()
            .ok_or_else(|| "Error: No hay pedidos pendientes en la cola".to_string())
    }

    // PARTE 4: Preparación para Búsqueda Binaria
    fn obtener_inventario_ordenado(&self) -> Vec<Producto> {
        let mut productos: Vec<Producto> = self.inventario.values().cloned().collect();
        // Ordenar por ID para permitir búsqueda binaria
        productos.sort_by_key(|p| p.id);
        productos
    }
}

// PARTE 4: Algoritmo de Búsqueda Binaria Manual
fn buscar_binaria(catalogo: &[Producto], id_objetivo: u32) -> Option<&Producto> {
    if catalogo.is_empty() {
        return None;
    }

    let mut bajo = 0;
    let mut alto = catalogo.len() - 1;

    while bajo <= alto {
        let medio = bajo + (alto - bajo) / 2;
        let producto_medio = &catalogo[medio];

        if producto_medio.id == id_objetivo {
            return Some(producto_medio);
        } else if producto_medio.id < id_objetivo {
            bajo = medio + 1;
        } else {
            if medio == 0 {
                break;
            } // Evitar overflow en usize
            alto = medio - 1;
        }
    }
    None
}

// PARTE 5: Integración
fn main() {
    let mut almacen = Almacen::nuevo();

    // 1. Registro de productos
    let productos_iniciales = vec![
        Producto::nuevo(105, "Laptop".into(), Categoria::Electronica, 1200.0),
        Producto::nuevo(101, "Camisa".into(), Categoria::Ropa, 25.0),
        Producto::nuevo(110, "Manzanas".into(), Categoria::Alimentos, 5.0),
        Producto::nuevo(103, "Monitor".into(), Categoria::Electronica, 300.0),
        Producto::nuevo(108, "Pantalón".into(), Categoria::Ropa, 40.0),
    ];

    for p in productos_iniciales {
        if let Err(e) = almacen.agregar_producto(p) {
            println!("{}", e);
        }
    }

    // 2. Probar Historial (Pila)
    if let Some(p) = almacen.inventario.get_mut(&105) {
        p.actualizar_precio(1250.0);
        p.actualizar_precio(1300.0);
        p.actualizar_precio(1280.0);
        p.actualizar_precio(1270.0); // El historial debería tener [1250, 1300, 1280]
        println!("Precio actual Laptop: ${}", p.precio_actual);
        match p.obtener_ultimo_historial() {
            Ok(prev) => println!("Último precio en historial: ${}", prev),
            Err(e) => println!("{}", e),
        }
    }

    // 3. Cola de Pedidos
    almacen.registrar_pedido(OrdenEnvio {
        id_pedido: 1,
        id_producto: 105,
        cliente: "Juan".into(),
    });
    almacen.registrar_pedido(OrdenEnvio {
        id_pedido: 2,
        id_producto: 101,
        cliente: "Maria".into(),
    });

    match almacen.despachar_siguiente() {
        Ok(orden) => println!(
            "Despachando pedido #{} para {}",
            orden.id_pedido, orden.cliente
        ),
        Err(e) => println!("{}", e),
    }

    // 4. Búsqueda Binaria
    let catalogo_ordenado = almacen.obtener_inventario_ordenado();
    println!("\n--- Realizando Auditoría (Búsqueda Binaria) ---");
    let ids_a_buscar = [103, 999];

    for id in ids_a_buscar {
        match buscar_binaria(&catalogo_ordenado, id) {
            Some(p) => println!("ID {}: Encontrado -> {}", id, p.nombre),
            None => println!("ID {}: No existe en el inventario", id),
        }
    }

    // 5. Valor Total
    let total: f64 = almacen.inventario.values().map(|p| p.precio_actual).sum();
    println!("\nVALOR TOTAL DEL INVENTARIO: ${:.2}", total);
}
