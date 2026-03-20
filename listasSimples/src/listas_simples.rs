// ============================================================
// lista_simple.rs - Implementación de Lista Simplemente Ligada
// ============================================================

// Box<T> es un puntero en el heap, equivale a un puntero dinámico en C++
// Option<Box<Nodo>> significa: puede haber un Nodo o puede ser None (null)
pub struct Nodo {
    pub valor: i32,
    pub siguiente: Option<Box<Nodo>>, // puntero al siguiente nodo (o None si es el último)
}

impl Nodo {
    // Crea un nuevo nodo con valor dado y sin siguiente nodo
    pub fn new(valor: i32) -> Self {
        Nodo {
            valor,
            siguiente: None,
        }
    }
}

// Struct principal de la lista
pub struct ListaSimpleLigada {
    cabeza: Option<Box<Nodo>>, // primer nodo de la lista (o None si está vacía)
    tamanio: usize,            // cantidad de elementos en la lista
}

impl ListaSimpleLigada {
    // Constructor: crea una lista vacía
    pub fn new() -> Self {
        ListaSimpleLigada {
            cabeza: None,
            tamanio: 0,
        }
    }

    // Retorna true si la lista no tiene elementos
    pub fn esta_vacia(&self) -> bool {
        self.cabeza.is_none()
    }

    // Retorna la cantidad de nodos en la lista
    pub fn obtener_tamanio(&self) -> usize {
        self.tamanio
    }

    // Inserta un nuevo nodo al inicio de la lista
    // El nuevo nodo apunta al antiguo primer nodo
    pub fn insertar_inicio(&mut self, valor: i32) {
        let mut nuevo_nodo = Box::new(Nodo::new(valor));

        // El siguiente del nuevo nodo es la cabeza actual
        // take() extrae el valor de Option dejando None en su lugar
        nuevo_nodo.siguiente = self.cabeza.take();

        // El nuevo nodo pasa a ser la cabeza
        self.cabeza = Some(nuevo_nodo);
        self.tamanio += 1;

        println!("Insertado {} al inicio", valor);
    }

    // Inserta un nuevo nodo al final de la lista
    // Recorre toda la lista hasta encontrar el último nodo
    pub fn insertar_final(&mut self, valor: i32) {
        let nuevo_nodo = Box::new(Nodo::new(valor));

        // as_mut() convierte Option<Box<Nodo>> a Option<&mut Box<Nodo>>
        // para poder modificar el nodo sin moverlo
        match self.cabeza.as_mut() {
            // Si la lista está vacía, el nuevo nodo es la cabeza
            None => self.cabeza = Some(nuevo_nodo),

            // Si hay nodos, recorrer hasta el último
            Some(mut actual) => {
                // Mientras el nodo actual tenga siguiente, avanzar
                while actual.siguiente.is_some() {
                    // as_mut().unwrap() nos da referencia mutable al siguiente nodo
                    actual = actual.siguiente.as_mut().unwrap();
                }
                // Al salir del while, actual es el último nodo
                actual.siguiente = Some(nuevo_nodo);
            }
        }

        self.tamanio += 1;
        println!("Insertado {} al final", valor);
    }

    // Inserta un nodo en una posición específica (0 = inicio)
    // Si la posición supera el tamaño, inserta al final
    pub fn insertar_en_posicion(&mut self, valor: i32, posicion: usize) {
        // Si posición es 0 o la lista está vacía, insertar al inicio
        if posicion == 0 || self.esta_vacia() {
            self.insertar_inicio(valor);
            return;
        }

        // Si posición es mayor o igual al tamaño, insertar al final
        if posicion >= self.tamanio {
            self.insertar_final(valor);
            return;
        }

        // Recorrer hasta el nodo ANTERIOR a la posición deseada
        let mut actual = self.cabeza.as_mut().unwrap();
        for _ in 0..posicion - 1 {
            actual = actual.siguiente.as_mut().unwrap();
        }

        // Crear nuevo nodo y enlazarlo
        let mut nuevo_nodo = Box::new(Nodo::new(valor));

        // El nuevo nodo apunta al nodo que estaba en esa posición
        nuevo_nodo.siguiente = actual.siguiente.take();

        // El nodo anterior apunta al nuevo nodo
        actual.siguiente = Some(nuevo_nodo);

        self.tamanio += 1;
        println!("Insertado {} en posición {}", valor, posicion);
    }

