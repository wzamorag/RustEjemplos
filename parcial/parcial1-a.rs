// ============================================================================
// EXAMEN PARCIAL RUST - ALTERNATIVA 1: SISTEMA DE PROCESAMIENTO DE PEDIDOS
// Solución Completa - Todos los temas integrados
// ============================================================================

use std::collections::{HashMap, VecDeque};

// ============================================================================
// PARTE 1: PILA Y ENUM - Gestión de Estados
// ============================================================================

/// Enum que representa los estados por los que pasa un pedido
#[derive(Debug, Clone, PartialEq)]
enum EstadoPedido {
    Recibido,
    Procesando,
    Enviado,
    Entregado,
}

impl EstadoPedido {
    /// Retorna la representación en texto del estado
    fn descripcion(&self) -> &'static str {
        match self {
            EstadoPedido::Recibido => "Recibido",
            EstadoPedido::Procesando => "Procesando",
            EstadoPedido::Enviado => "Enviado",
            EstadoPedido::Entregado => "Entregado",
        }
    }
}

/// Estructura que representa un pedido
#[derive(Debug, Clone)]
struct Pedido {
    id: u32,
    cliente: String,
    monto: f64,
    // PILA: Vector que almacena los estados (comportamiento LIFO)
    estados: Vec<EstadoPedido>,
}

impl Pedido {
    /// Constructor de un nuevo pedido
    fn nuevo(id: u32, cliente: String, monto: f64) -> Self {
        let mut pedido = Pedido {
            id,
            cliente,
            monto,
            estados: Vec::new(),
        };
        // Al crear el pedido, su primer estado es "Recibido"
        pedido.agregar_estado(EstadoPedido::Recibido);
        pedido
    }

    /// Agrega un nuevo estado a la pila (push)
    fn agregar_estado(&mut self, estado: EstadoPedido) {
        self.estados.push(estado);
    }

    /// Obtiene el estado actual (top de la pila) sin remover
    fn estado_actual(&self) -> Option<&EstadoPedido> {
        self.estados.last()
    }

    /// Finaliza el pedido (desapila todos los estados)
    fn finalizar(&mut self) {
        while !self.estados.is_empty() {
            self.estados.pop();
        }
    }

    /// Obtiene el historial completo de estados
    fn historial_estados(&self) -> Vec<&EstadoPedido> {
        self.estados.iter().collect()
    }
}

// ============================================================================
// PARTE 2: COLA (QUEUE) - Procesamiento de Pedidos
// ============================================================================

/// Sistema que gestiona la cola de procesamiento de pedidos
#[derive(Debug)]
struct SistemaProcesamiento {
    // COLA: VecDeque para comportamiento FIFO
    cola_pendientes: VecDeque<Pedido>,
    // Vector de pedidos ya procesados (para búsqueda posterior)
    pedidos_procesados: Vec<Pedido>,
}

impl SistemaProcesamiento {
    /// Crea un nuevo sistema de procesamiento vacío
    fn nuevo() -> Self {
        SistemaProcesamiento {
            cola_pendientes: VecDeque::new(),
            pedidos_procesados: Vec::new(),
        }
    }

    /// Encola un nuevo pedido (push_back)
    fn encolar_pedido(&mut self, pedido: Pedido) {
        self.cola_pendientes.push_back(pedido);
    }

    /// Procesa el siguiente pedido en la cola (pop_front)
    /// Mueve el pedido de la cola a procesados y actualiza su estado
    fn procesar_siguiente(&mut self) -> Option<Pedido> {
        if let Some(mut pedido) = self.cola_pendientes.pop_front() {
            pedido.agregar_estado(EstadoPedido::Procesando);
            self.pedidos_procesados.push(pedido.clone());
            Some(pedido)
        } else {
            None
        }
    }

    /// Retorna la cantidad de pedidos en cola
    fn cantidad_pendientes(&self) -> usize {
        self.cola_pendientes.len()
    }

    /// Lista todos los pedidos pendientes en la cola
    fn listar_pendientes(&self) {
        if self.cola_pendientes.is_empty() {
            println!("No hay pedidos pendientes.");
            return;
        }

        println!("\n=== PEDIDOS PENDIENTES EN COLA ===");
        for (idx, pedido) in self.cola_pendientes.iter().enumerate() {
            println!(
                "{}. [ID: {}] {} - ${:.2}",
                idx + 1,
                pedido.id,
                pedido.cliente,
                pedido.monto
            );
        }
    }

