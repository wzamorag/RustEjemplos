// ============================================================================
// EJEMPLO: TABLAS HASH EN RUST
// ============================================================================
// Una tabla hash es una estructura de datos que implementa un arreglo
// asociativo: una estructura que mapea claves a valores.
//
// Ventajas:
// - Búsqueda promedio: O(1)
// - Inserción promedio: O(1)
// - Eliminación promedio: O(1)
//
// Desafíos:
// - Colisiones de hash (dos claves con el mismo hash)
// - Necesidad de función hash adecuada
// - Factor de carga (cantidad de elementos / capacidad)
// ============================================================================

use std::collections::HashMap;

// ============================================================================
// PARTE 1: CONCEPTOS FUNDAMENTALES - FUNCIONES HASH
// ============================================================================

// Definición: Una función hash es una función matemática que convierte
// una entrada (clave) en un número (hash) que indica una posición en la tabla.
//
// Características de una buena función hash:
// 1. Determinística: igual entrada → igual salida
// 2. Rápida: debe calcularse rápidamente
// 3. Uniforme: distribuye bien los valores en el rango
// 4. Mínimas colisiones: diferentes claves → diferentes hashes
// 5. Avalancha: pequeño cambio en entrada → gran cambio en salida

// Función hash simple: suma de caracteres módulo tamaño
// Esta es una función hash básica educativa (no se usa en producción)
fn hash_simple(clave: &str, tamaño: usize) -> usize {
    // Inicializamos el acumulador en cero
    let mut suma = 0;

    // Iteramos sobre cada byte (carácter) de la clave
    for byte in clave.bytes() {
        // Sumamos el valor ASCII de cada carácter
        suma += byte as usize;
    }

    // Retornamos el módulo para asegurar que el hash esté en rango [0, tamaño)
    suma % tamaño
}

// Función hash mejorada: polinomial (Polynomial Rolling Hash)
// Mejor distribución que la suma simple
fn hash_polinomial(clave: &str, tamaño: usize) -> usize {
    // Inicializamos el hash en cero
    let mut hash: u64 = 0;

    // Base para el polinomio (número primo, típicamente 31 o 53)
    let base: u64 = 31;

    // Potencia de la base para cada posición
    let mut poder = 1u64;

    // Iteramos sobre cada carácter de la clave
    for byte in clave.bytes() {
        // Multiplicamos el hash por la base y sumamos el valor del carácter
        hash = (hash + (byte as u64) * poder) % (1_000_000_007);
        // Actualizamos la potencia para el siguiente carácter
        poder = (poder * base) % (1_000_000_007);
    }

    // Retornamos el hash en el rango válido
    (hash as usize) % tamaño
}

// Función hash djb2 (Daniel J. Bernstein)
// Una de las funciones hash más populares
fn hash_djb2(clave: &str, tamaño: usize) -> usize {
    // Iniciamos el hash con el valor mágico 5381
    let mut hash: u64 = 5381;

    // Iteramos sobre cada byte de la clave
    for byte in clave.bytes() {
        // Multiplicamos por 33 y sumamos el byte
        // La fórmula es: hash = hash * 33 + byte
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
    }

    // Retornamos en el rango válido
    (hash as usize) % tamaño
}

// ============================================================================
// PARTE 2: TABLA HASH CON ENCADENAMIENTO (CHAINING)
// ============================================================================

// Definición de Resolución de Colisiones por Encadenamiento:
// Cuando dos claves producen el mismo hash, los pares clave-valor se
// almacenan en una lista enlazada. Esto evita perder datos pero reduce O(1)
// a O(k) donde k es el tamaño de la cadena en colisión.

// Estructura para un par clave-valor
#[derive(Clone, Debug, PartialEq)]
struct ParClaveValor<K: Clone, V: Clone> {
    // clave: la llave para acceder al valor
    clave: K,
    // valor: el dato asociado a la clave
    valor: V,
}

// Estructura de tabla hash con encadenamiento
struct TablaHashEncadenamiento<K: Clone, V: Clone> {
    // buckets: vector de vectores (cada bucket es una cadena de pares)
    // Si dos claves hashean al mismo índice, se almacenan en el mismo bucket
    buckets: Vec<Vec<ParClaveValor<K, V>>>,

    // capacidad: tamaño de la tabla (número de buckets)
    capacidad: usize,

