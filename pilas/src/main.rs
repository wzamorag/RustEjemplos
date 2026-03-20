// ============================================================================
// Una pila es una estructura de datos LIFO (Last In, First Out)
// Es decir, el último elemento que entra es el primero que sale
// Analogía: una pila de platos - agregas platos arriba y los quitas de arriba
// ============================================================================

// ============================================================================
// PARTE 1: IMPLEMENTACIÓN BÁSICA DE UNA PILA GENÉRICA
// ============================================================================

// Definimos una estructura de pila genérica que puede almacenar cualquier tipo de dato
struct Pila<T> {
    // Vec<T> es un vector dinámico que almacena elementos de tipo T
    // Usamos un vector como contenedor interno para guardar los elementos
    elementos: Vec<T>,
}

// Implementamos los métodos de la pila
impl<T> Pila<T> {
    // Método para crear una pila vacía
    // Self::new() retorna una nueva instancia de Pila
    fn nueva() -> Self {
        Pila {
            // Vec::new() crea un vector dinámico vacío
            elementos: Vec::new(),
        }
    }

    // Método para agregar un elemento a la pila (push)
    // &mut self permite modificar la pila actual
    // elemento: T es el valor que vamos a agregar a la pila
    fn apilar(&mut self, elemento: T) {
        // push() agrega el elemento al final del vector
        // En una pila, "arriba" es el final del vector
        self.elementos.push(elemento);
    }

    // Método para remover y retornar el elemento del tope de la pila (pop)
    // Retorna un Option<T>: Some(valor) si hay elementos, None si está vacía
    fn desapilar(&mut self) -> Option<T> {
        // pop() remueve el último elemento del vector y lo retorna
        // Si el vector está vacío, retorna None
        self.elementos.pop()
    }

    // Método para ver el elemento del tope sin removerlo (peek)
    // &self permite solo leer, no modificar la pila
    // Retorna Option<&T>: una referencia al último elemento o None
    fn tope(&self) -> Option<&T> {
        // last() retorna una referencia al último elemento del vector
        // .as_ref() convierte Option<&T> a la forma que queremos
        self.elementos.last()
    }

    // Método para verificar si la pila está vacía
    fn vacia(&self) -> bool {
        // is_empty() retorna true si el vector no tiene elementos
        self.elementos.is_empty()
    }

    // Método para obtener el número de elementos en la pila
    fn tamaño(&self) -> usize {
        // len() retorna la cantidad de elementos en el vector
        self.elementos.len()
    }

    // Método para limpiar la pila (remover todos los elementos)
    fn limpiar(&mut self) {
        // clear() remueve todos los elementos del vector
        self.elementos.clear();
    }
}

// ============================================================================
// PARTE 2: FUNCIÓN PARA VERIFICAR PARÉNTESIS BALANCEADOS
// Esta es una aplicación clásica de pilas en programación
// ============================================================================

// Esta función verifica si una cadena de paréntesis está correctamente balanceada
// Recibe una referencia a una cadena (&str)
// Retorna un bool: true si están balanceados, false si no
fn parentesis_balanceados(cadena: &str) -> bool {
    // Creamos una pila vacía de caracteres
    let mut pila: Pila<char> = Pila::nueva();

    // Iteramos sobre cada carácter de la cadena
    for caracter in cadena.chars() {
        // Si encontramos un paréntesis de apertura, lo apilamos
        if caracter == '(' {
            pila.apilar(caracter);
        }
        // Si encontramos un paréntesis de cierre
        else if caracter == ')' {
            // Verificamos si la pila está vacía
            if pila.vacia() {
                // Si está vacía, hay un paréntesis de cierre sin apertura
                return false;
            }
            // Si no está vacía, desapilamos el paréntesis de apertura correspondiente
            pila.desapilar();
        }
        // Ignoramos otros caracteres que no sean paréntesis
    }

    // Al final, la pila debe estar vacía
    // Si hay elementos, significa que hay paréntesis de apertura sin cerrar
    pila.vacia()
}

// ============================================================================
// PARTE 3: FUNCIÓN PARA INVERTIR UNA CADENA USANDO PILA
// ============================================================================

// Esta función invierte el orden de los caracteres de una cadena
// Recibe una referencia a una cadena (&str)
// Retorna un String con los caracteres invertidos
fn invertir_cadena(cadena: &str) -> String {
    // Creamos una pila vacía de caracteres
    let mut pila: Pila<char> = Pila::nueva();
    // Apilamos todos los caracteres de la cadena
    for caracter in cadena.chars() {
        // char es el tipo de dato para un carácter individual
        pila.apilar(caracter);
    }
    // Creamos una cadena nueva vacía para guardar el resultado
    let mut resultado = String::new();
    // Mientras la pila no esté vacía, desapilamos caracteres
    while !pila.vacia() {
        // desapilar() retorna Option<char>
        // Con unwrap(), obtenemos el valor dentro de Some
        // En este caso es seguro porque ya verificamos que no está vacía
        let caracter = pila.desapilar().unwrap();
        // push() agrega el carácter al final de la cadena resultado
        resultado.push(caracter);
    }

    // Retornamos la cadena invertida
    resultado
}

