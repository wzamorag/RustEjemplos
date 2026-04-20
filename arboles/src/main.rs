// ============================================================================
// ÁRBOL BINARIO DE BÚSQUEDA (ABB) - VERSIÓN DIDÁCTICA
// ============================================================================
// Este código implementa un Árbol Binario de Búsqueda en Rust.
// Es ideal para explicar conceptos de estructuras de datos recursivas y
// gestión de memoria en Rust.
// ============================================================================

// ============================================================================
// PASO 1: DEFINICIÓN DE LA ESTRUCTURA DEL NODO
// ============================================================================
// Un Nodo es el bloque básico de nuestro árbol.
//
// Explicación de cada campo:
// - valor: i32 → El dato que almacena el nodo
// - izquierda: Option<Box<Nodo>> → Puntero al hijo izquierdo (si existe)
// - derecha: Option<Box<Nodo>> → Puntero al hijo derecho (si existe)
//
// ¿Por qué usamos Box?
// Box permite almacenar datos en el heap en lugar del stack.
// Esto es crucial porque:
//   1. No conocemos el tamaño del árbol en tiempo de compilación
//   2. Los árboles son estructuras recursivas e indeterminadas
//
// ¿Por qué usamos Option?
// Option nos permite representar la ausencia de un nodo hijo.
// Cada hijo puede ser:
//   - Some(Box<Nodo>) → existe un nodo
//   - None → no existe un nodo en esa posición

#[derive(Debug)]
struct Nodo {
    valor: i32,
    izquierda: Option<Box<Nodo>>,
    derecha: Option<Box<Nodo>>,
}

// ============================================================================
// PASO 2: IMPLEMENTACIÓN DE MÉTODOS DEL NODO
// ============================================================================

impl Nodo {
    // ========================================================================
    // MÉTODO: nuevo() - Constructor
    // ========================================================================
    // Objetivo: Crear un nuevo nodo con un valor dado
    // Los hijos son inicialmente None (nodo hoja)
    //
    // Ejemplo:
    //   let nodo = Nodo::nuevo(50);
    //   Resultado: Nodo { valor: 50, izquierda: None, derecha: None }

    fn nuevo(valor: i32) -> Self {
        Nodo {
            valor,
            izquierda: None,
            derecha: None,
        }
    }

    // ========================================================================
    // MÉTODO: insertar() - Inserción recursiva
    // ========================================================================
    // Objetivo: Insertar un nuevo valor manteniendo la propiedad ABB
    //
    // PROPIEDAD ABB:
    // - Valores menores → rama izquierda
    // - Valores mayores → rama derecha
    // - Valores iguales → se ignoran (opcional, según el diseño)
    //
    // PROCESO (recursivo):
    // 1. Comparar nuevo_valor con el valor actual
    // 2. Si es menor: ir a la izquierda
    // 3. Si es mayor: ir a la derecha
    // 4. Si encontramos None: crear un nuevo nodo
    //
    // Ejemplo visual para insertar [50, 30, 70]:
    //
    //        50           50            50
    //       /   \   →    /   \    →    /  \
    //     None  None   30   None     30   70
    //
    // Complejidad de tiempo:
    // - Mejor caso (árbol equilibrado): O(log n)
    // - Peor caso (árbol degenerado): O(n)

    fn insertar(&mut self, nuevo_valor: i32) {
        if nuevo_valor < self.valor {
            // RAMA IZQUIERDA: El nuevo valor es menor
            match self.izquierda {
                // Si ya existe un nodo a la izquierda, recurso en él
                Some(ref mut nodo) => nodo.insertar(nuevo_valor),
                // Si no existe, creamos un nuevo nodo
                None => self.izquierda = Some(Box::new(Nodo::nuevo(nuevo_valor))),
            }
        } else if nuevo_valor > self.valor {
            // RAMA DERECHA: El nuevo valor es mayor
            match self.derecha {
                // Si ya existe un nodo a la derecha, recursamos en él
                Some(ref mut nodo) => nodo.insertar(nuevo_valor),
                // Si no existe, creamos un nuevo nodo
                None => self.derecha = Some(Box::new(Nodo::nuevo(nuevo_valor))),
            }
        }
        // NOTA: Si nuevo_valor == self.valor, no hacemos nada (valor duplicado)
    }

    // ========================================================================
    // MÉTODO: buscar() - Búsqueda recursiva
    // ========================================================================
    // Objetivo: Determinar si un valor existe en el árbol
    // Retorno: true si existe, false si no existe
    //
    // PROCESO (recursivo):
    // 1. ¿El valor actual es el objetivo? → Retornar true
    // 2. ¿El objetivo es menor? → Buscar en la rama izquierda
    // 3. ¿El objetivo es mayor? → Buscar en la rama derecha
    // 4. Si no hay rama disponible → Retornar false
    //
    // Ejemplo de búsqueda de 40 en el árbol [50, 30, 70, 20, 40]:
    //
    //        50  ← ¿40 == 50? No. ¿40 < 50? Sí → ir izquierda
    //       /  \
    //      30   70 ← ¿40 == 30? No. ¿40 < 30? No. ¿40 > 30? Sí → ir derecha
    //     / \
    //    20 40 ← ¿40 == 40? Sí → true
    //
    // Complejidad de tiempo:
    // - Mejor caso: O(1) - el valor está en la raíz
    // - Promedio: O(log n) - árbol equilibrado
    // - Peor caso: O(n) - árbol degenerado (lista)

