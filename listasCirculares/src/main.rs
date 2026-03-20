// ============================================================================
// LISTAS CIRCULARES EN RUST - DOCUMENTACIÓN EDUCATIVA
// ============================================================================
// Este programa implementa dos tipos de listas circulares:
// 1. Lista Circular Simple (un solo puntero siguiente)
// 2. Lista Circular Doble (punteros siguiente y anterior)

use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// PARTE 1: LISTA CIRCULAR SIMPLE
// ============================================================================

/// Estructura que representa un nodo en una lista circular simple.
///
/// En Rust, en lugar de usar punteros crudos como en C++, usamos:
/// - `Rc<RefCell<T>>`: permite múltiples referencias al mismo dato
/// - `RefCell`: permite mutabilidad en tiempo de ejecución
///
/// # Campos
/// - `dato`: el valor almacenado en el nodo (tipo i32)
/// - `siguiente`: referencia al siguiente nodo (Option envuelve un posible None)
#[derive(Clone)]
struct NodoCircularSimple {
    dato: i32,
    siguiente: Option<Rc<RefCell<NodoCircularSimple>>>,
}

impl NodoCircularSimple {
    /// Constructor para crear un nuevo nodo.
    fn nuevo(valor: i32) -> Self {
        NodoCircularSimple {
            dato: valor,
            siguiente: None,
        }
    }
}

/// Estructura principal para la Lista Circular Simple.
struct ListaCircularSimple {
    ultimo: Option<Rc<RefCell<NodoCircularSimple>>>,
    cantidad: usize,
}

impl ListaCircularSimple {
    /// Constructor para crear una lista vacía.
    fn nuevo() -> Self {
        ListaCircularSimple {
            ultimo: None,
            cantidad: 0,
        }
    }

    /// Inserta un nuevo valor al final de la lista.
    fn insertar(&mut self, valor: i32) {
        let nuevo_nodo = Rc::new(RefCell::new(NodoCircularSimple::nuevo(valor)));

        match &self.ultimo {
            None => {
                nuevo_nodo.borrow_mut().siguiente = Some(Rc::clone(&nuevo_nodo));
                self.ultimo = Some(nuevo_nodo);
            }
            Some(ultimo_actual) => {
                let primer_nodo = Rc::clone(&ultimo_actual.borrow().siguiente.as_ref().unwrap());
                nuevo_nodo.borrow_mut().siguiente = Some(primer_nodo);
                ultimo_actual.borrow_mut().siguiente = Some(Rc::clone(&nuevo_nodo));
                self.ultimo = Some(nuevo_nodo);
            }
        }

        self.cantidad += 1;
        println!("Insertado: {}", valor);
    }

    /// Inserta un valor al inicio de la lista.
    fn insertar_inicio(&mut self, valor: i32) {
        let nuevo_nodo = Rc::new(RefCell::new(NodoCircularSimple::nuevo(valor)));

        match &self.ultimo {
            None => {
                nuevo_nodo.borrow_mut().siguiente = Some(Rc::clone(&nuevo_nodo));
                self.ultimo = Some(nuevo_nodo);
            }
            Some(ultimo_actual) => {
                let primer_nodo = Rc::clone(&ultimo_actual.borrow().siguiente.as_ref().unwrap());
                nuevo_nodo.borrow_mut().siguiente = Some(primer_nodo);
                ultimo_actual.borrow_mut().siguiente = Some(Rc::clone(&nuevo_nodo));
            }
        }

        self.cantidad += 1;
        println!("Insertado al inicio: {}", valor);
    }

    /// Muestra todos los elementos de la lista.
    fn mostrar(&self) {
        match &self.ultimo {
            None => println!("Lista circular vacía"),
            Some(ultimo) => {
                print!("Lista Circular Simple: ");

                let primer_nodo = Rc::clone(&ultimo.borrow().siguiente.as_ref().unwrap());
                let mut actual = Rc::clone(&primer_nodo);

                loop {
                    print!("{}", actual.borrow().dato);

                    let siguiente = actual
                        .borrow()
                        .siguiente
                        .as_ref()
                        .map(|n| Rc::clone(n))
                        .unwrap();

                    if Rc::ptr_eq(&siguiente, &primer_nodo) {
                        break;
                    }

                    print!(" -> ");
                    actual = siguiente;
                }

                println!(" -> [Vuelta al inicio]");
            }
        }
    }