    // elementos: número total de elementos almacenados
    elementos: usize,
}

impl<K: Clone + PartialEq + std::fmt::Debug, V: Clone> TablaHashEncadenamiento<K, V> {
    // Constructor: crear una tabla hash vacía
    fn nueva(capacidad: usize) -> Self {
        // Inicializamos con capacidad buckets vacíos
        let buckets = vec![Vec::new(); capacidad];

        TablaHashEncadenamiento {
            buckets,
            capacidad,
            elementos: 0,
        }
    }

    // Función de hash genérica (usa la implementación de Hash de Rust)
    fn hash(&self, clave: &K) -> usize {
        // Convertimos a string para usar nuestra función hash
        // (En producción, usaríamos el trait Hash de Rust)
        let clave_str = format!("{:?}", clave);
        hash_djb2(&clave_str, self.capacidad)
    }

    // INSERCIÓN: agregar un par clave-valor a la tabla
    fn insertar(&mut self, clave: K, valor: V) {
        // Obtenemos el índice del bucket calculando el hash
        let indice = self.hash(&clave);

        // Accedemos al bucket (cadena) en ese índice
        let bucket = &mut self.buckets[indice];

        // Buscamos si la clave ya existe en el bucket
        for par in bucket.iter_mut() {
            // Si encontramos la clave, actualizamos su valor
            if par.clave == clave {
                par.valor = valor;
                return; // Salimos, no necesitamos hacer nada más
            }
        }

        // Si llegamos aquí, la clave no existe, así que la agregamos
        bucket.push(ParClaveValor { clave, valor });

        // Incrementamos el contador de elementos
        self.elementos += 1;
    }

    // BÚSQUEDA: obtener el valor asociado a una clave
    fn obtener(&self, clave: &K) -> Option<V> {
        // Calculamos el hash de la clave para encontrar el bucket
        let indice = self.hash(clave);

        // Buscamos en el bucket (cadena) específico
        for par in &self.buckets[indice] {
            // Si encontramos la clave, retornamos el valor
            if &par.clave == clave {
                return Some(par.valor.clone());
            }
        }

        // Si no encontramos la clave, retornamos None
        None
    }

    // Eliminar un par clave-valor
    fn eliminar(&mut self, clave: &K) -> Option<V> {
        // Calculamos el hash de la clave
        let indice = self.hash(clave);

        // Accedemos al bucket
        let bucket = &mut self.buckets[indice];

        // Buscamos la clave en el bucket
        for (i, par) in bucket.iter().enumerate() {
            // Si encontramos la clave
            if &par.clave == clave {
                // Removemos el par del bucket
                let par_removido = bucket.remove(i);
                // Decrementamos el contador de elementos
                self.elementos -= 1;
                // Retornamos el valor que estaba asociado
                return Some(par_removido.valor);
            }
        }

        // Si no encontramos la clave, retornamos None
        None
    }

    // Obtener el factor de carga
    // Factor de carga = elementos / capacidad
    fn factor_carga(&self) -> f64 {
        self.elementos as f64 / self.capacidad as f64
    }

    // Obtener cantidad de colisiones
    fn contar_colisiones(&self) -> usize {
        // Inicializamos el contador
        let mut colisiones = 0;

        // Iteramos sobre cada bucket
        for bucket in &self.buckets {
            // Si un bucket tiene más de un elemento, hay colisiones
            // (Colisión = más de un elemento en el mismo bucket)
            if bucket.len() > 1 {
                // Sumamos (tamaño del bucket - 1) colisiones
                colisiones += bucket.len() - 1;
            }
        }

        colisiones
    }

    // Obtener utilización de buckets
    fn buckets_utilizados(&self) -> usize {
        // Contamos cuántos buckets tienen al menos un elemento
        self.buckets.iter().filter(|b| !b.is_empty()).count()
    }
}

// ============================================================================
// PARTE 3: TABLA HASH CON DIRECCIONAMIENTO ABIERTO
// ============================================================================

// Definición: Direccionamiento Abierto (Open Addressing)
// Cuando hay una colisión, se busca otra posición vacía en la tabla
// (no se usa encadenamiento). Los métodos comunes son:
// - Sondeo lineal: siguiente posición (i+1, i+2, ...)
// - Sondeo cuadrático: (i+1², i+2², ...)
// - Doble hash: usa una segunda función hash