    /// Retorna referencia mutable de pedidos procesados para ordenamiento
    fn pedidos_procesados_mut(&mut self) -> &mut Vec<Pedido> {
        &mut self.pedidos_procesados
    }

    /// Retorna referencia de pedidos procesados
    fn pedidos_procesados(&self) -> &Vec<Pedido> {
        &self.pedidos_procesados
    }
}

// ============================================================================
// PARTE 3: BÚSQUEDA BINARIA - Buscar Pedidos por ID
// ============================================================================

/// Implementación de búsqueda binaria para pedidos ordenados por ID
impl SistemaProcesamiento {
    /// Busca un pedido por ID usando BÚSQUEDA BINARIA
    /// Precondición: Los pedidos deben estar ordenados por ID
    fn buscar_pedido_por_id(&self, id: u32) -> Option<&Pedido> {
        let mut izq = 0;
        let mut der = self.pedidos_procesados.len();

        while izq < der {
            let mid = izq + (der - izq) / 2;

            match self.pedidos_procesados[mid].id.cmp(&id) {
                std::cmp::Ordering::Equal => return Some(&self.pedidos_procesados[mid]),
                std::cmp::Ordering::Less => izq = mid + 1,
                std::cmp::Ordering::Greater => der = mid,
            }
        }

        None
    }

    /// Ordena los pedidos procesados por ID (necesario para búsqueda binaria)
    fn ordenar_pedidos_por_id(&mut self) {
        self.pedidos_procesados.sort_by_key(|p| p.id);
    }
}

// ============================================================================
// PARTE 4: TABLA HASH (HashMap) - Gestión de Clientes
// ============================================================================

/// Gestor de información de clientes usando HashMap
#[derive(Debug)]
struct GestorClientes {
    // TABLA HASH: String (nombre cliente) -> (total_gastado, cantidad_pedidos)
    clientes: HashMap<String, (f64, u32)>,
}

impl GestorClientes {
    /// Crea un nuevo gestor de clientes vacío
    fn nuevo() -> Self {
        GestorClientes {
            clientes: HashMap::new(),
        }
    }

    /// Agrega o actualiza un cliente con un nuevo pedido
    fn agregar_pedido(&mut self, cliente: &str, monto: f64) {
        self.clientes
            .entry(cliente.to_string())
            .and_modify(|(total, cantidad)| {
                *total += monto;
                *cantidad += 1;
            })
            .or_insert((monto, 1));
    }

    /// Obtiene estadísticas de un cliente específico
    fn obtener_estadisticas(&self, cliente: &str) -> Option<(f64, u32)> {
        self.clientes.get(cliente).copied()
    }

    /// Lista los 3 clientes que más han gastado (búsqueda secuencial)
    fn obtener_top_3_clientes(&self) -> Vec<(String, f64, u32)> {
        let mut clientes_vec: Vec<_> = self
            .clientes
            .iter()
            .map(|(nombre, (total, cantidad))| (nombre.clone(), *total, *cantidad))
            .collect();

        // BÚSQUEDA SECUENCIAL: Recorrer para encontrar los mejores 3
        clientes_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // Ordena por total gastado
        clientes_vec.into_iter().take(3).collect()
    }

    /// Muestra estadísticas de todos los clientes
    fn listar_clientes(&self) {
        if self.clientes.is_empty() {
            println!("No hay clientes registrados.");
            return;
        }

        println!("\n=== ESTADÍSTICAS DE CLIENTES ===");
        for (cliente, (total, cantidad)) in &self.clientes {
            println!("{}: ${:.2} ({} pedidos)", cliente, total, cantidad);
        }
    }
}

