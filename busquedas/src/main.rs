// ============================================================================
// EJEMPLO: BÚSQUEDA SECUENCIAL Y BÚSQUEDA BINARIA EN RUST
// ============================================================================
// Este programa demuestra dos algoritmos fundamentales de búsqueda:
// 1. Búsqueda Secuencial (Lineal): O(n)
// 2. Búsqueda Binaria: O(log n)
// ============================================================================

// ============================================================================
// PARTE 1: BÚSQUEDA SECUENCIAL (LINEAR SEARCH)
// ============================================================================

// Definición: La búsqueda secuencial o lineal es un algoritmo que examina
// cada elemento de una lista de forma ordenada (de inicio a fin) hasta
// encontrar el elemento buscado o llegar al final de la lista.
//
// Características:
// - Tiempo: O(n) en el peor caso
// - Espacio: O(1)
// - NO requiere que el arreglo esté ordenado
// - Funciona con cualquier tipo de datos que se puedan comparar

// Función que implementa búsqueda secuencial
// Parámetros:
//   arreglo: &[i32] - referencia a un slice de números (no modifica)
//   objetivo: i32 - el valor que estamos buscando
// Retorna: Option<usize> - Some(índice) si encuentra, None si no
fn busqueda_secuencial(arreglo: &[i32], objetivo: i32) -> Option<usize> {
    // Iteramos sobre cada elemento del arreglo con su índice
    // enumerate() nos da tanto el índice como el valor
    for (indice, &elemento) in arreglo.iter().enumerate() {
        // Si encontramos el elemento buscado
        if elemento == objetivo {
            // Retornamos Some(indice) indicando que lo encontramos
            return Some(indice);
        }
        // Si no coincide, continuamos al siguiente elemento
    }
    // Si terminamos el bucle sin encontrar nada, retornamos None
    None
}
// Función alternativa que retorna un booleano (solo si existe o no)
fn existe_secuencial(arreglo: &[i32], objetivo: i32) -> bool {
    // Usamos any() que itera y retorna true si algún elemento cumple la condición
    arreglo.iter().any(|&elemento| elemento == objetivo)
}
// Función que cuenta cuántas veces aparece un elemento
fn contar_ocurrencias(arreglo: &[i32], objetivo: i32) -> usize {
    // Usamos filter() para contar solo los elementos que coinciden
    // count() retorna cuántos elementos pasaron el filtro
    arreglo
        .iter()
        .filter(|&&elemento| elemento == objetivo)
        .count()
}

// ============================================================================
// PARTE 2: BÚSQUEDA BINARIA (BINARY SEARCH)
// ============================================================================

// Definición: La búsqueda binaria es un algoritmo que divide el espacio
// de búsqueda a la mitad en cada iteración, eliminando la mitad que no
// contiene el elemento buscado.
//
// Características:
// - Tiempo: O(log n) en todos los casos
// - Espacio: O(1) para versión iterativa, O(log n) para recursiva
// - REQUIERE que el arreglo esté ordenado
// - Mucho más eficiente que búsqueda secuencial para arreglos grandes

// Función que implementa búsqueda binaria iterativa
// Parámetros:
//   arreglo: &[i32] - referencia a un slice ordenado de números
//   objetivo: i32 - el valor que estamos buscando
// Retorna: Option<usize> - Some(índice) si encuentra, None si no
fn busqueda_binaria(arreglo: &[i32], objetivo: i32) -> Option<usize> {
    // izquierda: índice del inicio del rango de búsqueda
    let mut izquierda = 0;
    // derecha: índice del final del rango de búsqueda
    // len() - 1 porque los índices van de 0 a len()-1
    let mut derecha = if arreglo.is_empty() {
        return None; // Si el arreglo está vacío, no hay nada que buscar
    } else {
        arreglo.len() - 1
    };
    // Continuamos mientras el rango sea válido (izquierda <= derecha)
    while izquierda <= derecha {
        // Calculamos el punto medio del rango actual
        // Usamos (izquierda + derecha) / 2 para encontrar el elemento central
        let medio = izquierda + (derecha - izquierda) / 2;
        // Obtenemos el elemento en la posición media
        let elemento_medio = arreglo[medio];
        // Comparamos el elemento del medio con el objetivo
        if elemento_medio == objetivo {
            // ¡Encontramos el elemento!
            return Some(medio);
        } else if elemento_medio < objetivo {
            // El elemento buscado está en la mitad derecha
            // Movemos izquierda al punto después del medio
            izquierda = medio + 1;
        } else {
            // El elemento buscado está en la mitad izquierda
            // Movemos derecha al punto antes del medio
            // Usamos saturating_sub() para evitar underflow
            derecha = medio.saturating_sub(1);
        }
    }
    // Si salimos del bucle sin encontrar nada, retornamos None
    None
}