    fn buscar(&self, objetivo: i32) -> bool {
        // Caso base: ¿Encontramos el valor?
        if objetivo == self.valor {
            return true;
        }

        // ¿Debemos buscar a la izquierda?
        if objetivo < self.valor {
            match &self.izquierda {
                Some(nodo) => nodo.buscar(objetivo), // Recursamos a la izquierda
                None => false,                       // No hay rama izquierda
            }
        }
        // El objetivo es mayor, buscar a la derecha
        else {
            match &self.derecha {
                Some(nodo) => nodo.buscar(objetivo), // Recursamos a la derecha
                None => false,                       // No hay rama derecha
            }
        }
    }

    // ========================================================================
    // MÉTODO: recorrido_inorder() - Recorrido en profundidad
    // ========================================================================
    // Objetivo: Visitar todos los nodos en orden (izquierda → raíz → derecha)
    // Resultado: Los valores se guardan en orden ascendente
    //
    // PROCESO (recursivo - Divide y Conquista):
    // 1. Procesar subtítulo izquierdo (recursivamente)
    // 2. "Visitar" el nodo actual (guardar su valor)
    // 3. Procesar subtítulo derecho (recursivamente)
    //
    // Ejemplo visual para el árbol [50, 30, 70, 20, 40, 60, 80]:
    //
    //           50
    //         /    \
    //        30     70
    //       / \    / \
    //      20 40  60 80
    //
    // Recorrido inorder:
    //   20 → 30 → 40 → 50 → 60 → 70 → 80  (¡ORDENADO!)
    //
    // Pasos detallados:
    //   1. Visitar(20) → 20 no tiene hijos → guarda 20
    //   2. Visitar(30) → primero izq(20), luego 30, luego der(40)
    //   3. Visitar(40) → 40 no tiene hijos → guarda 40
    //   ... y así sucesivamente
    //
    // Complejidad:
    // - Tiempo: O(n) - visitamos cada nodo exactamente una vez
    // - Espacio: O(h) - donde h es la altura del árbol (pila de recursión)

    fn recorrido_inorder(&self, resultado: &mut Vec<i32>) {
        // PASO 1: Procesar subtítulo izquierdo
        if let Some(ref nodo_izq) = self.izquierda {
            nodo_izq.recorrido_inorder(resultado);
        }

        // PASO 2: Procesar el nodo actual (visita)
        resultado.push(self.valor);

        // PASO 3: Procesar subtítulo derecho
        if let Some(ref nodo_der) = self.derecha {
            nodo_der.recorrido_inorder(resultado);
        }
    }

    // ========================================================================
    // MÉTODO BONUS: altura() - Calcular la altura del árbol
    // ========================================================================
    // Objetivo: Determinar la altura de un nodo
    // Altura = máximo número de aristas desde el nodo hasta una hoja
    //
    // DEFINICIÓN:
    // - Un nodo hoja tiene altura 0
    // - Un nodo con hijos tiene altura = 1 + max(altura_izq, altura_der)
    //
    // Complejidad: O(n) - debe visitar todos los nodos

    fn altura(&self) -> i32 {
        let altura_izq = match &self.izquierda {
            Some(nodo) => nodo.altura(),
            None => 0,
        };

        let altura_der = match &self.derecha {
            Some(nodo) => nodo.altura(),
            None => 0,
        };

        1 + altura_izq.max(altura_der)
    }

    // ========================================================================
    // MÉTODO BONUS: contar_nodos() - Contar elementos
    // ========================================================================
    // Objetivo: Determinar cuántos nodos hay en el subárbol
    // Fórmula: 1 (nodo actual) + contar(izq) + contar(der)

    fn contar_nodos(&self) -> i32 {
        let izq = match &self.izquierda {
            Some(nodo) => nodo.contar_nodos(),
            None => 0,
        };

        let der = match &self.derecha {
            Some(nodo) => nodo.contar_nodos(),
            None => 0,
        };

        1 + izq + der
    }
}