// ============================================================================
// PARTE 5: INTEGRACIÓN COMPLETA - Programa Principal
// ============================================================================

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     SISTEMA DE PROCESAMIENTO DE PEDIDOS - EXAMEN RUST     ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    // Crear instancias del sistema
    let mut sistema = SistemaProcesamiento::nuevo();
    let mut gestor_clientes = GestorClientes::nuevo();

    // =======================================================================
    // Paso 1: CREAR Y ENCOLAR PEDIDOS (Parte 2)
    // =======================================================================
    println!("\n📦 PASO 1: Creando y encolando 5 pedidos...");

    let pedidos_iniciales = vec![
        ("Juan García", 500.0),
        ("María López", 750.0),
        ("Pedro Martínez", 600.0),
        ("Juan García", 450.0),
        ("Ana Rodríguez", 800.0),
    ];

    for (idx, (cliente, monto)) in pedidos_iniciales.iter().enumerate() {
        let pedido = Pedido::nuevo((idx + 1) as u32, cliente.to_string(), *monto);
        sistema.encolar_pedido(pedido);
        gestor_clientes.agregar_pedido(cliente, monto);
    }

    println!("✓ Se encolaron {} pedidos", 5);
    sistema.listar_pendientes();

    // =======================================================================
    // Paso 2: PROCESAR PEDIDOS (Parte 2 - Cola, Parte 1 - Pila)
    // =======================================================================
    println!("\n⚙️ PASO 2: Procesando 3 pedidos de la cola...");

    for i in 1..=3 {
        if let Some(pedido) = sistema.procesar_siguiente() {
            println!(
                "✓ Procesado: [ID: {}] {} - ${:.2}",
                pedido.id, pedido.cliente, pedido.monto
            );
            println!("  Estado actual: {:?}", pedido.estado_actual());
        }
    }

    println!("Pedidos aún en cola: {}", sistema.cantidad_pendientes());

    // =======================================================================
    // Paso 3: ORDENAR Y BUSCAR POR ID (Parte 3 - Búsqueda Binaria)
    // =======================================================================
    println!("\n🔍 PASO 3: Ordenando pedidos por ID y realizando búsqueda binaria...");

    sistema.ordenar_pedidos_por_id();

    let id_buscar = 2;
    match sistema.buscar_pedido_por_id(id_buscar) {
        Some(pedido) => {
            println!("✓ Pedido encontrado [ID: {}]:", id_buscar);
            println!("  Cliente: {}", pedido.cliente);
            println!("  Monto: ${:.2}", pedido.monto);
            println!(
                "  Estado actual: {}",
                pedido.estado_actual().unwrap().descripcion()
            );
            println!(
                "  Historial: {:?}",
                pedido
                    .historial_estados()
                    .iter()
                    .map(|e| e.descripcion())
                    .collect::<Vec<_>>()
            );
        }
        None => println!("✗ Pedido con ID {} no encontrado", id_buscar),
    }

    // =======================================================================
    // Paso 4: ESTADÍSTICAS DE CLIENTES (Parte 4 - HashMap)
    // =======================================================================
    println!("\n📊 PASO 4: Estadísticas de clientes (HashMap)...");

    gestor_clientes.listar_clientes();

    // =======================================================================
    // Paso 5: TOP 3 CLIENTES (Parte 4 - Búsqueda Secuencial)
    // =======================================================================
    println!("\n🏆 PASO 5: Top 3 clientes que más han gastado...");

    let top_3 = gestor_clientes.obtener_top_3_clientes();
    for (idx, (cliente, total, cantidad)) in top_3.iter().enumerate() {
        println!(
            "{}. {} - ${:.2} ({} pedidos)",
            idx + 1,
            cliente,
            total,
            cantidad
        );
    }

    // =======================================================================
    // Paso 6: DEMOSTRACIÓN DE PILA CON UN PEDIDO
    // =======================================================================
    println!("\n📚 PASO 6: Demostración de PILA con transiciones de estado...");

    let mut pedido_demo = Pedido::nuevo(99, "Cliente Demo".to_string(), 1000.0);

    println!(
        "Estados iniciales del pedido: {:?}",
        pedido_demo.historial_estados()
    );
    println!(
        "Estado actual: {}",
        pedido_demo.estado_actual().unwrap().descripcion()
    );

    pedido_demo.agregar_estado(EstadoPedido::Enviado);
    println!("\nDespués de enviar:");
    println!(
        "Estados: {:?}",
        pedido_demo
            .historial_estados()
            .iter()
            .map(|e| e.descripcion())
            .collect::<Vec<_>>()
    );

    pedido_demo.agregar_estado(EstadoPedido::Entregado);
    println!("\nDespués de entregar:");
    println!(
        "Estados: {:?}",
        pedido_demo
            .historial_estados()
            .iter()
            .map(|e| e.descripcion())
            .collect::<Vec<_>>()
    );

    // =======================================================================
    // RESUMEN FINAL
    // =======================================================================
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                      RESUMEN FINAL                        ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!("Pedidos procesados: {}", sistema.pedidos_procesados().len());
    println!(
        "Pedidos pendientes en cola: {}",
        sistema.cantidad_pendientes()
    );
    println!("Total de clientes: {}", gestor_clientes.clientes.len());
    println!("\n✓ Examen completado exitosamente");
}
