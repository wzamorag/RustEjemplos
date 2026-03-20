// ============================================================================
// EJEMPLO: COLAS (QUEUES) EN RUST
// ============================================================================
// Una cola es una estructura de datos FIFO (First In, First Out)
// Es decir, el PRIMER elemento que entra es el PRIMERO que sale
// Analogía: una fila en un banco - quien llega primero es atendido primero
// ============================================================================

// Importamos VecDeque que es la estructura ideal para implementar colas
// VecDeque es un doble-final-vector que permite operaciones eficientes en ambos extremos
use std::collections::VecDeque;

// ============================================================================
// PARTE 1: IMPLEMENTACIÓN BÁSICA DE UNA COLA GENÉRICA
// ============================================================================

// Definimos una estructura de cola genérica que puede almacenar cualquier tipo de dato
struct Cola<T> {
    // VecDeque<T> es una estructura que permite agregar/remover elementos
    // eficientemente tanto al inicio como al final
    // Usamos VecDeque en lugar de Vec porque es más eficiente para colas
    elementos: VecDeque<T>,
}

// Implementamos los métodos de la cola
impl<T> Cola<T> {
    // Método para crear una cola vacía
    // Self::new() retorna una nueva instancia de Cola
    fn nueva() -> Self {
        Cola {
            // VecDeque::new() crea una cola vacía
            elementos: VecDeque::new(),
        }
    }

    // Método para agregar un elemento al final de la cola (enqueue)
    // &mut self permite modificar la cola actual
    // elemento: T es el valor que vamos a agregar a la cola
    fn encolar(&mut self, elemento: T) {
        // push_back() agrega el elemento al final de la cola
        // En una cola FIFO, los nuevos elementos entran por el final
        self.elementos.push_back(elemento);
    }

    // Método para remover y retornar el primer elemento de la cola (dequeue)
    // Retorna un Option<T>: Some(valor) si hay elementos, None si está vacía
    fn desencolar(&mut self) -> Option<T> {
        // pop_front() remueve el primer elemento de la cola y lo retorna
        // Si la cola está vacía, retorna None
        // Este es el comportamiento FIFO: el primero que entró es el primero que sale
        self.elementos.pop_front()
    }

    // Método para ver el primer elemento sin removerlo (peek front)
    // &self permite solo leer, no modificar la cola
    // Retorna Option<&T>: una referencia al primer elemento o None
    fn frente(&self) -> Option<&T> {
        // front() retorna una referencia al primer elemento de la cola
        self.elementos.front()
    }

    // Método para ver el último elemento sin removerlo (peek back)
    // Útil para entender la estructura de la cola
    fn atras(&self) -> Option<&T> {
        // back() retorna una referencia al último elemento de la cola
        self.elementos.back()
    }

    // Método para verificar si la cola está vacía
    fn vacia(&self) -> bool {
        // is_empty() retorna true si la cola no tiene elementos
        self.elementos.is_empty()
    }

    // Método para obtener el número de elementos en la cola
    fn tamaño(&self) -> usize {
        // len() retorna la cantidad de elementos en la cola
        self.elementos.len()
    }

    // Método para limpiar la cola (remover todos los elementos)
    fn limpiar(&mut self) {
        // clear() remueve todos los elementos de la cola
        self.elementos.clear();
    }

    // Método para obtener una referencia a todos los elementos (para iterar)
    fn elementos_ref(&self) -> &VecDeque<T> {
        // Retornamos una referencia a la estructura interna
        &self.elementos
    }
}

// ============================================================================
// PARTE 2: SIMULACIÓN DE UN BANCO CON SISTEMA DE COLAS
// ============================================================================

// Estructura para representar a un cliente en el banco
#[derive(Clone, Debug)]
struct Cliente {
    // id: número único del cliente
    id: u32,
    // nombre: nombre del cliente
    nombre: String,
    // tiempo_atencion: minutos que tardará en ser atendido
    tiempo_atencion: u32,
}

