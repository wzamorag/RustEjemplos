#[derive(Debug)]
struct Nodo {
    valor: i32,
    izquierda: Option<Box<Nodo>>,
    derecha: Option<Box<Nodo>>,
}
//implementando los metodos
impl Nodo {
    //constructor
    fn nuevo(valor: i32) -> Self {
        Nodo {
            valor,
            izquierda: None,
            derecha: None,
        }
    }
    // Método para mostrar el árbol de forma jerárquica
    fn mostrar(&self, nivel: usize) {
        // Primero procesamos el lado derecho (aparecerá arriba en la consola)
        if let Some(ref derecha) = self.derecha {
            derecha.mostrar(nivel + 1);
        }

        // Imprimimos el valor actual con espacios según el nivel
        println!("{}{}", "    ".repeat(nivel), self.valor);

        // Luego procesamos el lado izquierdo (aparecerá abajo en la consola)
        if let Some(ref izquierda) = self.izquierda {
            izquierda.mostrar(nivel + 1);
        }
    }
    //insertar
    fn insertar(&mut self, nuevo_valor: i32) {
        if nuevo_valor < self.valor {
            if let Some(ref mut izquierda) = self.izquierda {
                izquierda.insertar(nuevo_valor);
            } else {
                self.izquierda = Some(Box::new(Nodo::nuevo(nuevo_valor)));
            }
        } else {
            if let Some(ref mut derecha) = self.derecha {
                derecha.insertar(nuevo_valor);
            } else {
                self.derecha = Some(Box::new(Nodo::nuevo(nuevo_valor)));
            }
        }
    }
    fn insertar_con_pasos(&mut self, nuevo_valor: i32, profundidad: usize) {
        let prefijo = "  ".repeat(profundidad);
        println!(
            "{}-> Comparando {} con el nodo actual ({})",
            prefijo, nuevo_valor, self.valor
        );

        if nuevo_valor < self.valor {
            println!(
                "{}   {} < {}, yendo a la IZQUIERDA",
                prefijo, nuevo_valor, self.valor
            );
            if let Some(ref mut izquierda) = self.izquierda {
                izquierda.insertar_con_pasos(nuevo_valor, profundidad + 1);
            } else {
                println!(
                    "{}   [!] Espacio vacío. Insertando {} a la izquierda de {}",
                    prefijo, nuevo_valor, self.valor
                );
                self.izquierda = Some(Box::new(Nodo::nuevo(nuevo_valor)));
            }
        } else {
            println!(
                "{}   {} >= {}, yendo a la DERECHA",
                prefijo, nuevo_valor, self.valor
            );
            if let Some(ref mut derecha) = self.derecha {
                derecha.insertar_con_pasos(nuevo_valor, profundidad + 1);
            } else {
                println!(
                    "{}   [!] Espacio vacío. Insertando {} a la derecha de {}",
                    prefijo, nuevo_valor, self.valor
                );
                self.derecha = Some(Box::new(Nodo::nuevo(nuevo_valor)));
            }
        }
    }
    //buscar
    fn buscar(&self, valor: i32) -> bool {
        if self.valor == valor {
            true
        } else if valor < self.valor {
            self.izquierda
                .as_ref()
                .map_or(false, |nodo| nodo.buscar(valor))
        } else {
            self.derecha
                .as_ref()
                .map_or(false, |nodo| nodo.buscar(valor))
        }
    }
    //recorrido_inorden
    fn recorrido_inorden(&self) {
        if let Some(ref izquierda) = self.izquierda {
            izquierda.recorrido_inorden();
        }
        println!("{}", self.valor);
        if let Some(ref derecha) = self.derecha {
            derecha.recorrido_inorden();
        }
    }
    //altura
    fn altura(&self) -> usize {
        1 + self
            .izquierda
            .as_ref()
            .map_or(0, |nodo| nodo.altura())
            .max(self.derecha.as_ref().map_or(0, |nodo| nodo.altura()))
    }
    //eliminar
    fn eliminar(&mut self, valor: i32) -> Option<Box<Nodo>> {
        if valor < self.valor {
            if let Some(ref mut izquierda) = self.izquierda {
                izquierda.eliminar(valor);
            }
        } else if valor > self.valor {
            if let Some(ref mut derecha) = self.derecha {
                derecha.eliminar(valor);
            }
        } else {
            if self.izquierda.is_none() {
                return self.derecha.take();
            }
            if self.derecha.is_none() {
                return self.izquierda.take();
            }
            let mut temp = self.derecha.as_mut().unwrap();
            while let Some(ref mut izquierda) = temp.izquierda {
                temp = izquierda;
            }
            self.valor = temp.valor;
            self.derecha = temp.eliminar(temp.valor);
        }
        None
    }
    //contar nodos
    fn contar_nodos(&self) -> usize {
        1 + self
            .izquierda
            .as_ref()
            .map_or(0, |nodo| nodo.contar_nodos())
            + self.derecha.as_ref().map_or(0, |nodo| nodo.contar_nodos())
    }
    //grados
    fn grados(&self) -> usize {
        self.izquierda.as_ref().map_or(0, |_| 1) + self.derecha.as_ref().map_or(0, |_| 1)
    }
}

fn main() {
    let mut arbol = Nodo {
        valor: 7,
        izquierda: None,
        derecha: None,
    };
    // arbol.insertar(3);
    // arbol.insertar(10); // Cambié algunos valores para que se vea mejor la jerarquía
    // arbol.insertar(2);
    // arbol.insertar(5);
    // arbol.insertar(8);
    // arbol.insertar(12);
    println!("Altura: {}", arbol.altura());
    println!("Nodos: {}", arbol.contar_nodos());
    println!("Grados: {}", arbol.grados());
    arbol.eliminar(5);
    //buscar
    println!("Buscar el 3: {}", arbol.buscar(3));
    println!("Altura: {}", arbol.altura());
    println!("Nodos: {}", arbol.contar_nodos());
    println!("Grados: {}", arbol.grados());
    //recorrido in order
    // let mut lista_ordenada = Vec::new();
    arbol.recorrido_inorden();
    println!("--- Insertando el 3 ---");
    arbol.insertar_con_pasos(3, 0);

    println!("\n--- Insertando el 9 ---");
    arbol.insertar_con_pasos(9, 0);

    println!("\n--- Insertando el 5 ---");
    arbol.insertar_con_pasos(5, 0);

    println!("\nResultado final:");
    println!(
        "mostrando el arbol jerarquicamente: {:#?}",
        arbol.mostrar(0)
    );

    // println!("{:#?}\n", arbol);
}