    // Busca un valor en la lista e imprime si se encontró y en qué posición
    pub fn buscar(&self, valor: i32) {
        let mut actual = self.cabeza.as_ref(); // referencia inmutable al primer nodo
        let mut posicion = 0;

        // Recorrer la lista mientras haya nodos
        while let Some(nodo) = actual {
            if nodo.valor == valor {
                println!("Valor {} encontrado en posición {}", valor, posicion);
                return;
            }
            // Avanzar al siguiente nodo
            actual = nodo.siguiente.as_ref();
            posicion += 1;
        }

        println!("Valor {} no encontrado en la lista", valor);
    }

    // Elimina el primer nodo que tenga el valor indicado
    pub fn eliminar(&mut self, valor: i32) {
        // Caso especial: eliminar la cabeza
        if let Some(ref nodo) = self.cabeza {
            if nodo.valor == valor {
                // La nueva cabeza es el siguiente del nodo eliminado
                self.cabeza = self.cabeza.take().unwrap().siguiente;
                self.tamanio -= 1;
                println!("Eliminado valor {}", valor);
                return;
            }
        }

        // Buscar el nodo anterior al que queremos eliminar
        let mut actual = self.cabeza.as_mut();
        while let Some(nodo) = actual {
            // Verificar si el siguiente nodo tiene el valor buscado
            if let Some(ref siguiente) = nodo.siguiente {
                if siguiente.valor == valor {
                    // Saltar el nodo a eliminar enlazando con el que sigue
                    nodo.siguiente = nodo.siguiente.take().unwrap().siguiente;
                    self.tamanio -= 1;
                    println!("Eliminado valor {}", valor);
                    return;
                }
            }
            actual = nodo.siguiente.as_mut();
        }

        println!("Valor {} no encontrado para eliminar", valor);
    }

    // Elimina el nodo en la posición indicada (0 = primero)
    pub fn eliminar_en_posicion(&mut self, posicion: usize) {
        if self.esta_vacia() {
            println!("La lista está vacía");
            return;
        }

        if posicion >= self.tamanio {
            println!("Posición {} fuera de rango", posicion);
            return;
        }

        // Caso especial: eliminar la cabeza (posición 0)
        if posicion == 0 {
            self.cabeza = self.cabeza.take().unwrap().siguiente;
            self.tamanio -= 1;
            println!("Eliminado nodo en posición {}", posicion);
            return;
        }

        // Recorrer hasta el nodo ANTERIOR a la posición a eliminar
        let mut actual = self.cabeza.as_mut().unwrap();
        for _ in 0..posicion - 1 {
            actual = actual.siguiente.as_mut().unwrap();
        }

        // Saltar el nodo en la posición indicada
        actual.siguiente = actual.siguiente.take().unwrap().siguiente;
        self.tamanio -= 1;
        println!("Eliminado nodo en posición {}", posicion);
    }

    // Muestra todos los valores de la lista en una línea
    pub fn mostrar(&self) {
        if self.esta_vacia() {
            println!("Lista vacía");
            return;
        }

        print!("Lista: ");
        let mut actual = self.cabeza.as_ref();

        while let Some(nodo) = actual {
            print!("{}", nodo.valor);

            // Si tiene siguiente, imprimir la flecha
            if nodo.siguiente.is_some() {
                print!(" -> ");
            }
            actual = nodo.siguiente.as_ref();
        }

        println!(" -> NULL");
    }

    // Muestra la lista con información detallada de cada nodo y su posición
    pub fn mostrar_detallado(&self) {
        if self.esta_vacia() {
            println!("Lista vacía");
            return;
        }

        println!("Estructura detallada:");
        let mut actual = self.cabeza.as_ref();
        let mut posicion = 0;

        while let Some(nodo) = actual {
            let siguiente = match &nodo.siguiente {
                Some(sig) => sig.valor.to_string(),
                None => "NULL".to_string(),
            };
            println!(
                "  [{}] Valor: {} | Siguiente: {}",
                posicion, nodo.valor, siguiente
            );
            actual = nodo.siguiente.as_ref();
            posicion += 1;
        }
    }
}

// Drop equivale al destructor de C++
// Se ejecuta automáticamente cuando la lista sale de scope
impl Drop for ListaSimpleLigada {
    fn drop(&mut self) {
        println!("\nLista destruida, memoria liberada.");
    }
}
