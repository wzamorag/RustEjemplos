// ============================================================================
// EXAMEN PARCIAL RUST - ALTERNATIVA 2: GESTOR DE INVENTARIO DE TIENDA
// Solución Completa - Todos los temas integrados
// ============================================================================

use std::collections::{HashMap, VecDeque};

// ============================================================================
// PARTE 1: ESTRUCTURA BASE Y PILA DE PRECIOS HISTÓRICOS
// ============================================================================

/// Enum para tipos de movimientos en el inventario
#[derive(Debug, Clone, Copy)]
enum TipoMovimiento {
    Entrada,
    Salida,
    Reajuste,
}

impl TipoMovimiento {
    fn descripcion(&self) -> &'static str {
        match self {
            TipoMovimiento::Entrada => "Entrada",
            TipoMovimiento::Salida => "Salida",
            TipoMovimiento::Reajuste => "Reajuste",
        }
    }
}

/// Estructura de un Producto con pila de precios históricos
#[derive(Debug, Clone)]
struct Producto {
    codigo: u32,
    nombre: String,
    cantidad_stock: u32,
    precio_actual: f64,
    // PILA: Vec para almacenar historial de precios (máximo 5)
    precios_historicos: Vec<f64>,
}

impl Producto {
    /// Constructor de un nuevo producto
    fn nuevo(codigo: u32, nombre: String, cantidad_stock: u32, precio_inicial: f64) -> Self {
        let mut producto = Producto {
            codigo,
            nombre,
            cantidad_stock,
            precio_actual: precio_inicial,
            precios_historicos: Vec::new(),
        };
        // El precio actual es parte del historial
        producto.precios_historicos.push(precio_inicial);
        producto
    }

    /// Agrega un nuevo precio a la pila de precios históricos (máximo 5)
    fn actualizar_precio(&mut self, nuevo_precio: f64) {
        self.precio_actual = nuevo_precio;
        self.precios_historicos.push(nuevo_precio);

        // Mantener solo los últimos 5 precios (FIFO para la pila)
        if self.precios_historicos.len() > 5 {
            self.precios_historicos.remove(0);
        }
    }

    /// Retorna el precio más reciente (tope de la pila)
    fn precio_actual(&self) -> f64 {
        self.precio_actual
    }

    /// Obtiene el historial completo de precios
    fn obtener_historial_precios(&self) -> Vec<f64> {
        self.precios_historicos.clone()
    }

    /// Calcula el valor total en inventario para este producto
    fn valor_total(&self) -> f64 {
        self.cantidad_stock as f64 * self.precio_actual
    }

    /// Actualiza la cantidad en stock
    fn actualizar_stock(&mut self, cantidad: i32) -> Result<(), String> {
        let nuevo_stock = (self.cantidad_stock as i32) + cantidad;
        if nuevo_stock < 0 {
            return Err(format!(
                "Error: Stock insuficiente para {}. Actual: {}, Solicitud: -{}",
                self.nombre,
                self.cantidad_stock,
                cantidad.abs()
            ));
        }
        self.cantidad_stock = nuevo_stock as u32;
        Ok(())
    }
}

// ============================================================================
// PARTE 2: TABLA HASH DE PRODUCTOS (INVENTARIO)
// ============================================================================

/// Estructura que gestiona el inventario completo usando HashMap
#[derive(Debug)]
struct Inventario {
    // TABLA HASH: código -> Producto
    productos: HashMap<u32, Producto>,
}

impl Inventario {
    /// Crea un nuevo inventario vacío
    fn nuevo() -> Self {
        Inventario {
            productos: HashMap::new(),
        }
    }

    /// Agrega un nuevo producto al inventario
    fn agregar_producto(&mut self, producto: Producto) -> Result<(), String> {
        if self.productos.contains_key(&producto.codigo) {
            return Err(format!(
                "Error: Ya existe un producto con código {}",
                producto.codigo
            ));
        }
        self.productos.insert(producto.codigo, producto);
        Ok(())
    }

