// ============================================================
// CONVERSIÓN DE C++ A RUST: Lista Doblemente Ligada
// ============================================================
//
// DIFERENCIAS CLAVE RESPECTO A C++:
//
// 1. NO HAY ARCHIVOS .h / .cpp SEPARADOS
//    En Rust todo vive en módulos dentro del mismo proyecto.
//    El archivo `claseDobleLigada.h` desaparece — la struct y su
//    implementación se definen en el mismo .rs.
//
// 2. GESTIÓN DE MEMORIA AUTOMÁTICA (sin new/delete)
//    Rust NO tiene punteros crudos en código seguro. En su lugar:
//      - Option<T>         → valor que puede o no existir (= nullptr en C++)
//      - Rc<RefCell<T>>    → necesario para listas doblemente ligadas porque
//                            un nodo necesita DOS dueños simultáneos
//                            (el nodo anterior Y el siguiente lo referencian),
//                            algo que viola la regla de "un solo dueño" de Rust.
//      - Rc (Reference Counted) maneja el conteo de referencias automáticamente.
//      - RefCell permite mutar el contenido aunque haya múltiples referencias.
//
// 3. SEGURIDAD GARANTIZADA EN COMPILACIÓN
//    El compilador impide dangling pointers, double-free y data races.
//    No hay riesgo de olvidar un `delete`.
// ============================================================

// `use` equivale a `#include` en C++
use std::cell::RefCell;
use std::rc::Rc; // Rc = Reference Counted → múltiples dueños compartidos // RefCell → mutabilidad interior con verificación en runtime

// ─────────────────────────────────────────────────────────────
// TYPE ALIAS: Enlace
// ─────────────────────────────────────────────────────────────
// En C++ un enlace sería simplemente `Nodo*`.
// En Rust necesitamos el triple envoltorio para una lista doblemente ligada:
//   Rc        → permite que varios nodos compartan la propiedad
//   RefCell   → permite modificar el nodo aunque esté compartido
//   Option    → puede ser None (= nullptr) o Some(nodo)
type Enlace = Rc<RefCell<Nodo>>;

// ─────────────────────────────────────────────────────────────
// STRUCT NODO
// ─────────────────────────────────────────────────────────────
// En C++:
//   struct Nodo {
//       int dato;
//       Nodo* siguiente;
//       Nodo* anterior;
//   };
struct Nodo {
    dato: i32,
    siguiente: Option<Enlace>, // Nodo* siguiente  → None es nullptr
    anterior: Option<Enlace>,  // Nodo* anterior   → None es nullptr
}

impl Nodo {
    // Constructor estático — equivalente a `new Nodo(dato)` en C++
    // Devuelve el nodo ya envuelto en Rc<RefCell<>> para uso directo
    fn nuevo(dato: i32) -> Enlace {
        Rc::new(RefCell::new(Nodo {
            dato,
            siguiente: None,
            anterior: None,
        }))
    }
}

// ─────────────────────────────────────────────────────────────
// STRUCT LISTA DOBLEMENTE LIGADA
// ─────────────────────────────────────────────────────────────
// En C++:
//   class ListaDoblementeLigada {
//       Nodo* cabeza;
//       Nodo* cola;
//   };
struct ListaDoblementeLigada {
    cabeza: Option<Enlace>,
    cola: Option<Enlace>,
}

// `impl` agrupa los métodos de la struct — equivale al cuerpo de la clase en C++
impl ListaDoblementeLigada {
    // Constructor — en C++ sería el constructor por defecto `ListaDoblementeLigada()`
    // La convención en Rust es llamarlo `new()` o `nueva()`
    fn nueva() -> Self {
        ListaDoblementeLigada {
            cabeza: None,
            cola: None,
        }
    }

    // ── insertarInicio ───────────────────────────────────────────
    // C++: void insertarInicio(int dato)
    // `&mut self` = el método modifica la lista (equivale a un método no-const)
    fn insertar_inicio(&mut self, dato: i32) {
        println!("Insertando {} al inicio", dato);
        let nuevo = Nodo::nuevo(dato);

        // `take()` extrae el valor de Option dejando None en su lugar
        // (necesario para no violar las reglas de propiedad de Rust)
        match self.cabeza.take() {
            None => {
                // Lista vacía: el nuevo nodo es cabeza Y cola
                self.cola = Some(Rc::clone(&nuevo));
                self.cabeza = Some(nuevo);
            }
            Some(cabeza_actual) => {
                // C++: nuevo->siguiente = cabeza;
                //       cabeza->anterior = nuevo;
                nuevo.borrow_mut().siguiente = Some(Rc::clone(&cabeza_actual));
                cabeza_actual.borrow_mut().anterior = Some(Rc::clone(&nuevo));
                self.cabeza = Some(nuevo);
            }
        }
    }