#[derive(Clone, Debug, PartialEq)]
enum EstadoSlot<K: Clone, V: Clone> {
    // Vacío: nunca fue ocupado
    Vacio,
    // Ocupado: tiene un par clave-valor
    Ocupado(ParClaveValor<K, V>),
    // Eliminado: fue ocupado pero se eliminó (marca para rehashing)
    Eliminado,
}

// Tabla hash con direccionamiento abierto (sondeo lineal)
struct TablaHashAbierta<K: Clone, V: Clone> {
    // slots: vector de estados de slots
    slots: Vec<EstadoSlot<K, V>>,

    // capacidad: tamaño actual de la tabla
    capacidad: usize,

    // elementos: número de elementos activos
    elementos: usize,
}

impl<K: Clone + PartialEq + std::fmt::Debug, V: Clone + PartialEq> TablaHashAbierta<K, V> {
    // Constructor
    fn nueva(capacidad: usize) -> Self {
        // Inicializamos todos los slots como vacíos
        let slots = vec![EstadoSlot::Vacio; capacidad];

        TablaHashAbierta {
            slots,
            capacidad,
            elementos: 0,
        }
    }

    // Función de hash
    fn hash(&self, clave: &K) -> usize {
        let clave_str = format!("{:?}", clave);
        hash_djb2(&clave_str, self.capacidad)
    }

    // INSERCIÓN con sondeo lineal
    fn insertar(&mut self, clave: K, valor: V) {
        // Obtenemos el índice inicial
        let mut indice = self.hash(&clave);

        // Contador para evitar bucle infinito
        let mut intentos = 0;

        // Mientras no encontremos un slot donde insertar
        loop {
            // Verificamos si hemos probado todos los slots
            if intentos >= self.capacidad {
                // La tabla está llena, esto no debería pasar en práctica
                panic!("Tabla hash llena");
            }

            // Obtenemos el slot en la posición actual
            match &self.slots[indice] {
                // Si el slot está vacío, podemos insertar aquí
                EstadoSlot::Vacio => {
                    self.slots[indice] = EstadoSlot::Ocupado(ParClaveValor { clave, valor });
                    self.elementos += 1;
                    return;
                }

                // Si el slot está marcado como eliminado, podemos reutilizarlo
                EstadoSlot::Eliminado => {
                    self.slots[indice] = EstadoSlot::Ocupado(ParClaveValor { clave, valor });
                    self.elementos += 1;
                    return;
                }

                // Si está ocupado, verificamos si es la misma clave
                EstadoSlot::Ocupado(par) => {
                    if par.clave == clave {
                        // Actualizar valor existente
                        self.slots[indice] = EstadoSlot::Ocupado(ParClaveValor { clave, valor });
                        return;
                    }
                    // Colisión: probar siguiente posición (sondeo lineal)
                }
            }

            // Pasamos a la siguiente posición (sondeo lineal)
            indice = (indice + 1) % self.capacidad;
            intentos += 1;
        }
    }

    // BÚSQUEDA con sondeo lineal
    fn obtener(&self, clave: &K) -> Option<V> {
        // Obtenemos el índice inicial
        let mut indice = self.hash(clave);

        // Contador para evitar búsqueda infinita
        let mut intentos = 0;

        // Buscamos mientras no encontremos un slot vacío
        while intentos < self.capacidad {
            match &self.slots[indice] {
                // Si encontramos un slot vacío, la clave no existe
                EstadoSlot::Vacio => {
                    return None;
                }

                // Si encontramos un slot ocupado, verificamos la clave
                EstadoSlot::Ocupado(par) => {
                    if &par.clave == clave {
                        return Some(par.valor.clone());
                    }
                }

                // Si está marcado como eliminado, continuamos buscando
                EstadoSlot::Eliminado => {}
            }

            // Probamos la siguiente posición
            indice = (indice + 1) % self.capacidad;
            intentos += 1;
        }

        None
    }

