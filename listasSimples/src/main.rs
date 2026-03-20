// ============================================================
// main.rs - Punto de entrada y demostración de la lista
// ============================================================

mod listas_simples; // declara el módulo (busca lista_simple.rs en src/)
use listas_simples::ListaSimpleLigada; // importa la struct para usarla aquí

// Función que demuestra todas las operaciones de la lista
fn demostrar_operaciones() {
    // Crear una nueva lista vacía
    let mut lista = ListaSimpleLigada::new();

    println!("=== DEMOSTRACIÓN LISTA SIMPLEMENTE LIGADA ===\n");

    // --- Insertar elementos ---
    println!("--- Insertando elementos ---");
    lista.insertar_inicio(30); // lista: 30
    lista.insertar_inicio(20); // lista: 20 -> 30
    lista.insertar_inicio(10); // lista: 10 -> 20 -> 30
    lista.insertar_final(40); // lista: 10 -> 20 -> 30 -> 40
    lista.insertar_final(50); // lista: 10 -> 20 -> 30 -> 40 -> 50
    lista.insertar_en_posicion(25, 2); // lista: 10 -> 20 -> 25 -> 30 -> 40 -> 50
    lista.insertar_en_posicion(5, 0); // al inicio
    lista.insertar_en_posicion(60, 7); // al final

    println!();
    lista.mostrar();
    println!("Tamaño de la lista: {}\n", lista.obtener_tamanio());

    // --- Estructura detallada ---
    println!("--- Estructura detallada ---");
    lista.mostrar_detallado();
    println!();

    // --- Buscar elementos ---
    println!("--- Buscando elementos ---");
    lista.buscar(25); // existe en la lista
    lista.buscar(100); // no existe
    println!();

    // --- Eliminar elementos ---
    println!("--- Eliminando elementos ---");

    lista.eliminar(25); // valor en el medio
    lista.mostrar();

    lista.eliminar(5); // primer elemento
    lista.mostrar();

    lista.eliminar_en_posicion(0); // eliminar por posición (inicio)
    lista.mostrar();

    lista.eliminar_en_posicion(2); // eliminar por posición (medio)
    lista.mostrar();

    println!("\nTamaño final: {}", lista.obtener_tamanio());
    println!(
        "¿Lista vacía? {}",
        if lista.esta_vacia() { "Sí" } else { "No" }
    );

    // Al salir de esta función, lista sale de scope
    // y el Drop se ejecuta automáticamente
}

fn main() {
    demostrar_operaciones();
}