    /// Muestra la estructura detallada con punteros y posiciones.
    fn mostrar_detallado(&self) {
        match &self.ultimo {
            None => println!("Lista circular vacía"),
            Some(ultimo) => {
                println!("\n=== ESTRUCTURA DETALLADA ===");

                let primer_nodo = Rc::clone(&ultimo.borrow().siguiente.as_ref().unwrap());
                let mut actual = Rc::clone(&primer_nodo);
                let mut posicion = 0;

                loop {
                    let dato_actual = actual.borrow().dato;
                    let dato_siguiente = actual
                        .borrow()
                        .siguiente
                        .as_ref()
                        .map(|n| n.borrow().dato)
                        .unwrap_or(-1);

                    println!(
                        "Posición {}: [Dato: {} | Siguiente: {}]",
                        posicion, dato_actual, dato_siguiente
                    );

                    let siguiente = actual
                        .borrow()
                        .siguiente
                        .as_ref()
                        .map(|n| Rc::clone(n))
                        .unwrap();

                    if Rc::ptr_eq(&siguiente, &primer_nodo) {
                        break;
                    }

                    actual = siguiente;
                    posicion += 1;
                }

                let dato_ultimo = ultimo.borrow().dato;
                let dato_primer = primer_nodo.borrow().dato;
                println!(
                    "Último nodo: {} -> Primer nodo: {}",
                    dato_ultimo, dato_primer
                );
            }
        }
    }

    /// Busca un valor en la lista.
    fn buscar(&self, valor: i32) -> bool {
        match &self.ultimo {
            None => false,
            Some(ultimo) => {
                let primer_nodo = Rc::clone(&ultimo.borrow().siguiente.as_ref().unwrap());
                let mut actual = Rc::clone(&primer_nodo);

                loop {
                    if actual.borrow().dato == valor {
                        return true;
                    }

                    let siguiente = actual
                        .borrow()
                        .siguiente
                        .as_ref()
                        .map(|n| Rc::clone(n))
                        .unwrap();

                    if Rc::ptr_eq(&siguiente, &primer_nodo) {
                        break;
                    }

                    actual = siguiente;
                }

                false
            }
        }
    }

    /// Elimina la primera ocurrencia de un valor.
    fn eliminar(&mut self, valor: i32) {
        match &self.ultimo {
            None => println!("Lista vacía"),
            Some(ultimo) => {
                if self.cantidad == 1 && ultimo.borrow().dato == valor {
                    self.ultimo = None;
                    self.cantidad = 0;
                    println!("Eliminado: {} (único elemento)", valor);
                    return;
                }

                let primer_nodo = Rc::clone(&ultimo.borrow().siguiente.as_ref().unwrap());
                let mut anterior = Rc::clone(ultimo);
                let mut actual = Rc::clone(&primer_nodo);

                loop {
                    if actual.borrow().dato == valor {
                        let siguiente = actual
                            .borrow()
                            .siguiente
                            .as_ref()
                            .map(|n| Rc::clone(n))
                            .unwrap();
                        anterior.borrow_mut().siguiente = Some(siguiente);

                        if Rc::ptr_eq(&actual, ultimo) {
                            self.ultimo = Some(anterior);
                        }

                        self.cantidad -= 1;
                        println!("Eliminado: {}", valor);
                        return;
                    }

                    let siguiente = actual
                        .borrow()
                        .siguiente
                        .as_ref()
                        .map(|n| Rc::clone(n))
                        .unwrap();

                    if Rc::ptr_eq(&siguiente, &primer_nodo) {
                        break;
                    }

                    anterior = actual;
                    actual = siguiente;
                }

                println!("Elemento {} no encontrado", valor);
            }
        }
    }

    /// Recorre la lista circular n vueltas.
    fn recorrer_circular(&self, vueltas: usize) {
        match &self.ultimo {
            None => println!("Lista vacía"),
            Some(ultimo) => {
                let primer_nodo = Rc::clone(&ultimo.borrow().siguiente.as_ref().unwrap());
                let mut actual = Rc::clone(&primer_nodo);

                print!("Recorriendo {} vueltas: ", vueltas);

                for i in 0..(vueltas * self.cantidad) {
                    print!("{}", actual.borrow().dato);

                    let siguiente = actual
                        .borrow()
                        .siguiente
                        .as_ref()
                        .map(|n| Rc::clone(n))
                        .unwrap();

                    if i < (vueltas * self.cantidad - 1) {
                        print!(" -> ");
                    }

                    actual = siguiente;
                }

                println!(" [...]");
            }
        }
    }
}

// ============================================================================
// PARTE 2: LISTA CIRCULAR DOBLE
// ============================================================================

/// Nodo para lista circular doble con punteros anterior y siguiente.
#[derive(Clone)]
struct NodoCircularDoble {
    dato: i32,
    siguiente: Option<Rc<RefCell<NodoCircularDoble>>>,
    anterior: Option<Rc<RefCell<NodoCircularDoble>>>,
}