    /// Actualiza el stock de un producto
    fn actualizar_stock(&mut self, codigo: u32, cantidad: i32) -> Result<(), String> {
        match self.productos.get_mut(&codigo) {
            Some(producto) => producto.actualizar_stock(cantidad),
            None => Err(format!("Error: No existe producto con código {}", codigo)),
        }
    }

    /// Búsqueda secuencial por nombre (puede retornar múltiples resultados)
    fn buscar_por_nombre(&self, nombre_busca: &str) -> Vec<&Producto> {
        let nombre_busca = nombre_busca.to_lowercase();
        self.productos
            .values()
            .filter(|p| p.nombre.to_lowercase().contains(&nombre_busca))
            .collect()
    }

    /// Calcula el valor total del inventario
    fn valor_total_inventario(&self) -> f64 {
        self.productos.values().map(|p| p.valor_total()).sum()
    }

    /// Obtiene productos ordenados por código
    fn obtener_productos_ordenados(&self) -> Vec<Producto> {
        let mut productos: Vec<Producto> = self.productos.values().cloned().collect();
        productos.sort_by_key(|p| p.codigo);
        productos
    }

    /// Búsqueda binaria de un producto por código
    fn buscar_por_codigo_binaria(&self, codigo_busca: u32) -> Option<Producto> {
        let productos = self.obtener_productos_ordenados();
        let mut izq = 0;
        let mut der = productos.len();

        while izq < der {
            let mid = izq + (der - izq) / 2;

            match productos[mid].codigo.cmp(&codigo_busca) {
                std::cmp::Ordering::Equal => return Some(productos[mid].clone()),
                std::cmp::Ordering::Less => izq = mid + 1,
                std::cmp::Ordering::Greater => der = mid,
            }
        }

        None
    }

    /// Retorna productos con código en un rango específico
    fn productos_en_rango(&self, codigo_min: u32, codigo_max: u32) -> Vec<Producto> {
        let productos = self.obtener_productos_ordenados();
        productos
            .into_iter()
            .filter(|p| p.codigo >= codigo_min && p.codigo <= codigo_max)
            .collect()
    }

    /// Obtiene el número total de productos
    fn cantidad_productos(&self) -> usize {
        self.productos.len()
    }

    /// Obtiene referencia a los productos
    fn obtener_productos(&self) -> &HashMap<u32, Producto> {
        &self.productos
    }
}

// ============================================================================
// PARTE 3: COLA DE MOVIMIENTOS
// ============================================================================

/// Estructura que representa un movimiento de inventario
#[derive(Debug, Clone)]
struct Movimiento {
    tipo: TipoMovimiento,
    codigo_producto: u32,
    cantidad: u32,
    timestamp: String,
}

impl Movimiento {
    /// Constructor de un nuevo movimiento
    fn nuevo(tipo: TipoMovimiento, codigo_producto: u32, cantidad: u32, timestamp: String) -> Self {
        Movimiento {
            tipo,
            codigo_producto,
            cantidad,
            timestamp,
        }
    }
}

/// Gestor de la cola de movimientos (FIFO)
#[derive(Debug)]
struct RegistroMovimientos {
    // COLA: VecDeque para comportamiento FIFO
    cola_movimientos: VecDeque<Movimiento>,
    historial_procesados: Vec<Movimiento>,
}

impl RegistroMovimientos {
    /// Crea un nuevo registro de movimientos vacío
    fn nuevo() -> Self {
        RegistroMovimientos {
            cola_movimientos: VecDeque::new(),
            historial_procesados: Vec::new(),
        }
    }

    /// Registra un movimiento en la cola
    fn registrar_movimiento(&mut self, movimiento: Movimiento) {
        self.cola_movimientos.push_back(movimiento);
    }

    /// Obtiene y procesa el siguiente movimiento de la cola
    fn obtener_siguiente(&mut self) -> Option<Movimiento> {
        if let Some(movimiento) = self.cola_movimientos.pop_front() {
            self.historial_procesados.push(movimiento.clone());
            Some(movimiento)
        } else {
            None
        }
    }