    // Eliminar: marca el slot como eliminado
    fn eliminar(&mut self, clave: &K) -> Option<V> {
        let mut indice = self.hash(clave);
        let mut intentos = 0;

        while intentos < self.capacidad {
            match &self.slots[indice] {
                EstadoSlot::Vacio => {
                    return None;
                }

                EstadoSlot::Ocupado(par) => {
                    if &par.clave == clave {
                        let valor = par.valor.clone();
                        // Marcamos como eliminado, no como vacío
                        // Esto preserva la cadena de sondeo
                        self.slots[indice] = EstadoSlot::Eliminado;
                        self.elementos -= 1;
                        return Some(valor);
                    }
                }

                EstadoSlot::Eliminado => {}
            }

            indice = (indice + 1) % self.capacidad;
            intentos += 1;
        }

        None
    }
}

// ============================================================================
// PARTE 4: TABLA HASH DINÁMICA
// ============================================================================

// Definición de Hash Dinámico:
// Una tabla hash dinámica aumenta automáticamente su capacidad cuando
// el factor de carga excede un umbral, y rehashea todos los elementos
// a los nuevos buckets. Esto mantiene O(1) amortizado.

struct TablaHashDinamica<K: Clone + PartialEq + std::fmt::Debug, V: Clone> {
    // tabla: la tabla hash interna
    tabla: TablaHashEncadenamiento<K, V>,

    // factor_carga_maximo: umbral para rehashear (típicamente 0.75)
    factor_carga_maximo: f64,
}

impl<K: Clone + PartialEq + std::fmt::Debug, V: Clone> TablaHashDinamica<K, V> {
    // Constructor
    fn nueva() -> Self {
        TablaHashDinamica {
            tabla: TablaHashEncadenamiento::nueva(16), // Capacidad inicial
            factor_carga_maximo: 0.75,
        }
    }

    // Insertar con rehashing automático
    fn insertar(&mut self, clave: K, valor: V) {
        // Insertamos el elemento
        self.tabla.insertar(clave.clone(), valor);

        // Verificamos si necesitamos rehashear
        if self.tabla.factor_carga() > self.factor_carga_maximo {
            self.rehashear();
        }
    }

    // Rehashear: duplica la capacidad y rehashea todos los elementos
    fn rehashear(&mut self) {
        // Calculamos la nueva capacidad
        let nueva_capacidad = self.tabla.capacidad * 2;

        println!(
            "  [Rehashing] Expandiendo tabla de {} a {}",
            self.tabla.capacidad, nueva_capacidad
        );

        // Creamos una tabla nueva con mayor capacidad
        let mut tabla_nueva = TablaHashEncadenamiento::nueva(nueva_capacidad);

        // Reinsertamos todos los elementos en la tabla nueva
        for bucket in &self.tabla.buckets {
            for par in bucket {
                tabla_nueva.insertar(par.clave.clone(), par.valor.clone());
            }
        }

        // Reemplazamos la tabla antigua con la nueva
        self.tabla = tabla_nueva;
    }

    // Búsqueda
    fn obtener(&self, clave: &K) -> Option<V> {
        self.tabla.obtener(clave)
    }

    // Eliminación
    fn eliminar(&mut self, clave: &K) -> Option<V> {
        self.tabla.eliminar(clave)
    }

    // Obtener estadísticas
    fn estadisticas(&self) -> (usize, usize, f64, usize) {
        (
            self.tabla.elementos,
            self.tabla.capacidad,
            self.tabla.factor_carga(),
            self.tabla.contar_colisiones(),
        )
    }
}

// ============================================================================
// PARTE 5: TABLA HASH DISTRIBUIDA (SIMULACIÓN EDUCATIVA)
// ============================================================================

// Definición de Tabla Hash Distribuida:
// En sistemas distribuidos, la tabla hash se divide entre múltiples
// servidores/nodos. Una clave se distribuye a un nodo basado en su hash.
// Ejemplo: Memcached, Redis Cluster.

// Estructura para simular un nodo en la tabla hash distribuida
#[derive(Clone)]
struct NodoDistribuido<V: Clone> {
    // id: identificador del nodo
    id: usize,

    // datos: tabla hash local en este nodo (siempre usa String como clave)
    datos: HashMap<String, V>,
}

impl<V: Clone> NodoDistribuido<V> {
    // Constructor
    fn nuevo(id: usize) -> Self {
        NodoDistribuido {
            id,
            datos: HashMap::new(),
        }
    }

    // Almacenar un valor
    fn almacenar(&mut self, clave: String, valor: V) {
        self.datos.insert(clave, valor);
    }