// Función que simula un sistema de colas de atención en un banco
fn simular_banco() {
    println!("--- SIMULACIÓN DE BANCO CON COLA DE CLIENTES ---\n");

    // Creamos una cola de clientes
    let mut cola_banco: Cola<Cliente> = Cola::nueva();

    // Agregamos clientes a la cola
    println!("1. Llegada de clientes al banco:");

    // Cliente 1
    let cliente1 = Cliente {
        id: 1,
        nombre: "Juan".to_string(),
        tiempo_atencion: 5,
    };
    cola_banco.encolar(cliente1.clone());
    println!("   ✓ {} llega al banco", cliente1.nombre);

    // Cliente 2
    let cliente2 = Cliente {
        id: 2,
        nombre: "María".to_string(),
        tiempo_atencion: 8,
    };
    cola_banco.encolar(cliente2.clone());
    println!("   ✓ {} llega al banco", cliente2.nombre);

    // Cliente 3
    let cliente3 = Cliente {
        id: 3,
        nombre: "Carlos".to_string(),
        tiempo_atencion: 3,
    };
    cola_banco.encolar(cliente3.clone());
    println!("   ✓ {} llega al banco", cliente3.nombre);

    // Cliente 4
    let cliente4 = Cliente {
        id: 4,
        nombre: "Ana".to_string(),
        tiempo_atencion: 6,
    };
    cola_banco.encolar(cliente4.clone());
    println!("   ✓ {} llega al banco", cliente4.nombre);

    // Mostramos el estado de la cola
    println!("\n2. Estado de la cola:");
    println!("   Total de clientes esperando: {}", cola_banco.tamaño());
    println!(
        "   Primer cliente (siendo atendido o siguiente): {:?}",
        cola_banco.frente()
    );
    println!("   Último cliente en la fila: {:?}", cola_banco.atras());

    // Atendemos a los clientes
    println!("\n3. Atención de clientes (FIFO - Primero en Llegar, Primero en Salir):");

    let mut numero_cliente = 1;
    // Mientras hay clientes en la cola
    while !cola_banco.vacia() {
        // Desatendemos (sacamos) al siguiente cliente
        // unwrap() es seguro aquí porque verificamos que no está vacía
        let cliente = cola_banco.desencolar().unwrap();
        println!(
            "   Ventanilla {}: Atendiendo a {} (tiempo: {} minutos)",
            numero_cliente, cliente.nombre, cliente.tiempo_atencion
        );
        numero_cliente += 1;
    }

    println!("\n   ✓ Todos los clientes han sido atendidos");
    println!("   Cola vacía: {}", cola_banco.vacia());
}

// ============================================================================
// PARTE 3: SIMULACIÓN DE SISTEMA DE IMPRESIÓN (COLA DE TRABAJOS)
// ============================================================================

// Estructura para representar un trabajo de impresión
#[derive(Clone, Debug)]
struct TrabajoImpresion {
    // id: identificador único del trabajo
    id: u32,
    // nombre_archivo: nombre del archivo a imprimir
    nombre_archivo: String,
    // num_paginas: cantidad de páginas a imprimir
    num_paginas: u32,
    // prioridad: 1=baja, 2=normal, 3=alta
    prioridad: u32,
}

// Función que simula una cola de impresión
fn simular_impresora() {
    println!("\n--- SIMULACIÓN DE COLA DE IMPRESIÓN ---\n");

    // Creamos una cola de trabajos de impresión
    let mut cola_impresion: Cola<TrabajoImpresion> = Cola::nueva();

    println!("1. Envío de trabajos a la cola de impresión:");

    // Agregamos trabajos a la cola
    let trabajo1 = TrabajoImpresion {
        id: 1,
        nombre_archivo: "reporte.pdf".to_string(),
        num_paginas: 10,
        prioridad: 1,
    };
    cola_impresion.encolar(trabajo1.clone());
    println!(
        "   ✓ Trabajo 1: {} ({} páginas)",
        trabajo1.nombre_archivo, trabajo1.num_paginas
    );

    let trabajo2 = TrabajoImpresion {
        id: 2,
        nombre_archivo: "presentacion.pptx".to_string(),
        num_paginas: 25,
        prioridad: 2,
    };
    cola_impresion.encolar(trabajo2.clone());
    println!(
        "   ✓ Trabajo 2: {} ({} páginas)",
        trabajo2.nombre_archivo, trabajo2.num_paginas
    );

    let trabajo3 = TrabajoImpresion {
        id: 3,
        nombre_archivo: "documento.docx".to_string(),
        num_paginas: 5,
        prioridad: 1,
    };
    cola_impresion.encolar(trabajo3.clone());
    println!(
        "   ✓ Trabajo 3: {} ({} páginas)",
        trabajo3.nombre_archivo, trabajo3.num_paginas
    );

    println!("\n2. Estado de la cola de impresión:");
    println!("   Total de trabajos en cola: {}", cola_impresion.tamaño());
    println!("   Trabajo siendo impreso: {:?}", cola_impresion.frente());

    println!("\n3. Procesamiento de trabajos (FIFO):");

    // Procesamos los trabajos
    while !cola_impresion.vacia() {
        // Sacamos el siguiente trabajo
        let trabajo = cola_impresion.desencolar().unwrap();
        println!(
            "   Imprimiendo: {} - {} páginas",
            trabajo.nombre_archivo, trabajo.num_paginas
        );
    }

    println!("\n   ✓ Todos los trabajos han sido procesados");
}