// Función que implementa búsqueda binaria recursiva
// Esta versión ayuda a entender cómo funciona la recursión en búsqueda
fn busqueda_binaria_recursiva(
    arreglo: &[i32],
    objetivo: i32,
    izquierda: usize,
    derecha: usize,
) -> Option<usize> {
    // Caso base: si el rango es inválido, no encontramos nada
    if izquierda > derecha {
        return None;
    }

    // Calculamos el punto medio
    let medio = izquierda + (derecha - izquierda) / 2;
    let elemento_medio = arreglo[medio];

    // Comparamos el elemento del medio con el objetivo
    if elemento_medio == objetivo {
        // Encontramos el elemento
        Some(medio)
    } else if elemento_medio < objetivo {
        // Buscamos en la mitad derecha de forma recursiva
        busqueda_binaria_recursiva(arreglo, objetivo, medio + 1, derecha)
    } else {
        // Buscamos en la mitad izquierda de forma recursiva
        busqueda_binaria_recursiva(arreglo, objetivo, izquierda, medio.saturating_sub(1))
    }
}

// ============================================================================
// PARTE 3: FUNCIÓN DE UTILIDAD PARA MEDIR COMPARACIONES
// ============================================================================

// Estructura para guardar estadísticas de búsqueda
#[derive(Debug)]
struct EstadisticasBusqueda {
    // encontrado: si el elemento fue encontrado
    encontrado: bool,
    // indice: en qué posición está el elemento (si fue encontrado)
    indice: Option<usize>,
    // comparaciones: cuántas comparaciones se realizaron
    comparaciones: usize,
}

// Búsqueda secuencial con conteo de comparaciones
fn busqueda_secuencial_contando(arreglo: &[i32], objetivo: i32) -> EstadisticasBusqueda {
    // Inicializamos el contador de comparaciones
    let mut comparaciones = 0;

    // Iteramos sobre cada elemento
    for (indice, &elemento) in arreglo.iter().enumerate() {
        // Incrementamos el contador por cada comparación
        comparaciones += 1;

        // Comparamos el elemento actual con el objetivo
        if elemento == objetivo {
            // Encontramos el elemento, retornamos los resultados
            return EstadisticasBusqueda {
                encontrado: true,
                indice: Some(indice),
                comparaciones,
            };
        }
    }

    // No encontramos el elemento
    EstadisticasBusqueda {
        encontrado: false,
        indice: None,
        comparaciones,
    }
}

// Búsqueda binaria con conteo de comparaciones
fn busqueda_binaria_contando(arreglo: &[i32], objetivo: i32) -> EstadisticasBusqueda {
    // Inicializamos el contador de comparaciones
    let mut comparaciones = 0;

    let mut izquierda = 0;
    let mut derecha = if arreglo.is_empty() {
        return EstadisticasBusqueda {
            encontrado: false,
            indice: None,
            comparaciones: 0,
        };
    } else {
        arreglo.len() - 1
    };

    // Continuamos mientras el rango sea válido
    while izquierda <= derecha {
        // Calculamos el punto medio
        let medio = izquierda + (derecha - izquierda) / 2;
        let elemento_medio = arreglo[medio];

        // Incrementamos el contador por cada comparación
        comparaciones += 1;

        // Comparamos y ajustamos el rango
        if elemento_medio == objetivo {
            // Encontramos el elemento
            return EstadisticasBusqueda {
                encontrado: true,
                indice: Some(medio),
                comparaciones,
            };
        } else if elemento_medio < objetivo {
            izquierda = medio + 1;
        } else {
            derecha = medio.saturating_sub(1);
        }
    }

    // No encontramos el elemento
    EstadisticasBusqueda {
        encontrado: false,
        indice: None,
        comparaciones,
    }
}