    // Recuperar un valor
    fn obtener(&self, clave: &str) -> Option<&V> {
        self.datos.get(clave)
    }

    // Obtener número de elementos almacenados
    fn tamaño(&self) -> usize {
        self.datos.len()
    }
}

// Tabla hash distribuida
struct TablaHashDistribuida<V: Clone> {
    // nodos: vector de nodos que componen el cluster
    nodos: Vec<NodoDistribuido<V>>,

    // num_nodos: número de nodos en el cluster
    num_nodos: usize,
}

impl<V: Clone> TablaHashDistribuida<V> {
    // Constructor: crear un cluster con num_nodos nodos
    fn nueva(num_nodos: usize) -> Self {
        // Creamos num_nodos nodos vacíos
        let nodos = (0..num_nodos).map(|i| NodoDistribuido::nuevo(i)).collect();

        TablaHashDistribuida { nodos, num_nodos }
    }

    // Función de hash para distribuir datos entre nodos
    fn hash_nodo(&self, clave: &str) -> usize {
        // Usamos la función hash djb2 para obtener un número
        let hash = hash_djb2(clave, 1_000_000);
        // Mapeamos al número de nodo usando módulo
        hash % self.num_nodos
    }

    // Almacenar un valor en el nodo correspondiente
    fn almacenar(&mut self, clave: String, valor: V) {
        // Determinamos qué nodo debe almacenar este dato
        let nodo_id = self.hash_nodo(&clave);

        // Almacenamos en el nodo correspondiente
        self.nodos[nodo_id].almacenar(clave, valor);
    }

    // Obtener un valor del nodo correspondiente
    fn obtener(&self, clave: &str) -> Option<&V> {
        // Determinamos qué nodo contiene este dato
        let nodo_id = self.hash_nodo(clave);

        // Obtenemos del nodo correspondiente
        self.nodos[nodo_id].obtener(clave)
    }

    // Obtener estadísticas de distribución
    fn estadisticas(&self) -> Vec<(usize, usize)> {
        // Retornamos (id del nodo, número de elementos)
        self.nodos
            .iter()
            .map(|nodo| (nodo.id, nodo.tamaño()))
            .collect()
    }
}

// ============================================================================
// PARTE 6: FUNCIÓN PRINCIPAL CON EJEMPLOS
// ============================================================================