// ============================================================================
// PARTE 4: FUNCIÓN PARA REVISAR PALÍNDROMOS USANDO COLA
// ============================================================================

// Función que verifica si una cadena es un palíndromo usando cola
// Un palíndromo se lee igual de adelante hacia atrás y de atrás hacia adelante
fn es_palindromo(cadena: &str) -> bool {
    // Limpiamos la cadena: convertimos a minúsculas y removemos espacios
    let limpia: String = cadena
        // to_lowercase() convierte a minúsculas
        .to_lowercase()
        // filter() solo mantiene caracteres que no sean espacios
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    // Creamos una cola con los caracteres
    let mut cola: Cola<char> = Cola::nueva();

    // Encolamos todos los caracteres
    for caracter in limpia.chars() {
        cola.encolar(caracter);
    }

    // Creamos un vector para los caracteres en orden inverso
    let mut inverso = Vec::new();

    // Desencólamos todos los caracteres (obtenemos el orden original)
    while !cola.vacia() {
        inverso.push(cola.desencolar().unwrap());
    }

    // Invertimos el vector para comparar
    inverso.reverse();

    // Convertimos el vector de caracteres de vuelta a string
    let inverso_str: String = inverso.iter().collect();
    let limpia_str = limpia.clone();

    // Si la cadena original es igual a la invertida, es palíndromo
    limpia_str == inverso_str
}

// ============================================================================
// PARTE 5: PROGRAMA PRINCIPAL CON EJEMPLOS
// ============================================================================