    /// Cantidad de movimientos pendientes en la cola
    fn cantidad_pendientes(&self) -> usize {
        self.cola_movimientos.len()
    }

    /// Lista los movimientos pendientes
    fn listar_pendientes(&self) {
        if self.cola_movimientos.is_empty() {
            println!("No hay movimientos pendientes.");
            return;
        }

        println!("\n=== MOVIMIENTOS PENDIENTES ===");
        for (idx, mov) in self.cola_movimientos.iter().enumerate() {
            println!(
                "{}. [{}] Código: {} - Cantidad: {} - {}",
                idx + 1,
                mov.tipo.descripcion(),
                mov.codigo_producto,
                mov.cantidad,
                mov.timestamp
            );
        }
    }

    /// Retorna el historial de movimientos procesados
    fn obtener_historial(&self) -> &Vec<Movimiento> {
        &self.historial_procesados
    }
}

// ============================================================================
// PARTE 5: REPORTES Y ESTADÍSTICAS
// ============================================================================

/// Estructura para generar reportes del inventario
struct Reporte;

impl Reporte {
    /// Reporta productos con bajo stock (< 10 unidades)
    fn productos_bajo_stock(inventario: &Inventario) -> Vec<&Producto> {
        inventario
            .obtener_productos()
            .values()
            .filter(|p| p.cantidad_stock < 10)
            .collect()
    }

    /// Retorna el producto más caro y el más barato
    fn productos_extremos(inventario: &Inventario) -> Option<(&Producto, &Producto)> {
        if inventario.cantidad_productos() == 0 {
            return None;
        }

        let mut max_precio: &Producto = &Producto::nuevo(0, "".to_string(), 0, 0.0);
        let mut min_precio: &Producto = &Producto::nuevo(0, "".to_string(), 0, f64::MAX);

        for producto in inventario.obtener_productos().values() {
            if producto.precio_actual > max_precio.precio_actual {
                max_precio = producto;
            }
            if producto.precio_actual < min_precio.precio_actual {
                min_precio = producto;
            }
        }

        Some((max_precio, min_precio))
    }

    /// Top 5 productos por valor en inventario
    fn top_5_productos_valor(inventario: &Inventario) -> Vec<&Producto> {
        let mut productos: Vec<&Producto> = inventario.obtener_productos().values().collect();
        productos.sort_by(|a, b| b.valor_total().partial_cmp(&a.valor_total()).unwrap());
        productos.into_iter().take(5).collect()
    }

    /// Mostra reportes de bajo stock
    fn mostrar_bajo_stock(inventario: &Inventario) {
        let bajo_stock = Self::productos_bajo_stock(inventario);

        if bajo_stock.is_empty() {
            println!("✓ Todos los productos tienen stock adecuado.");
            return;
        }

        println!("\n⚠️  PRODUCTOS CON BAJO STOCK (< 10 unidades):");
        for producto in bajo_stock {
            println!(
                "  - {} (Código: {}) - Stock: {}",
                producto.nombre, producto.codigo, producto.cantidad_stock
            );
        }
    }

    /// Muestra productos más caro y más barato
    fn mostrar_productos_extremos(inventario: &Inventario) {
        if let Some((max, min)) = Self::productos_extremos(inventario) {
            println!("\n💰 PRODUCTOS EXTREMOS:");
            println!("  Más caro: {} - ${:.2}", max.nombre, max.precio_actual);
            println!("  Más barato: {} - ${:.2}", min.nombre, min.precio_actual);
        }
    }

    /// Muestra top 5 productos por valor
    fn mostrar_top_5(inventario: &Inventario) {
        let top_5 = Self::top_5_productos_valor(inventario);

        if top_5.is_empty() {
            println!("No hay productos en el inventario.");
            return;
        }

        println!("\n🏆 TOP 5 PRODUCTOS POR VALOR EN INVENTARIO:");
        for (idx, producto) in top_5.iter().enumerate() {
            println!(
                "{}. {} - ${:.2} ({} units × ${:.2})",
                idx + 1,
                producto.nombre,
                producto.valor_total(),
                producto.cantidad_stock,
                producto.precio_actual
            );
        }
    }
}