impl NodoCircularDoble {
    /// Constructor para un nuevo nodo doble.
    fn nuevo(valor: i32) -> Self {
        NodoCircularDoble {
            dato: valor,
            siguiente: None,
            anterior: None,
        }
    }
}

/// Lista Circular Doble - permite navegar en ambas direcciones.
struct ListaCircularDoble {
    cabeza: Option<Rc<RefCell<NodoCircularDoble>>>,
    cantidad: usize,
}

impl ListaCircularDoble {
    /// Constructor para una lista circular doble vacía.
    fn nuevo() -> Self {
        ListaCircularDoble {
            cabeza: None,
            cantidad: 0,
        }
    }

    /// Inserta un nuevo valor al final.
    fn insertar(&mut self, valor: i32) {
        let nuevo_nodo = Rc::new(RefCell::new(NodoCircularDoble::nuevo(valor)));

        match &self.cabeza {
            None => {
                nuevo_nodo.borrow_mut().siguiente = Some(Rc::clone(&nuevo_nodo));
                nuevo_nodo.borrow_mut().anterior = Some(Rc::clone(&nuevo_nodo));
                self.cabeza = Some(nuevo_nodo);
            }
            Some(cabeza_actual) => {
                let ultimo = Rc::clone(&cabeza_actual.borrow().anterior.as_ref().unwrap());

                nuevo_nodo.borrow_mut().siguiente = Some(Rc::clone(cabeza_actual));
                nuevo_nodo.borrow_mut().anterior = Some(Rc::clone(&ultimo));

                ultimo.borrow_mut().siguiente = Some(Rc::clone(&nuevo_nodo));
                cabeza_actual.borrow_mut().anterior = Some(nuevo_nodo);
            }
        }

        self.cantidad += 1;
        println!("Insertado: {}", valor);
    }

    /// Muestra la lista en dirección hacia adelante.
    fn mostrar_adelante(&self) {
        match &self.cabeza {
            None => println!("Lista circular doble vacía"),
            Some(cabeza) => {
                print!("Lista Circular Doble (Adelante): ");

                let mut actual = Rc::clone(cabeza);

                loop {
                    print!("{}", actual.borrow().dato);

                    let siguiente = actual
                        .borrow()
                        .siguiente
                        .as_ref()
                        .map(|n| Rc::clone(n))
                        .unwrap();

                    if !Rc::ptr_eq(&siguiente, cabeza) {
                        print!(" <-> ");
                    }

                    actual = siguiente;

                    if Rc::ptr_eq(&actual, cabeza) {
                        break;
                    }
                }

                println!(" -> [Vuelta al inicio]");
            }
        }
    }

    /// Muestra la lista en dirección hacia atrás.
    fn mostrar_atras(&self) {
        match &self.cabeza {
            None => println!("Lista circular doble vacía"),
            Some(cabeza) => {
                print!("Lista Circular Doble (Atrás): ");

                let mut actual = Rc::clone(&cabeza.borrow().anterior.as_ref().unwrap());
                let inicio = Rc::clone(&actual);

                loop {
                    print!("{}", actual.borrow().dato);

                    let anterior = actual
                        .borrow()
                        .anterior
                        .as_ref()
                        .map(|n| Rc::clone(n))
                        .unwrap();

                    if !Rc::ptr_eq(&anterior, &inicio) {
                        print!(" <-> ");
                    }

                    actual = anterior;

                    if Rc::ptr_eq(&actual, &inicio) {
                        break;
                    }
                }

                println!(" -> [Vuelta al final]");
            }
        }
    }

    /// Muestra la estructura completa con punteros anterior y siguiente.
    fn mostrar_estructura(&self) {
        match &self.cabeza {
            None => println!("Lista vacía"),
            Some(cabeza) => {
                println!("\n=== ESTRUCTURA COMPLETA ===");

                let mut actual = Rc::clone(cabeza);
                let mut posicion = 0;

                loop {
                    let datos_anterior = actual.borrow().anterior.as_ref().map(|n| n.borrow().dato);
                    let dato_actual = actual.borrow().dato;
                    let datos_siguiente =
                        actual.borrow().siguiente.as_ref().map(|n| n.borrow().dato);

                    println!(
                        "Nodo {}: [Anterior: {} | Dato: {} | Siguiente: {}]",
                        posicion,
                        datos_anterior.map_or("cabeza".to_string(), |d| d.to_string()),
                        dato_actual,
                        datos_siguiente.map_or("cabeza".to_string(), |d| d.to_string())
                    );

                    let siguiente = actual
                        .borrow()
                        .siguiente
                        .as_ref()
                        .map(|n| Rc::clone(n))
                        .unwrap();

                    if Rc::ptr_eq(&siguiente, cabeza) {
                        break;
                    }

                    actual = siguiente;
                    posicion += 1;
                }
            }
        }
    }