// ============================================================================
// PARTE 4: FUNCIÓN PRINCIPAL CON EJEMPLOS DE USO
// ============================================================================

fn main() {
    println!("===== EJEMPLO DE PILAS EN RUST =====\n");

    // =========== EJEMPLO 1: Operaciones básicas ===========
    println!("--- EJEMPLO 1: Operaciones Básicas ---");

    // Creamos una pila vacía de números enteros (i32)
    let mut pila_numeros: Pila<i32> = Pila::nueva();

    println!("1. Pila vacía creada");
    println!("   ¿Está vacía? {}", pila_numeros.vacia());

    // Apilamos algunos números
    println!("\n2. Apilando números: 10, 20, 30");
    pila_numeros.apilar(10);
    pila_numeros.apilar(20);
    pila_numeros.apilar(30);

    // Mostramos el tamaño de la pila
    println!("   Tamaño actual: {}", pila_numeros.tamaño());

    // Mostramos el elemento del tope sin removerlo
    println!("   Elemento del tope: {:?}", pila_numeros.tope());

    // Desapilamos un elemento
    println!("\n3. Desapilando un elemento");
    let elemento = pila_numeros.desapilar();
    println!("   Elemento removido: {:?}", elemento);
    println!("   Tamaño después de desapilar: {}", pila_numeros.tamaño());

    // Mostramos el nuevo tope
    println!("   Nuevo elemento del tope: {:?}", pila_numeros.tope());

    // Desapilamos todos los elementos
    println!("\n4. Desapilando todos los elementos:");
    while !pila_numeros.vacia() {
        // El while se ejecuta mientras la pila no esté vacía
        match pila_numeros.desapilar() {
            // Some(valor) significa que obtuvimos un elemento
            Some(valor) => println!("   Desapilado: {}", valor),
            // None significa que la pila está vacía (no debería ocurrir aquí)
            None => println!("   La pila está vacía"),
        }
    }

    println!("   ¿Está vacía? {}", pila_numeros.vacia());

    // =========== EJEMPLO 2: Verificar paréntesis balanceados ===========
    println!("\n--- EJEMPLO 2: Paréntesis Balanceados ---");

    // Array de cadenas para probar
    let expresiones = vec!["(())", "((()))", "(()", "())", "()", ""];

    // Probamos cada expresión
    for expr in expresiones {
        // Llamamos a la función de validación
        let resultado = parentesis_balanceados(expr);
        // Mostramos la expresión y si está balanceada
        println!("'{}' -> Balanceado: {}", expr, resultado);
    }

    // =========== EJEMPLO 3: Invertir una cadena ===========
    println!("\n--- EJEMPLO 3: Invertir Cadenas ---");

    let cadenas = vec!["Hola", "Rust", "Pila"];

    // Invertimos cada cadena
    for cadena in cadenas {
        // Llamamos a la función de inversión
        let invertida = invertir_cadena(cadena);
        // Mostramos la cadena original y la invertida
        println!("'{}' -> '{}'", cadena, invertida);
    }

    // =========== EJEMPLO 4: Pila de Strings ===========
    println!("\n--- EJEMPLO 4: Pila de Cadenas (Strings) ---");

    // Creamos una pila que almacena Strings (cadenas de texto)
    let mut pila_nombres: Pila<String> = Pila::nueva();

    println!("1. Apilando nombres:");
    // Apilamos nombres usando to_string() para convertir &str a String
    pila_nombres.apilar("Alice".to_string());
    pila_nombres.apilar("Bob".to_string());
    pila_nombres.apilar("Carlos".to_string());

    println!("   Tamaño: {}", pila_nombres.tamaño());

    println!("\n2. Desapilando en orden LIFO:");
    // Desapilamos todos los nombres
    while let Some(nombre) = pila_nombres.desapilar() {
        // while let es una forma cómoda de iterar sobre Option
        println!("   {}", nombre);
    }

    println!("\n===== FIN DEL EJEMPLO =====");
}

// ============================================================================
// CONCEPTO IMPORTANTE: LIFO (Last In, First Out)
// ============================================================================
// La característica principal de una pila es que el ÚLTIMO elemento
// agregado es el PRIMERO que sale.
//
// Secuencia de operaciones:
// Inicial: []
// Apilar 1: [1]
// Apilar 2: [1, 2]
// Apilar 3: [1, 2, 3]
// Desapilar: [1, 2]        <- Sacamos 3 (el último que entró)
// Desapilar: [1]           <- Sacamos 2
// Desapilar: []            <- Sacamos 1
//
// ============================================================================
// APLICACIONES PRÁCTICAS DE PILAS
// ============================================================================
// 1. Verificación de paréntesis/corchetes/llaves
// 2. Evaluación de expresiones matemáticas
// 3. Deshacer (Undo) en editores de texto
// 4. Recorrido de estructuras de árbol (DFS)
// 5. Manejo de llamadas a funciones (call stack)
// 6. Navegación en navegadores web (atrás/adelante)
// ============================================================================