    // ── insertarFinal ────────────────────────────────────────────
    // C++: void insertarFinal(int dato)
    fn insertar_final(&mut self, dato: i32) {
        println!("Insertando {} al final", dato);
        let nuevo = Nodo::nuevo(dato);

        match self.cola.take() {
            None => {
                self.cabeza = Some(Rc::clone(&nuevo));
                self.cola = Some(nuevo);
            }
            Some(cola_actual) => {
                // C++: cola->siguiente = nuevo;
                //       nuevo->anterior = cola;
                cola_actual.borrow_mut().siguiente = Some(Rc::clone(&nuevo));
                nuevo.borrow_mut().anterior = Some(Rc::clone(&cola_actual));
                self.cola = Some(nuevo);
            }
        }
    }

    // ── mostrarAdelante ──────────────────────────────────────────
    // C++: void mostrarAdelante()
    // `&self` = solo lectura, no modifica la lista (equivale a método const)
    fn mostrar_adelante(&self) {
        print!("Lista (adelante): ");
        // `.clone()` sobre un Rc solo incrementa el contador de referencias,
        // NO copia los datos — es barato y seguro
        let mut actual = self.cabeza.clone();

        // `while let` es el patrón idiomático de Rust para recorrer Option
        // equivale al `while (actual != nullptr)` de C++
        while let Some(nodo) = actual {
            // `borrow()` = acceso de solo lectura al interior del RefCell
            // equivale al operador `->` de C++
            print!("{} ", nodo.borrow().dato);
            actual = nodo.borrow().siguiente.clone();
        }
        println!();
    }

    // ── mostrarAtras ─────────────────────────────────────────────
    // C++: void mostrarAtras()
    // Recorre cola → cabeza usando los punteros `anterior`
    fn mostrar_atras(&self) {
        print!("Lista (atrás):    ");
        let mut actual = self.cola.clone();

        while let Some(nodo) = actual {
            print!("{} ", nodo.borrow().dato);
            actual = nodo.borrow().anterior.clone();
        }
        println!();
    }

    // ── buscar ───────────────────────────────────────────────────
    // C++: bool buscar(int dato)
    // Retorna bool igual que en C++
    fn buscar(&self, dato: i32) -> bool {
        let mut actual = self.cabeza.clone();

        while let Some(nodo) = actual {
            if nodo.borrow().dato == dato {
                return true;
            }
            actual = nodo.borrow().siguiente.clone();
        }
        false // En Rust la última expresión sin `;` es el valor de retorno
    }

    // ── eliminar ─────────────────────────────────────────────────
    // C++: void eliminar(int dato)
    //
    // DIFERENCIA CLAVE: En C++ hacías `delete nodo` manualmente.
    // En Rust, cuando el último Rc que apunta al nodo se destruye,
    // el Drop trait libera la memoria AUTOMÁTICAMENTE — cero fugas.
    fn eliminar(&mut self, dato: i32) {
        let mut actual = self.cabeza.clone();

        while let Some(nodo) = actual {
            if nodo.borrow().dato == dato {
                let anterior = nodo.borrow().anterior.clone();
                let siguiente = nodo.borrow().siguiente.clone();

                // Reconectar los vecinos (saltarse el nodo a eliminar)
                match &anterior {
                    Some(ant) => ant.borrow_mut().siguiente = siguiente.clone(),
                    None => self.cabeza = siguiente.clone(), // era la cabeza
                }
                match &siguiente {
                    Some(sig) => sig.borrow_mut().anterior = anterior.clone(),
                    None => self.cola = anterior.clone(), // era la cola
                }

                println!("Elemento {} eliminado.", dato);
                return;
            }
            actual = nodo.borrow().siguiente.clone();
        }

        println!("Elemento {} no encontrado.", dato);
    }
}

// ─────────────────────────────────────────────────────────────
// FUNCIÓN PRINCIPAL
// ─────────────────────────────────────────────────────────────
// C++: int main()
// Rust: fn main()  — no retorna int; los errores se manejan con Result o panic!
fn main() {
    // C++: ListaDoblementeLigada lista;
    // Rust: el constructor debe llamarse explícitamente
    // `mut` es obligatorio porque la lista será modificada
    let mut lista = ListaDoblementeLigada::nueva();

    println!("=== EJEMPLO 1: LISTA DOBLEMENTE LIGADA BÁSICA ===\n");

    lista.insertar_inicio(10);
    lista.insertar_inicio(20);
    lista.insertar_final(30);
    lista.insertar_final(40);

    println!();

    lista.mostrar_adelante();
    lista.mostrar_atras();

    println!();

    // C++: lista.buscar(20) ? "Sí" : "No"
    // Rust: el if-else es una expresión que produce un valor directamente
    println!(
        "¿Está 20 en la lista? {}",
        if lista.buscar(20) { "Sí" } else { "No" }
    );
    println!(
        "¿Está 50 en la lista? {}",
        if lista.buscar(50) { "Sí" } else { "No" }
    );

    println!();

    lista.eliminar(20);
    lista.mostrar_adelante();

    lista.eliminar(40);
    lista.mostrar_adelante();

    lista.eliminar(99); // No existe — imprime mensaje de error

    // No se necesita `return 0;` — main() en Rust termina sin valor de retorno.
}