    /// Busca un valor en la lista.
    fn buscar(&self, valor: i32) -> bool {
        match &self.cabeza {
            None => false,
            Some(cabeza) => {
                let mut actual = Rc::clone(cabeza);

                loop {
                    if actual.borrow().dato == valor {
                        return true;
                    }

                    let siguiente = actual
                        .borrow()
                        .siguiente
                        .as_ref()
                        .map(|n| Rc::clone(n))
                        .unwrap();

                    if Rc::ptr_eq(&siguiente, cabeza) {
                        break;
                    }

                    actual = siguiente;
                }

                false
            }
        }
    }

    /// Elimina la primera ocurrencia de un valor.
    fn eliminar(&mut self, valor: i32) {
        match &self.cabeza {
            None => println!("Lista vacía"),
            Some(_) => {
                let cabeza = Rc::clone(self.cabeza.as_ref().unwrap());
                let mut actual = Rc::clone(&cabeza);

                loop {
                    if actual.borrow().dato == valor {
                        if self.cantidad == 1 {
                            self.cabeza = None;
                            self.cantidad = 0;
                            println!("Eliminado: {} (único elemento)", valor);
                            return;
                        }

                        let anterior = actual
                            .borrow()
                            .anterior
                            .as_ref()
                            .map(|n| Rc::clone(n))
                            .unwrap();
                        let siguiente = actual
                            .borrow()
                            .siguiente
                            .as_ref()
                            .map(|n| Rc::clone(n))
                            .unwrap();

                        anterior.borrow_mut().siguiente = Some(Rc::clone(&siguiente));
                        siguiente.borrow_mut().anterior = Some(anterior);

                        if Rc::ptr_eq(&actual, &cabeza) {
                            self.cabeza = Some(siguiente);
                        }

                        self.cantidad -= 1;
                        println!("Eliminado: {}", valor);
                        return;
                    }

                    let siguiente = actual
                        .borrow()
                        .siguiente
                        .as_ref()
                        .map(|n| Rc::clone(n))
                        .unwrap();

                    if Rc::ptr_eq(&siguiente, &cabeza) {
                        break;
                    }

                    actual = siguiente;
                }

                println!("Elemento {} no encontrado", valor);
            }
        }
    }

    /// Rota la lista moviendo la cabeza.
    fn rotar(&mut self, posiciones: usize) {
        if let Some(ref cabeza) = self.cabeza {
            let mut actual = Rc::clone(cabeza);

            for _ in 0..posiciones {
                let siguiente = actual
                    .borrow()
                    .siguiente
                    .as_ref()
                    .map(|n| Rc::clone(n))
                    .unwrap();
                actual = siguiente;
            }

            self.cabeza = Some(actual);
            println!("Lista rotada {} posiciones", posiciones);
        }
    }
}

/// Función que demuestra el uso de ambas listas circulares.
fn demostrar_listas_circulares() {
    println!("=== DEMOSTRACIÓN LISTA CIRCULAR SIMPLE ===");

    let mut lista_simple = ListaCircularSimple::nuevo();

    lista_simple.insertar(10);
    lista_simple.insertar(20);
    lista_simple.insertar(30);
    lista_simple.insertar_inicio(5);

    lista_simple.mostrar();
    lista_simple.mostrar_detallado();

    println!(
        "\nBuscando 20: {}",
        if lista_simple.buscar(20) {
            "Encontrado"
        } else {
            "No encontrado"
        }
    );
    println!(
        "Buscando 99: {}",
        if lista_simple.buscar(99) {
            "Encontrado"
        } else {
            "No encontrado"
        }
    );

    lista_simple.recorrer_circular(3);
    lista_simple.eliminar(20);
    lista_simple.mostrar();

    println!("\n{}", "=".repeat(50));
    println!("\n=== DEMOSTRACIÓN LISTA CIRCULAR DOBLE ===");

    let mut lista_doble = ListaCircularDoble::nuevo();

    lista_doble.insertar(100);
    lista_doble.insertar(200);
    lista_doble.insertar(300);
    lista_doble.insertar(400);

    lista_doble.mostrar_adelante();
    lista_doble.mostrar_atras();
    lista_doble.mostrar_estructura();

    println!(
        "\nBuscando 300: {}",
        if lista_doble.buscar(300) {
            "Encontrado"
        } else {
            "No encontrado"
        }
    );

    lista_doble.rotar(2);
    lista_doble.mostrar_adelante();

    lista_doble.eliminar(300);
    lista_doble.mostrar_adelante();
}

fn main() {
    demostrar_listas_circulares();
}