fn main() {
    println!("===== TABLAS HASH EN RUST =====\n");

    // =========== EJEMPLO 1: Funciones Hash ===========
    println!("--- EJEMPLO 1: Comparación de Funciones Hash ---\n");

    let claves = vec!["alice", "bob", "carlos", "diana"];
    let tamaño = 10;

    println!("Tabla de tamaño: {}\n", tamaño);
    println!(
        "{:<10} | {:^15} | {:^15} | {:^15}",
        "Clave", "Hash Simple", "Hash Polinomial", "Hash DJB2"
    );
    println!("{:-<10}-+-{:-^15}-+-{:-^15}-+-{:-^15}", "", "", "", "");

    for clave in &claves {
        let h1 = hash_simple(clave, tamaño);
        let h2 = hash_polinomial(clave, tamaño);
        let h3 = hash_djb2(clave, tamaño);

        println!("{:<10} | {:^15} | {:^15} | {:^15}", clave, h1, h2, h3);
    }

    // =========== EJEMPLO 2: Tabla Hash con Encadenamiento ===========
    println!("\n--- EJEMPLO 2: Tabla Hash con Encadenamiento ---\n");

    let mut tabla_encadenamiento: TablaHashEncadenamiento<String, i32> =
        TablaHashEncadenamiento::nueva(5);

    println!("Insertando elementos en tabla de tamaño 5:\n");

    let datos = vec![
        ("alice", 25),
        ("bob", 30),
        ("carlos", 28),
        ("diana", 27),
        ("eva", 26),
        ("felipe", 29),
    ];

    for (nombre, edad) in datos {
        tabla_encadenamiento.insertar(nombre.to_string(), edad);
        println!("  Insertado: {} (edad: {})", nombre, edad);
    }

    println!("\nEstadísticas:");
    println!("  Elementos: {}", tabla_encadenamiento.elementos);
    println!("  Capacidad: {}", tabla_encadenamiento.capacidad);
    println!(
        "  Factor de carga: {:.2}",
        tabla_encadenamiento.factor_carga()
    );
    println!(
        "  Buckets utilizados: {}",
        tabla_encadenamiento.buckets_utilizados()
    );
    println!("  Colisiones: {}", tabla_encadenamiento.contar_colisiones());

    println!("\nBúsquedas:");
    for nombre in vec!["alice", "carlos", "zebra"] {
        match tabla_encadenamiento.obtener(&nombre.to_string()) {
            Some(edad) => println!("  {} tiene {} años", nombre, edad),
            None => println!("  {} no se encontró", nombre),
        }
    }

    // =========== EJEMPLO 3: Tabla Hash con Direccionamiento Abierto ===========
    println!("\n--- EJEMPLO 3: Tabla Hash con Direccionamiento Abierto ---\n");

    let mut tabla_abierta: TablaHashAbierta<String, String> = TablaHashAbierta::nueva(10);

    println!("Insertando en tabla con direccionamiento abierto:\n");

    tabla_abierta.insertar("manzana".to_string(), "fruta roja".to_string());
    tabla_abierta.insertar("banana".to_string(), "fruta amarilla".to_string());
    tabla_abierta.insertar("ciruela".to_string(), "fruta morada".to_string());
    tabla_abierta.insertar("durazno".to_string(), "fruta naranja".to_string());

    println!("Elementos insertados: {}", tabla_abierta.elementos);
    println!("Capacidad: {}", tabla_abierta.capacidad);

    println!("\nBúsquedas:");
    for fruta in vec!["manzana", "ciruela", "naranja"] {
        match tabla_abierta.obtener(&fruta.to_string()) {
            Some(desc) => println!("  {} → {}", fruta, desc),
            None => println!("  {} no encontrado", fruta),
        }
    }

    // =========== EJEMPLO 4: Tabla Hash Dinámica ===========
    println!("\n--- EJEMPLO 4: Tabla Hash Dinámica (con Rehashing) ---\n");

    let mut tabla_dinamica: TablaHashDinamica<String, i32> = TablaHashDinamica::nueva();

    println!("Insertando elementos con rehashing automático:\n");

    // Insertamos muchos elementos para provocar rehashing
    for i in 1..=20 {
        let clave = format!("usuario_{}", i);
        tabla_dinamica.insertar(clave, i);

        let (elem, cap, carga, col) = tabla_dinamica.estadisticas();

        // Mostrar cada inserción
        if i <= 5 || i % 5 == 0 || carga > 0.7 {
            println!(
                "  Paso {}: elementos={}, capacidad={}, carga={:.2}, colisiones={}",
                i, elem, cap, carga, col
            );
        }
    }

    println!("\nEstadísticas finales:");
    let (elem, cap, carga, col) = tabla_dinamica.estadisticas();
    println!("  Elementos: {}", elem);
    println!("  Capacidad: {}", cap);
    println!("  Factor de carga: {:.2}", carga);
    println!("  Colisiones: {}", col);

    // =========== EJEMPLO 5: Tabla Hash Distribuida ===========
    println!("\n--- EJEMPLO 5: Tabla Hash Distribuida ---\n");

    // Creamos un cluster con 4 nodos
    let mut tabla_distribuida: TablaHashDistribuida<String> = TablaHashDistribuida::nueva(4);

    println!("Cluster con {} nodos\n", 4);

    // Datos a distribuir
    let datos_distribuidos = vec![
        ("alice_perfil", "usuaria premium"),
        ("bob_datos", "datos de cliente"),
        ("carlos_cache", "resultado en cache"),
        ("diana_logs", "registros de actividad"),
        ("eva_config", "configuración"),
        ("felipe_sesion", "sesión activa"),
        ("gabriela_temp", "datos temporales"),
    ];

    println!("Distribuyendo datos entre nodos:\n");

    for (clave, valor) in &datos_distribuidos {
        tabla_distribuida.almacenar(clave.to_string(), valor.to_string());

        // Calculamos a qué nodo fue
        let nodo = tabla_distribuida.hash_nodo(clave);
        println!("  {} → Nodo {}", clave, nodo);
    }

    println!("\nDistribución por nodo:");
    for (nodo_id, cantidad) in tabla_distribuida.estadisticas() {
        println!("  Nodo {}: {} elementos", nodo_id, cantidad);
    }

    println!("\nBúsquedas en cluster distribuido:");
    for (clave, _) in &datos_distribuidos[..3] {
        if let Some(valor) = tabla_distribuida.obtener(clave) {
            let nodo = tabla_distribuida.hash_nodo(clave);
            println!("  {} (Nodo {}): {}", clave, nodo, valor);
        }
    }

    // =========== EJEMPLO 6: Usando HashMap de Rust ===========
    println!("\n--- EJEMPLO 6: HashMap Nativo de Rust ---\n");

    // Rust proporciona HashMap optimizado
    let mut diccionario: HashMap<String, String> = HashMap::new();

    println!("Usando HashMap nativo de Rust:\n");

    diccionario.insert("cat".to_string(), "animal doméstico".to_string());
    diccionario.insert("dog".to_string(), "mejor amigo del hombre".to_string());
    diccionario.insert("bird".to_string(), "animal con plumas".to_string());

    println!("Elementos almacenados: {}", diccionario.len());

    println!("\nBúsquedas:");
    for palabra in vec!["cat", "dog", "fish"] {
        match diccionario.get(palabra) {
            Some(significado) => println!("  {} → {}", palabra, significado),
            None => println!("  {} no está en el diccionario", palabra),
        }
    }

    // =========== EJEMPLO 7: Caso Práctico - Sistema de Caché ===========
    println!("\n--- EJEMPLO 7: Caso Práctico - Sistema de Caché ---\n");

    let mut cache: HashMap<String, String> = HashMap::new();

    println!("Simulando un sistema de caché:\n");

    // Función que consulta caché primero, luego calcula
    fn obtener_resultado(cache: &mut HashMap<String, String>, query: &str) -> String {
        // Verificamos si ya está en caché
        if let Some(resultado) = cache.get(query) {
            println!("  ✓ HIT: {} (desde caché)", query);
            resultado.clone()
        } else {
            println!("  ✗ MISS: {} (calculando...)", query);
            // Simulamos cálculo
            let resultado = format!("Resultado de '{}'", query);
            cache.insert(query.to_string(), resultado.clone());
            resultado
        }
    }

    println!("Consultas:");
    obtener_resultado(&mut cache, "SELECT * FROM users");
    obtener_resultado(&mut cache, "SELECT * FROM users"); // Hit
    obtener_resultado(&mut cache, "SELECT * FROM products");
    obtener_resultado(&mut cache, "SELECT * FROM users"); // Hit nuevamente

    println!("\n===== FIN DEL EJEMPLO =====");
}