fn main() {
    println!("===== EJEMPLO DE COLAS EN RUST =====\n");

    // =========== EJEMPLO 1: Operaciones básicas ===========
    println!("--- EJEMPLO 1: Operaciones Básicas ---\n");

    // Creamos una cola vacía de números enteros
    let mut cola_numeros: Cola<i32> = Cola::nueva();

    println!("1. Cola vacía creada");
    println!("   ¿Está vacía? {}", cola_numeros.vacia());

    // Encolamos algunos números
    println!("\n2. Encolando números: 10, 20, 30, 40");
    cola_numeros.encolar(10);
    cola_numeros.encolar(20);
    cola_numeros.encolar(30);
    cola_numeros.encolar(40);

    // Mostramos el tamaño de la cola
    println!("   Tamaño actual: {}", cola_numeros.tamaño());

    // Mostramos el primer elemento (frente)
    println!("   Primer elemento (frente): {:?}", cola_numeros.frente());

    // Mostramos el último elemento (final)
    println!("   Último elemento (final): {:?}", cola_numeros.atras());

    // Desencólamos un elemento
    println!("\n3. Desencolando un elemento:");
    let elemento = cola_numeros.desencolar();
    println!("   Elemento sacado: {:?}", elemento);
    println!("   Tamaño después de desencolar: {}", cola_numeros.tamaño());
    println!("   Nuevo frente: {:?}", cola_numeros.frente());

    // Desencólamos todos los elementos
    println!("\n4. Desencólando todos los elementos (FIFO):");
    while !cola_numeros.vacia() {
        // El while se ejecuta mientras la cola no esté vacía
        match cola_numeros.desencolar() {
            // Some(valor) significa que obtuvimos un elemento
            Some(valor) => println!("   Desencolado: {}", valor),
            // None significa que la cola está vacía (no debería ocurrir aquí)
            None => println!("   Cola vacía"),
        }
    }

    println!("   ¿Está vacía? {}", cola_numeros.vacia());

    // =========== EJEMPLO 2: Operaciones con Strings ===========
    println!("\n--- EJEMPLO 2: Cola de Cadenas ---");

    let mut cola_tareas: Cola<String> = Cola::nueva();

    println!("1. Tareas a realizar:");
    // Encolamos tareas
    cola_tareas.encolar("Estudiar Rust".to_string());
    cola_tareas.encolar("Hacer ejercicio".to_string());
    cola_tareas.encolar("Leer un libro".to_string());
    cola_tareas.encolar("Programar proyecto".to_string());

    println!("\n2. Realizando tareas en orden de llegada:");
    // Desencólamos y realizamos las tareas
    while let Some(tarea) = cola_tareas.desencolar() {
        // while let es más cómodo que un while normal con match
        println!("   ✓ Completada: {}", tarea);
    }

    // =========== EJEMPLO 3: Simulación de Banco ===========
    simular_banco();

    // =========== EJEMPLO 4: Simulación de Impresora ===========
    simular_impresora();

    // =========== EJEMPLO 5: Palíndromos ===========
    println!("\n--- EJEMPLO 5: Verificación de Palíndromos ---\n");

    let textos = vec![
        "anilina",
        "reconocer",
        "racecar",
        "hola mundo",
        "Ama a Roma",
        "No es palíndromo",
    ];

    for texto in textos {
        let es_pal = es_palindromo(texto);
        println!("'{}' -> ¿Palíndromo? {}", texto, es_pal);
    }

    // =========== EJEMPLO 6: Comparación FIFO vs LIFO ===========
    println!("\n--- EJEMPLO 6: Diferencia entre Cola (FIFO) y Pila (LIFO) ---\n");

    println!("Mismo orden de entrada: 1, 2, 3, 4");

    // Con Cola (FIFO)
    println!("\nCola (FIFO - Primero Entra, Primero Sale):");
    let mut cola_demo: Cola<i32> = Cola::nueva();
    cola_demo.encolar(1);
    cola_demo.encolar(2);
    cola_demo.encolar(3);
    cola_demo.encolar(4);

    print!("Orden de salida: ");
    while let Some(val) = cola_demo.desencolar() {
        print!("{} ", val);
    }
    println!("← Orden original");

    // Con Pila (LIFO) - para comparación
    println!("\nPila (LIFO - Último Entra, Primero Sale):");
    let mut pila_demo = vec![1, 2, 3, 4];

    print!("Orden de salida: ");
    while !pila_demo.is_empty() {
        print!("{} ", pila_demo.pop().unwrap());
    }
    println!("← Orden invertido");

    println!("\n===== FIN DEL EJEMPLO =====");
}

// ============================================================================
// CONCEPTO IMPORTANTE: FIFO (First In, First Out)
// ============================================================================
// La característica principal de una cola es que el PRIMER elemento
// agregado es el PRIMERO que sale. Es como una fila en la vida real.
//
// Secuencia de operaciones:
// Inicial: []
// Encolar 1: [1]
// Encolar 2: [1, 2]
// Encolar 3: [1, 2, 3]
// Desencolar: [2, 3]       ← Sacamos 1 (el primero que entró)
// Desencolar: [3]          ← Sacamos 2
// Desencolar: []           ← Sacamos 3
//
// ============================================================================
// ¿POR QUÉ USAMOS VecDeque EN LUGAR DE Vec PARA COLAS?
// ============================================================================
// Vec: pop_front() es O(n) porque mueve todos los elementos
// VecDeque: pop_front() es O(1) porque usa índices circulares
//
// Para colas que necesitan remove/insert en ambos extremos,
// VecDeque es mucho más eficiente que Vec
//
// ============================================================================
// APLICACIONES PRÁCTICAS DE COLAS
// ============================================================================
// 1. Sistemas de atención al cliente (bancos, tiendas)
// 2. Colas de impresión
// 3. Planificación de procesos en sistemas operativos (CPU scheduling)
// 4. Búsqueda en amplitud (BFS) en grafos
// 5. Manejo de solicitudes HTTP en servidores web
// 6. Colas de eventos en aplicaciones
// 7. Buffering de datos (video streaming, audio)
// 8. Simulación de sistemas de atención
// ============================================================================