// ============================================================================
// PARTE 6: PROGRAMA PRINCIPAL COMPLETO
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          GESTOR DE INVENTARIO DE TIENDA - EXAMEN            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    // =======================================================================
    // PASO 1: INICIALIZAR INVENTARIO Y CREAR PRODUCTOS
    // =======================================================================
    println!("\n📦 PASO 1: Creando inventario con 10 productos...");

    let mut inventario = Inventario::nuevo();
    let productos_iniciales = vec![
        ("Laptop", 100, 1, 1200.0),
        ("Mouse", 200, 50, 25.0),
        ("Teclado", 101, 35, 75.0),
        ("Monitor", 201, 15, 350.0),
        ("Audífonos", 102, 40, 80.0),
        ("Webcam", 202, 20, 120.0),
        ("Escritorio", 103, 8, 400.0),
        ("Silla", 203, 12, 300.0),
        ("Cable USB", 104, 100, 5.0),
        ("Hub USB", 204, 18, 45.0),
    ];

    for (nombre, codigo, cantidad, precio) in productos_iniciales {
        let producto = Producto::nuevo(codigo, nombre.to_string(), cantidad, precio);
        inventario.agregar_producto(producto).unwrap();
    }

    println!(
        "✓ Se agregaron {} productos al inventario",
        inventario.cantidad_productos()
    );

    // =======================================================================
    // PASO 2: REGISTRAR MOVIMIENTOS EN LA COLA
    // =======================================================================
    println!("\n📋 PASO 2: Registrando 8 movimientos en la cola...");

    let mut registro = RegistroMovimientos::nuevo();

    let movimientos_registrados = vec![
        (TipoMovimiento::Entrada, 100, 5, "2024-01-15 09:00"),
        (TipoMovimiento::Salida, 200, 10, "2024-01-15 10:30"),
        (TipoMovimiento::Salida, 101, 3, "2024-01-15 11:00"),
        (TipoMovimiento::Entrada, 201, 2, "2024-01-15 11:45"),
        (TipoMovimiento::Reajuste, 102, 5, "2024-01-15 12:00"),
        (TipoMovimiento::Salida, 202, 4, "2024-01-15 13:15"),
        (TipoMovimiento::Entrada, 203, 3, "2024-01-15 14:00"),
        (TipoMovimiento::Salida, 204, 7, "2024-01-15 15:30"),
    ];

    for (tipo, codigo, cantidad, timestamp) in movimientos_registrados {
        let movimiento = Movimiento::nuevo(tipo, codigo, cantidad, timestamp.to_string());
        registro.registrar_movimiento(movimiento);
    }

    println!("✓ Se registraron {} movimientos", 8);
    registro.listar_pendientes();

    // =======================================================================
    // PASO 3: PROCESAR MOVIMIENTOS (COLA FIFO)
    // =======================================================================
    println!("\n⚙️  PASO 3: Procesando 5 movimientos en orden FIFO...");

    for i in 1..=5 {
        if let Some(movimiento) = registro.obtener_siguiente() {
            match movimiento.tipo {
                TipoMovimiento::Entrada => {
                    let _ = inventario
                        .actualizar_stock(movimiento.codigo_producto, movimiento.cantidad as i32);
                    println!(
                        "✓ {} Producto {} +{} unidades",
                        i, movimiento.codigo_producto, movimiento.cantidad
                    );
                }
                TipoMovimiento::Salida => {
                    match inventario
                        .actualizar_stock(movimiento.codigo_producto, -(movimiento.cantidad as i32))
                    {
                        Ok(_) => println!(
                            "✓ {} Producto {} -{} unidades",
                            i, movimiento.codigo_producto, movimiento.cantidad
                        ),
                        Err(e) => println!("✗ {} {}", i, e),
                    }
                }
                TipoMovimiento::Reajuste => {
                    let _ = inventario
                        .actualizar_stock(movimiento.codigo_producto, movimiento.cantidad as i32);
                    println!(
                        "✓ {} Reajuste en producto {}",
                        i, movimiento.codigo_producto
                    );
                }
            }
        }
    }

    println!(
        "Movimientos aún pendientes: {}",
        registro.cantidad_pendientes()
    );

    // =======================================================================
    // PASO 4: BÚSQUEDA SECUENCIAL POR NOMBRE
    // =======================================================================
    println!("\n🔍 PASO 4: Búsqueda secuencial de productos por nombre...");

    let termino_busca = "Cable";
    let resultados = inventario.buscar_por_nombre(termino_busca);

    if resultados.is_empty() {
        println!("No se encontraron productos con '{}'", termino_busca);
    } else {
        println!(
            "Se encontraron {} producto(s) con '{}':",
            resultados.len(),
            termino_busca
        );
        for producto in resultados {
            println!(
                "  - {} (Código: {}) - Stock: {} - ${:.2}",
                producto.nombre, producto.codigo, producto.cantidad_stock, producto.precio_actual
            );
        }
    }

    // =======================================================================
    // PASO 5: BÚSQUEDA BINARIA POR CÓDIGO
    // =======================================================================
    println!("\n🔎 PASO 5: Búsqueda binaria por código...");

    let codigo_buscar = 201;
    match inventario.buscar_por_codigo_binaria(codigo_buscar) {
        Some(producto) => {
            println!("✓ Producto encontrado (Búsqueda Binaria):");
            println!("  Código: {}", producto.codigo);
            println!("  Nombre: {}", producto.nombre);
            println!("  Stock: {}", producto.cantidad_stock);
            println!("  Precio: ${:.2}", producto.precio_actual);
            println!("  Valor Total: ${:.2}", producto.valor_total());
        }
        None => println!("✗ No se encontró producto con código {}", codigo_buscar),
    }

    // =======================================================================
    // PASO 6: BÚSQUEDA DE RANGO
    // =======================================================================
    println!("\n📊 PASO 6: Productos con código en rango [100-200]...");

    let productos_rango = inventario.productos_en_rango(100, 200);
    println!("Se encontraron {} productos:", productos_rango.len());
    for producto in productos_rango {
        println!(
            "  - {} (Código: {}) - Stock: {}",
            producto.nombre, producto.codigo, producto.cantidad_stock
        );
    }

    // =======================================================================
    // PASO 7: ACTUALIZACIÓN DE PRECIOS (PILA)
    // =======================================================================
    println!("\n💲 PASO 7: Actualización de precios (Pila de Históricos)...");

    // Obtener un producto y actualizar su precio
    if let Some(producto) = inventario.buscar_por_codigo_binaria(100) {
        println!("\nProducto: {}", producto.nombre);
        println!("Precio actual: ${:.2}", producto.precio_actual);
        println!(
            "Historial de precios: {:?}",
            producto.obtener_historial_precios()
        );

        // Simular cambio de precio
        let mut prod_actualizado = producto.clone();
        prod_actualizado.actualizar_precio(1150.0);
        println!("\nDespués de actualizar precio a $1150.00:");
        println!(
            "Historial: {:?}",
            prod_actualizado.obtener_historial_precios()
        );
    }

    // =======================================================================
    // PASO 8: REPORTES Y ESTADÍSTICAS
    // =======================================================================
    println!("\n📈 PASO 8: Generando reportes...");

    Reporte::mostrar_bajo_stock(&inventario);
    Reporte::mostrar_productos_extremos(&inventario);
    Reporte::mostrar_top_5(&inventario);

    // =======================================================================
    // RESUMEN FINAL
    // =======================================================================
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                      RESUMEN FINAL                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("Total de productos: {}", inventario.cantidad_productos());
    println!(
        "Valor total del inventario: ${:.2}",
        inventario.valor_total_inventario()
    );
    println!(
        "Movimientos procesados: {}",
        registro.obtener_historial().len()
    );
    println!("Movimientos pendientes: {}", registro.cantidad_pendientes());
    println!("\n✓ Examen completado exitosamente");
}