// ============================================================================
// RESUMEN: CONCEPTOS CLAVE DE TABLAS HASH
// ============================================================================
//
// 1. FUNCIÓN HASH
//    - Convierte una clave en un índice de tabla
//    - Características: determinística, rápida, uniforme, pocos choques
//
// 2. COLISIONES
//    - Ocurren cuando dos claves producen el mismo hash
//    - Soluciones: encadenamiento o direccionamiento abierto
//
// 3. ENCADENAMIENTO (Chaining)
//    - Cada bucket contiene una lista enlazada
//    - Ventaja: simple, fácil de eliminar
//    - Desventaja: uso extra de memoria
//
// 4. DIRECCIONAMIENTO ABIERTO (Open Addressing)
//    - Cuando hay colisión, buscar otra posición vacía
//    - Métodos: sondeo lineal, cuadrático, doble hash
//    - Ventaja: mejor localidad de caché
//    - Desventaja: más complejo para eliminación
//
// 5. FACTOR DE CARGA
//    - Razón elementos / capacidad
//    - Influye en probabilidad de colisiones
//    - Típicamente se rehashea cuando excede 0.75
//
// 6. HASH DINÁMICO
//    - Crece automáticamente cuando factor de carga es alto
//    - Mantiene O(1) amortizado
//    - Requiere rehashing: O(n)
//
// 7. HASH DISTRIBUIDO
//    - Distribuye datos entre múltiples nodos
//    - Útil para sistemas a gran escala
//    - Ejemplos: Memcached, Redis, DynamoDB
//
// COMPLEJIDAD:
// - Inserción: O(1) promedio
// - Búsqueda: O(1) promedio
// - Eliminación: O(1) promedio
// - Rehashing: O(n)
//
// ============================================================================