// ============================================================================
// PASO 3: FUNCIÓN MAIN - DEMOSTRACIONES
// ============================================================================

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║   ÁRBOL BINARIO DE BÚSQUEDA (ABB) - DEMOSTRACIÓN DIDÁCTICA  ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // FASE 1: CREAR LA RAÍZ
    // ========================================================================
    println!("📌 FASE 1: Crear la raíz del árbol");
    println!("────────────────────────────────────");
    let mut raiz = Nodo::nuevo(50);
    println!("Raíz creada con valor: 50\n");

    // ========================================================================
    // FASE 2: INSERTAR VALORES
    // ========================================================================
    println!("📌 FASE 2: Insertar valores");
    println!("────────────────────────────────────");
    let valores = [30, 70, 20, 40, 60, 80];
    println!("Insertando valores: {:?}\n", valores);

    println!("Orden de inserción y ubicación:");
    println!("  50 (raíz)");
    println!("  ├─ 30 → menor que 50, va a izquierda");
    println!("  ├─ 70 → mayor que 50, va a derecha");
    println!("  ├─ 20 → menor que 50, menor que 30, va a izq-izq");
    println!("  ├─ 40 → menor que 50, mayor que 30, va a izq-der");
    println!("  ├─ 60 → mayor que 50, menor que 70, va a der-izq");
    println!("  └─ 80 → mayor que 50, mayor que 70, va a der-der\n");

    for v in valores {
        raiz.insertar(v);
    }
    println!("✓ Inserción completada\n");

    // ========================================================================
    // FASE 3: BÚSQUEDAS
    // ========================================================================
    println!("📌 FASE 3: Búsquedas en el árbol");
    println!("────────────────────────────────────");

    let valores_buscar = [(40, true), (99, false), (50, true), (20, true)];
    for (valor, esperado) in valores_buscar {
        let resultado = raiz.buscar(valor);
        let símbolo = if resultado == esperado { "✓" } else { "✗" };
        println!(
            "{} ¿Existe el {}? {} (esperado: {})",
            símbolo, valor, resultado, esperado
        );
    }
    println!();

    // ========================================================================
    // FASE 4: ESTRUCTURA DEL ÁRBOL
    // ========================================================================
    println!("📌 FASE 4: Estructura interna del árbol");
    println!("────────────────────────────────────");
    println!("Representación completa (Debug format):");
    println!("{:#?}\n", raiz);

    // ========================================================================
    // FASE 5: RECORRIDO IN-ORDER
    // ========================================================================
    println!("📌 FASE 5: Recorrido In-order (izq → raíz → der)");
    println!("────────────────────────────────────");
    let mut lista_ordenada = Vec::new();
    raiz.recorrido_inorder(&mut lista_ordenada);

    println!("Valores en orden: {:?}", lista_ordenada);
    println!("Observación: ¡Los valores están ordenados de menor a mayor!\n");

    // ========================================================================
    // FASE 6: ESTADÍSTICAS
    // ========================================================================
    println!("📌 FASE 6: Estadísticas del árbol");
    println!("────────────────────────────────────");
    let num_nodos = raiz.contar_nodos();
    let altura = raiz.altura();

    println!("Número total de nodos: {}", num_nodos);
    println!("Altura del árbol: {}", altura);
    println!(
        "Árbol está bien equilibrado: {}",
        if altura <= (num_nodos as f32).log2().ceil() as i32 {
            "Sí ✓"
        } else {
            "No"
        }
    );
    println!();

    // ========================================================================
    // FASE 7: EXPLICACIÓN PEDAGÓGICA FINAL
    // ========================================================================
    println!("📌 FASE 7: Conceptos clave");
    println!("────────────────────────────────────");
    println!("1. PROPIEDAD ABB:");
    println!("   - Cada nodo tiene máximo 2 hijos");
    println!("   - Valores menores a la izquierda");
    println!("   - Valores mayores a la derecha");
    println!();
    println!("2. VENTAJAS:");
    println!("   - Búsqueda rápida: O(log n) en promedio");
    println!("   - Datos ordenados naturalmente");
    println!("   - Mejor que lista lineal para grandes volúmenes");
    println!();
    println!("3. COMPLEJIDADES:");
    println!("   - Inserción: O(log n) promedio, O(n) peor caso");
    println!("   - Búsqueda: O(log n) promedio, O(n) peor caso");
    println!("   - Recorrido: O(n) siempre");
    println!();
    println!("4. RUST CONCEPTS:");
    println!("   - Box<T>: Almacenamiento en heap");
    println!("   - Option<T>: Representa ausencia de valor");
    println!("   - &mut self: Necesario para modificar el árbol");
    println!("   - match: Manejo seguro de Option");
    println!();
}

// ============================================================================
// EJERCICIOS PROPUESTOS PARA LOS ESTUDIANTES
// ============================================================================
//
// 1. EXTENSIÓN BÁSICA:
//    - Implementar recorrido preorder (raíz → izq → der)
//    - Implementar recorrido postorder (izq → der → raíz)
//    - ¿Qué utilidad tiene cada uno?
//
// 2. NIVEL INTERMEDIO:
//    - Implementar método eliminar(valor) - caso más difícil
//    - Implementar buscar_mínimo() y buscar_máximo()
//    - Implementar suma_todos() - suma de todos los valores
//
// 3. NIVEL AVANZADO:
//    - Auto-balanceo (árbol AVL o Rojo-Negro)
//    - Iterador personalizado (sin recursión)
//    - Visualización ASCII del árbol
//    - Convertir a Árbol de Búsqueda Balanceado
//
// ============================================================================