// ============================================================================
// PARTE 4: FUNCIÓN PRINCIPAL CON EJEMPLOS
// ============================================================================

fn main() {
    println!("===== BÚSQUEDA SECUENCIAL VS BÚSQUEDA BINARIA EN RUST =====\n");
    // =========== EJEMPLO 1: Búsqueda Secuencial Básica ===========
    println!("--- EJEMPLO 1: Búsqueda Secuencial Básica ---\n");
    // Arreglo desordenado (búsqueda secuencial NO requiere orden)
    let numeros_desordenados = vec![15, 3, 9, 27, 1, 45, 8, 12, 33, 6];
    println!("Arreglo: {:?}", numeros_desordenados);
    println!("(No está ordenado - búsqueda secuencial funciona igual)\n");
    // Buscamos varios números
    let objetivos = vec![27, 100, 1, 45];
    println!("Buscando elementos:");
    for objetivo in objetivos {
        match busqueda_secuencial(&numeros_desordenados, objetivo) {
            // Si encontramos el elemento
            Some(indice) => {
                println!("  {} encontrado en índice {}", objetivo, indice);
            }
            // Si no encontramos el elemento
            None => {
                println!("  {} NO encontrado", objetivo);
            }
        }
    }

    // =========== EJEMPLO 2: Búsqueda Binaria Básica ===========
    println!("\n--- EJEMPLO 2: Búsqueda Binaria Básica ---\n");
    // Arreglo ordenado (búsqueda binaria REQUIERE orden)
    let numeros_ordenados = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25];
    println!("Arreglo: {:?}", numeros_ordenados);
    println!("(Está ordenado - búsqueda binaria es mucho más eficiente)\n");
    println!("Buscando elementos:");
    for objetivo in vec![13, 100, 1, 19] {
        match busqueda_binaria(&numeros_ordenados, objetivo) {
            Some(indice) => {
                println!("  {} encontrado en índice {}", objetivo, indice);
            }
            None => {
                println!("  {} NO encontrado", objetivo);
            }
        }
    }

    // =========== EJEMPLO 3: Búsqueda Binaria Recursiva ===========
    println!("\n--- EJEMPLO 3: Búsqueda Binaria Recursiva ---\n");

    println!("Usando versión recursiva:");
    for objetivo in vec![7, 20, 25] {
        match busqueda_binaria_recursiva(
            &numeros_ordenados,
            objetivo,
            0,
            numeros_ordenados.len() - 1,
        ) {
            Some(indice) => {
                println!("  {} encontrado en índice {}", objetivo, indice);
            }
            None => {
                println!("  {} NO encontrado", objetivo);
            }
        }
    }

    // =========== EJEMPLO 4: Funciones Alternativas ===========
    println!("\n--- EJEMPLO 4: Funciones Alternativas ---\n");

    let numeros = vec![2, 4, 2, 6, 2, 8, 2, 10];

    // Verificar si existe (booleano)
    println!("¿Existe el 2? {}", existe_secuencial(&numeros, 2));
    println!("¿Existe el 100? {}", existe_secuencial(&numeros, 100));

    // Contar ocurrencias
    println!(
        "\nCuántas veces aparece el 2: {} veces",
        contar_ocurrencias(&numeros, 2)
    );
    println!(
        "Cuántas veces aparece el 100: {} veces",
        contar_ocurrencias(&numeros, 100)
    );

    // =========== EJEMPLO 5: Comparación de Rendimiento ===========
    println!("\n--- EJEMPLO 5: Comparación de Eficiencia ---");
    println!("(Contando el número de comparaciones)\n");

    // Arreglo grande ordenado
    let mut arreglo_grande: Vec<i32> = (1..=1000).collect();

    println!("Arreglo de {} elementos ordenados\n", arreglo_grande.len());

    // Casos a buscar
    let casos = vec![
        ("Primer elemento (1)", 1),
        ("Elemento del medio (500)", 500),
        ("Último elemento (1000)", 1000),
        ("Elemento no existe (999999)", 999999),
    ];

    println!("Comparación de comparaciones realizadas:\n");
    println!(
        "{:<30} | {:^20} | {:^20}",
        "Caso", "Búsqueda Secuencial", "Búsqueda Binaria"
    );
    println!("{:-<30}-+-{:-^20}-+-{:-^20}", "", "", "");

    for (descripcion, objetivo) in casos {
        // Búsqueda secuencial
        let result_sec = busqueda_secuencial_contando(&arreglo_grande, objetivo);

        // Búsqueda binaria
        let result_bin = busqueda_binaria_contando(&arreglo_grande, objetivo);

        println!(
            "{:<30} | {:^20} | {:^20}",
            descripcion, result_sec.comparaciones, result_bin.comparaciones
        );
    }

    // =========== EJEMPLO 6: Análisis Teórico ===========
    println!("\n--- EJEMPLO 6: Análisis de Complejidad ---\n");

    let tamaños = vec![10, 100, 1000, 10000, 100000];

    println!("Comparaciones esperadas en el peor caso:\n");
    println!(
        "{:<15} | {:^20} | {:^20}",
        "Tamaño (n)", "Búsqueda Secuencial", "Búsqueda Binaria"
    );
    println!("{:-<15}-+-{:-^20}-+-{:-^20}", "", "", "");

    for n in tamaños {
        // O(n) para búsqueda secuencial
        let secuencial = n;

        // O(log n) para búsqueda binaria (redondeado hacia arriba)
        let binaria = (n as f64).log2().ceil() as i32;

        println!("{:<15} | {:^20} | {:^20}", n, secuencial, binaria);
    }

    // =========== EJEMPLO 7: Caso Educativo ---
    println!("\n--- EJEMPLO 7: Simulación de Búsqueda de un Nombre ---\n");

    // Lista de nombres ordenados alfabéticamente
    let nombres = vec![
        "Alice", "Bob", "Carlos", "Diana", "Eva", "Felipe", "Gabriela", "Hector", "Iris", "Juan",
        "Karen", "Luis", "Mariana", "Nicolas", "Olivia", "Pablo", "Quique", "Rosa",
    ];

    println!("Directorio telefónico con {} personas\n", nombres.len());

    // Buscamos algunos nombres
    let a_buscar = vec!["Mariana", "Zeus", "Alice"];

    for nombre in a_buscar {
        match nombres.iter().position(|&n| n == nombre) {
            Some(indice) => {
                println!("✓ {} encontrado en posición {}", nombre, indice + 1);
            }
            None => {
                println!("✗ {} no está en el directorio", nombre);
            }
        }
    }

    println!("\n===== FIN DEL EJEMPLO =====");
}

