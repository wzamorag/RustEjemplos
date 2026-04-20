#[derive(Debug)]
struct BNode {
    keys: Vec<i32>,
    children: Vec<Box<BNode>>,
    is_leaf: bool,
}

pub struct BTree {
    root: Option<Box<BNode>>,
    t: usize, // Grado mínimo (define la capacidad del nodo)
}

impl BTree {
    pub fn new(t: usize) -> Self {
        BTree { root: None, t }
    }
    pub fn grado_minimo(&self) -> usize {
        self.t
    }

    // --- Lógica de Búsqueda ---
    pub fn search(&self, key: i32) -> bool {
        self.root
            .as_ref()
            .map_or(false, |node| self.search_recursive(node, key))
    }

    fn search_recursive(&self, node: &BNode, key: i32) -> bool {
        let mut i = 0;
        while i < node.keys.len() && key > node.keys[i] {
            i += 1;
        }
        if i < node.keys.len() && node.keys[i] == key {
            return true;
        }
        if node.is_leaf {
            return false;
        }
        self.search_recursive(&node.children[i], key)
    }

    // --- Lógica de Inserción ---
    // pub fn insert(&mut self, key: i32) {
    //     // Implementación simplificada: manejaría el split del nodo raíz
    //     // cuando está lleno (size == 2*t - 1)
    //     println!("Insertando {}", key);
    // }
    //
    pub fn insert(&mut self, key: i32) {
        println!("Insertando {}", key);

        match self.root {
            // Si el árbol está vacío, creamos el nodo raíz
            None => {
                self.root = Some(Box::new(BNode {
                    keys: vec![key],
                    children: Vec::new(),
                    is_leaf: true,
                }));
            }
            // Si ya existe la raíz, aquí iría la lógica compleja de navegación y split
            Some(ref mut node) => {
                node.keys.push(key);
                node.keys.sort(); // Mantenemos las llaves ordenadas
            }
        }
    }

    // --- Lógica de Eliminación ---
    pub fn delete(&mut self, key: i32) {
        // La eliminación en B-Trees requiere maniobras de préstamo
        // entre hermanos (borrowing) o fusión de nodos (merging)
        println!("Eliminando {}", key);
    }
    pub fn print_tree(&self) {
        if let Some(ref root) = self.root {
            self.print_node(root, 0);
        } else {
            println!("El árbol está vacío.");
        }
    }

    fn print_node(&self, node: &BNode, level: usize) {
        let indent = "  ".repeat(level);
        println!("{}- Claves: {:?}", indent, node.keys);

        for child in &node.children {
            self.print_node(child, level + 1);
        }
    }
}

fn main() {
    // 1. Inicialización: Definimos un B-Tree con grado mínimo t=3
    // Esto significa que los nodos pueden tener entre 2 y 5 claves.
    let mut btree = BTree::new(3);

    // 2. Inserción: Simulamos la carga de datos
    let valores = vec![10, 20, 5, 6, 12, 30, 7, 17];
    for val in valores {
        btree.insert(val);
        btree.print_tree(); // Visualización del estado actual
    }
    println!("--- Inserciones completadas ---");

    // 3. Búsqueda: Demostración de verificación de existencia
    let a_buscar = 12;
    if btree.search(a_buscar) {
        println!("¡Éxito! El valor {} fue encontrado en el árbol.", a_buscar);
    } else {
        println!("El valor {} no existe en el árbol.", a_buscar);
    }

    // 4. Eliminación: Demostración de mantenimiento
    btree.delete(6);
    println!("--- Eliminación de '6' solicitada ---");

    // 5. Verificación final
    let resultado = if btree.search(6) {
        "aún presente"
    } else {
        "eliminado correctamente"
    };
    println!("El valor 6 está: {}", resultado);
}