// ============================================================================
// RESUMEN: CUÁNDO USAR CADA ALGORITMO
// ============================================================================
//
// BÚSQUEDA SECUENCIAL (Linear Search):
// - Úsala cuando: El arreglo está DESORDENADO
// - Úsala cuando: El arreglo es PEQUEÑO
// - Úsala cuando: Solo buscas UNA VEZ
// - Complejidad: O(n)
// - Ventaja: Simple y NO requiere datos ordenados
//
// BÚSQUEDA BINARIA (Binary Search):
// - Úsala cuando: El arreglo está ORDENADO
// - Úsala cuando: El arreglo es GRANDE
// - Úsala cuando: Buscas MÚLTIPLES VECES
// - Complejidad: O(log n)
// - Ventaja: MÁS RÁPIDA para datos ordenados
//
// ============================================================================
// EJEMPLO VISUAL: ¿Cómo funciona la búsqueda binaria?
// ============================================================================
// Buscando 13 en: [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
//                           índices: 0  1  2  3  4  5   6   7   8   9
//
// Paso 1: izquierda=0, derecha=9, medio=4
//         [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
//                       ↑
//         elemento[4]=9, comparamos: 9 < 13, ir a la derecha
//
// Paso 2: izquierda=5, derecha=9, medio=7
//         [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
//                             ↑
//         elemento[7]=15, comparamos: 15 > 13, ir a la izquierda
//
// Paso 3: izquierda=5, derecha=6, medio=5
//         [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
//                        ↑
//         elemento[5]=11, comparamos: 11 < 13, ir a la derecha
//
// Paso 4: izquierda=6, derecha=6, medio=6
//         [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
//                            ↑
//         elemento[6]=13, ¡ENCONTRADO!
//
// Total: 4 comparaciones (vs 7 con búsqueda secuencial)
//
// ============================================================================
