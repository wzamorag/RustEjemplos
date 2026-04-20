// ============================================================================
// EXAMEN PARCIAL RUST - ALTERNATIVA 3: SISTEMA DE GESTIÓN DE BIBLIOTECA
// Solución Completa - Todos los temas integrados
// ============================================================================

use std::collections::{HashMap, VecDeque};

// ============================================================================
// PARTE 1: LIBRO CON PILA DE HISTORIAL DE PRÉSTAMOS
// ============================================================================

/// Enum para el estado de un libro
#[derive(Debug, Clone, PartialEq)]
enum EstadoLibro {
    Disponible,
    Prestado { usuario: String, fecha: String },
}

/// Estructura que representa un Libro con historial de préstamos (PILA)
#[derive(Debug, Clone)]
struct Libro {
    isbn: u64,
    titulo: String,
    autor: String,
    estado: EstadoLibro,
    // PILA: Historial de últimos préstamos (máximo 5)
    historial_prestamos: Vec<String>,
}

impl Libro {
    /// Constructor de un nuevo libro
    fn nuevo(isbn: u64, titulo: String, autor: String) -> Self {
        Libro {
            isbn,
            titulo,
            autor,
            estado: EstadoLibro::Disponible,
            historial_prestamos: Vec::new(),
        }
    }

    /// Registra un préstamo (agregar a la pila)
    fn registrar_prestamo(&mut self, usuario: &str, fecha: &str) -> Result<(), String> {
        match &self.estado {
            EstadoLibro::Disponible => {
                self.estado = EstadoLibro::Prestado {
                    usuario: usuario.to_string(),
                    fecha: fecha.to_string(),
                };

                // Agregar al historial
                let registro = format!("{} - {} ({})", usuario, fecha, self.titulo);
                self.historial_prestamos.push(registro);

                // Mantener máximo 5 registros (FIFO)
                if self.historial_prestamos.len() > 5 {
                    self.historial_prestamos.remove(0);
                }

                Ok(())
            }
            EstadoLibro::Prestado { usuario: u, .. } => {
                Err(format!("Error: '{}' ya está prestado a {}", self.titulo, u))
            }
        }
    }

    /// Registra una devolución (desapila del estado actual)
    fn registrar_devolucion(&mut self) -> Result<(), String> {
        match &self.estado {
            EstadoLibro::Disponible => Err(format!("Error: '{}' no está prestado", self.titulo)),
            EstadoLibro::Prestado { .. } => {
                self.estado = EstadoLibro::Disponible;
                Ok(())
            }
        }
    }

    /// Obtiene el historial completo de préstamos
    fn obtener_historial(&self) -> Vec<String> {
        self.historial_prestamos.clone()
    }

    /// Verifica si el libro está disponible
    fn esta_disponible(&self) -> bool {
        matches!(self.estado, EstadoLibro::Disponible)
    }

    /// Obtiene el usuario actual a quien está prestado
    fn usuario_actual(&self) -> Option<&str> {
        match &self.estado {
            EstadoLibro::Disponible => None,
            EstadoLibro::Prestado { usuario, .. } => Some(usuario),
        }
    }
}

// ============================================================================
// PARTE 2: TABLA HASH DE USUARIOS Y LIBROS
// ============================================================================

/// Estructura que representa un usuario
#[derive(Debug, Clone)]
struct Usuario {
    id: u32,
    nombre: String,
    email: String,
    libros_activos: Vec<u64>, // ISBNs de libros prestados
}

impl Usuario {
    /// Constructor de un nuevo usuario
    fn nuevo(id: u32, nombre: String, email: String) -> Self {
        Usuario {
            id,
            nombre,
            email,
            libros_activos: Vec::new(),
        }
    }

    /// Agrega un libro al usuario
    fn agregar_libro(&mut self, isbn: u64) {
        if !self.libros_activos.contains(&isbn) {
            self.libros_activos.push(isbn);
        }
    }

    /// Remueve un libro del usuario
    fn quitar_libro(&mut self, isbn: u64) {
        self.libros_activos.retain(|&isbne| isbne != isbn);
    }

    /// Cantidad de libros activos
    fn cantidad_libros(&self) -> usize {
        self.libros_activos.len()
    }
}

/// Gestor principal de la biblioteca
#[derive(Debug)]
struct GestorBiblioteca {
    // TABLA HASH: ID usuario -> Usuario
    usuarios: HashMap<u32, Usuario>,
    // TABLA HASH: ISBN -> Libro
    libros: HashMap<u64, Libro>,
}

impl GestorBiblioteca {
    /// Crea un nuevo gestor de biblioteca
    fn nuevo() -> Self {
        GestorBiblioteca {
            usuarios: HashMap::new(),
            libros: HashMap::new(),
        }
    }

    /// Registra un nuevo usuario
    fn registrar_usuario(&mut self, usuario: Usuario) -> Result<(), String> {
        if self.usuarios.contains_key(&usuario.id) {
            return Err(format!("Error: Usuario con ID {} ya existe", usuario.id));
        }
        self.usuarios.insert(usuario.id, usuario);
        Ok(())
    }

    /// Agrega un nuevo libro al catálogo
    fn agregar_libro(&mut self, libro: Libro) -> Result<(), String> {
        if self.libros.contains_key(&libro.isbn) {
            return Err(format!("Error: Libro con ISBN {} ya existe", libro.isbn));
        }
        self.libros.insert(libro.isbn, libro);
        Ok(())
    }

    /// Realiza un préstamo de libro a un usuario
    fn prestar_libro(&mut self, id_usuario: u32, isbn: u64, fecha: &str) -> Result<(), String> {
        // Verificar que el usuario existe
        if !self.usuarios.contains_key(&id_usuario) {
            return Err(format!("Error: Usuario {} no existe", id_usuario));
        }

        // Verificar que el libro existe
        if !self.libros.contains_key(&isbn) {
            return Err(format!("Error: Libro con ISBN {} no existe", isbn));
        }

        // Obtener nombre del usuario
        let nombre_usuario = self.usuarios[&id_usuario].nombre.clone();

        // Registrar el préstamo en el libro
        self.libros
            .get_mut(&isbn)
            .unwrap()
            .registrar_prestamo(&nombre_usuario, fecha)?;

        // Agregar el libro al usuario
        self.usuarios
            .get_mut(&id_usuario)
            .unwrap()
            .agregar_libro(isbn);

        Ok(())
    }

    /// Devuelve un libro prestado
    fn devolver_libro(&mut self, id_usuario: u32, isbn: u64) -> Result<(), String> {
        // Verificar que el usuario existe
        if !self.usuarios.contains_key(&id_usuario) {
            return Err(format!("Error: Usuario {} no existe", id_usuario));
        }

        // Registrar la devolución en el libro
        self.libros
            .get_mut(&isbn)
            .ok_or("Error: Libro no encontrado")?
            .registrar_devolucion()?;

        // Quitar el libro del usuario
        self.usuarios
            .get_mut(&id_usuario)
            .unwrap()
            .quitar_libro(isbn);

        Ok(())
    }

    /// Búsqueda secuencial de usuario por email
    fn buscar_usuario_por_email(&self, email: &str) -> Option<&Usuario> {
        self.usuarios
            .values()
            .find(|u| u.email.to_lowercase().contains(&email.to_lowercase()))
    }

    /// Obtiene los libros prestados por un usuario
    fn listar_libros_de_usuario(&self, id_usuario: u32) -> Option<Vec<&Libro>> {
        self.usuarios.get(&id_usuario).map(|usuario| {
            usuario
                .libros_activos
                .iter()
                .filter_map(|isbn| self.libros.get(isbn))
                .collect()
        })
    }

    /// Obtiene libros ordenados por ISBN
    fn obtener_libros_ordenados(&self) -> Vec<Libro> {
        let mut libros: Vec<Libro> = self.libros.values().cloned().collect();
        libros.sort_by_key(|l| l.isbn);
        libros
    }

    /// Búsqueda binaria de un libro por ISBN
    fn buscar_libro_por_isbn(&self, isbn_buscar: u64) -> Option<Libro> {
        let libros = self.obtener_libros_ordenados();
        let mut izq = 0;
        let mut der = libros.len();

        while izq < der {
            let mid = izq + (der - izq) / 2;

            match libros[mid].isbn.cmp(&isbn_buscar) {
                std::cmp::Ordering::Equal => return Some(libros[mid].clone()),
                std::cmp::Ordering::Less => izq = mid + 1,
                std::cmp::Ordering::Greater => der = mid,
            }
        }

        None
    }

    /// Búsqueda secuencial de libros por título
    fn buscar_libro_por_titulo(&self, titulo_busca: &str) -> Vec<&Libro> {
        let titulo_busca = titulo_busca.to_lowercase();
        self.libros
            .values()
            .filter(|l| l.titulo.to_lowercase().contains(&titulo_busca))
            .collect()
    }

    /// Obtiene libros con ISBN en un rango
    fn libros_por_rango_isbn(&self, isbn_min: u64, isbn_max: u64) -> Vec<Libro> {
        let libros = self.obtener_libros_ordenados();
        libros
            .into_iter()
            .filter(|l| l.isbn >= isbn_min && l.isbn <= isbn_max)
            .collect()
    }

    /// Cantidad total de libros
    fn cantidad_libros(&self) -> usize {
        self.libros.len()
    }

    /// Cantidad total de usuarios
    fn cantidad_usuarios(&self) -> usize {
        self.usuarios.len()
    }

    /// Obtiene referencia a los libros
    fn obtener_libros(&self) -> &HashMap<u64, Libro> {
        &self.libros
    }

    /// Obtiene referencia a los usuarios
    fn obtener_usuarios(&self) -> &HashMap<u32, Usuario> {
        &self.usuarios
    }
}

// ============================================================================
// PARTE 3: COLA DE SOLICITUDES DE PRÉSTAMO
// ============================================================================

/// Estructura para una solicitud de préstamo
#[derive(Debug, Clone)]
struct SolicitudPrestamo {
    id_usuario: u32,
    isbn_libro: u64,
    fecha_solicitud: String,
}

impl SolicitudPrestamo {
    /// Constructor
    fn nuevo(id_usuario: u32, isbn_libro: u64, fecha_solicitud: String) -> Self {
        SolicitudPrestamo {
            id_usuario,
            isbn_libro,
            fecha_solicitud,
        }
    }
}

/// Cola de solicitudes de préstamo (FIFO)
#[derive(Debug)]
struct ColaSolicitudes {
    // COLA: VecDeque para comportamiento FIFO
    cola: VecDeque<SolicitudPrestamo>,
    procesadas: Vec<SolicitudPrestamo>,
}

impl ColaSolicitudes {
    /// Crea una nueva cola vacía
    fn nuevo() -> Self {
        ColaSolicitudes {
            cola: VecDeque::new(),
            procesadas: Vec::new(),
        }
    }

    /// Agrega una solicitud a la cola
    fn solicitar_prestamo(&mut self, solicitud: SolicitudPrestamo) {
        self.cola.push_back(solicitud);
    }

    /// Procesa la siguiente solicitud (pop_front)
    fn procesar_siguiente(&mut self) -> Option<SolicitudPrestamo> {
        if let Some(solicitud) = self.cola.pop_front() {
            self.procesadas.push(solicitud.clone());
            Some(solicitud)
        } else {
            None
        }
    }

    /// Cantidad de solicitudes pendientes
    fn cantidad_pendientes(&self) -> usize {
        self.cola.len()
    }

    /// Lista las solicitudes pendientes
    fn listar_pendientes(&self) {
        if self.cola.is_empty() {
            println!("No hay solicitudes pendientes.");
            return;
        }

        println!("\n=== SOLICITUDES DE PRÉSTAMO PENDIENTES ===");
        for (idx, sol) in self.cola.iter().enumerate() {
            println!(
                "{}. Usuario {} pide ISBN {} - {}",
                idx + 1,
                sol.id_usuario,
                sol.isbn_libro,
                sol.fecha_solicitud
            );
        }
    }

    /// Obtiene el historial de solicitudes procesadas
    fn obtener_historial(&self) -> &Vec<SolicitudPrestamo> {
        &self.procesadas
    }
}

// ============================================================================
// PARTE 5: REPORTES DE LA BIBLIOTECA
// ============================================================================

struct ReporteBiblioteca;

impl ReporteBiblioteca {
    /// Reporte de libros disponibles vs prestados
    fn libros_disponibles_vs_prestados(gestor: &GestorBiblioteca) {
        let mut disponibles = 0;
        let mut prestados = 0;

        for libro in gestor.obtener_libros().values() {
            if libro.esta_disponible() {
                disponibles += 1;
            } else {
                prestados += 1;
            }
        }

        println!("\n📊 LIBROS DISPONIBLES VS PRESTADOS:");
        println!("  Disponibles: {}", disponibles);
        println!("  Prestados: {}", prestados);
        println!("  Total: {}", disponibles + prestados);
    }

    /// Top 5 libros más prestados (por historial)
    fn top_5_libros_mas_prestados(gestor: &GestorBiblioteca) {
        let mut libros_vec: Vec<(&Libro, usize)> = gestor
            .obtener_libros()
            .values()
            .map(|l| (l, l.historial_prestamos.len()))
            .collect();

        libros_vec.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n🏆 TOP 5 LIBROS MÁS PRESTADOS:");
        for (idx, (libro, cantidad)) in libros_vec.iter().take(5).enumerate() {
            println!(
                "{}. '{}' por {} - Préstamos: {}",
                idx + 1,
                libro.titulo,
                libro.autor,
                cantidad
            );
        }
    }

    /// Usuarios con más préstamos activos
    fn usuarios_mas_activos(gestor: &GestorBiblioteca) {
        let mut usuarios_vec: Vec<(&Usuario, usize)> = gestor
            .obtener_usuarios()
            .values()
            .map(|u| (u, u.cantidad_libros()))
            .collect();

        usuarios_vec.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n👥 USUARIOS CON MÁS PRÉSTAMOS ACTIVOS:");
        for (idx, (usuario, cantidad)) in usuarios_vec.iter().take(3).enumerate() {
            if *cantidad > 0 {
                println!(
                    "{}. {} ({}) - {} libros",
                    idx + 1,
                    usuario.nombre,
                    usuario.email,
                    cantidad
                );
            }
        }
    }

    /// Libros nunca prestados
    fn libros_nunca_prestados(gestor: &GestorBiblioteca) {
        let nunca_prestados: Vec<&Libro> = gestor
            .obtener_libros()
            .values()
            .filter(|l| l.historial_prestamos.is_empty())
            .collect();

        if nunca_prestados.is_empty() {
            println!("\n✓ Todos los libros han sido prestados al menos una vez.");
            return;
        }

        println!("\n📚 LIBROS NUNCA PRESTADOS:");
        for libro in nunca_prestados {
            println!(
                "  - '{}' por {} (ISBN: {})",
                libro.titulo, libro.autor, libro.isbn
            );
        }
    }
}

// ============================================================================
// PARTE 6: PROGRAMA PRINCIPAL COMPLETO
// ============================================================================

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║       SISTEMA DE GESTIÓN DE BIBLIOTECA - EXAMEN            ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    // =======================================================================
    // PASO 1: REGISTRAR USUARIOS
    // =======================================================================
    println!("\n👥 PASO 1: Registrando usuarios...");

    let mut gestor = GestorBiblioteca::nuevo();
    let mut cola_solicitudes = ColaSolicitudes::nuevo();

    let usuarios = vec![
        (101, "Juan García", "juan@email.com"),
        (102, "María López", "maria@email.com"),
        (103, "Pedro Martínez", "pedro@email.com"),
        (104, "Ana Rodríguez", "ana@email.com"),
        (105, "Carlos Silva", "carlos@email.com"),
    ];

    for (id, nombre, email) in usuarios {
        let usuario = Usuario::nuevo(id, nombre.to_string(), email.to_string());
        gestor.registrar_usuario(usuario).unwrap();
    }

    println!("✓ Se registraron {} usuarios", gestor.cantidad_usuarios());

    // =======================================================================
    // PASO 2: AGREGAR LIBROS AL CATÁLOGO
    // =======================================================================
    println!("\n📚 PASO 2: Agregando libros al catálogo...");

    let libros = vec![
        (9780060932214, "El Quijote", "Miguel de Cervantes"),
        (9780451524935, "1984", "George Orwell"),
        (9780316769174, "The Catcher in the Rye", "J.D. Salinger"),
        (9780142437933, "Pride and Prejudice", "Jane Austen"),
        (9780451526342, "Frankenstein", "Mary Shelley"),
        (9780141439556, "Great Expectations", "Charles Dickens"),
        (9780451529015, "The Great Gatsby", "F. Scott Fitzgerald"),
        (9780380016815, "To Kill a Mockingbird", "Harper Lee"),
        (9780451530786, "Moby Dick", "Herman Melville"),
        (9780451530776, "Jane Eyre", "Charlotte Brontë"),
        (9780060848898, "The Hobbit", "J.R.R. Tolkien"),
        (9780547928227, "The Lord of the Rings", "J.R.R. Tolkien"),
        (9780451531988, "Crime and Punishment", "Fyodor Dostoevsky"),
        (9780451526595, "Les Misérables", "Victor Hugo"),
        (9780451523808, "Anna Karenina", "Leo Tolstoy"),
    ];

    for (isbn, titulo, autor) in libros {
        let libro = Libro::nuevo(isbn, titulo.to_string(), autor.to_string());
        gestor.agregar_libro(libro).unwrap();
    }

    println!("✓ Se agregaron {} libros", gestor.cantidad_libros());

    // =======================================================================
    // PASO 3: CREAR SOLICITUDES DE PRÉSTAMO
    // =======================================================================
    println!("\n📋 PASO 3: Registrando solicitudes de préstamo...");

    let solicitudes = vec![
        (101, 9780060932214, "2024-01-15 09:00"),
        (102, 9780451524935, "2024-01-15 09:30"),
        (103, 9780316769174, "2024-01-15 10:00"),
        (101, 9780142437933, "2024-01-15 10:30"),
        (104, 9780451526342, "2024-01-15 11:00"),
        (105, 9780451529015, "2024-01-15 11:30"),
        (102, 9780380016815, "2024-01-15 12:00"),
        (103, 9780451530786, "2024-01-15 12:30"),
    ];

    for (id_usuario, isbn, fecha) in solicitudes {
        let solicitud = SolicitudPrestamo::nuevo(id_usuario, isbn, fecha.to_string());
        cola_solicitudes.solicitar_prestamo(solicitud);
    }

    println!("✓ Se registraron {} solicitudes", 8);
    cola_solicitudes.listar_pendientes();

    // =======================================================================
    // PASO 4: PROCESAR SOLICITUDES (COLA FIFO)
    // =======================================================================
    println!("\n⚙️  PASO 4: Procesando 5 solicitudes en orden FIFO...");

    for i in 1..=5 {
        if let Some(solicitud) = cola_solicitudes.procesar_siguiente() {
            match gestor.prestar_libro(
                solicitud.id_usuario,
                solicitud.isbn_libro,
                &solicitud.fecha_solicitud,
            ) {
                Ok(_) => {
                    if let Some(libro) = gestor.buscar_libro_por_isbn(solicitud.isbn_libro) {
                        println!(
                            "✓ {} Préstamo aprobado: '{}' a Usuario {}",
                            i, libro.titulo, solicitud.id_usuario
                        );
                    }
                }
                Err(e) => println!("✗ {} {}", i, e),
            }
        }
    }

    println!(
        "Solicitudes pendientes: {}",
        cola_solicitudes.cantidad_pendientes()
    );

    // =======================================================================
    // PASO 5: BÚSQUEDA BINARIA POR ISBN
    // =======================================================================
    println!("\n🔍 PASO 5: Búsqueda binaria de libros por ISBN...");

    let isbn_buscar = 9780451524935;
    match gestor.buscar_libro_por_isbn(isbn_buscar) {
        Some(libro) => {
            println!("✓ Libro encontrado (Búsqueda Binaria):");
            println!("  ISBN: {}", libro.isbn);
            println!("  Título: {}", libro.titulo);
            println!("  Autor: {}", libro.autor);
            println!(
                "  Estado: {}",
                if libro.esta_disponible() {
                    "Disponible"
                } else {
                    "Prestado"
                }
            );
            if let Some(usuario) = libro.usuario_actual() {
                println!("  Prestado a: {}", usuario);
            }
        }
        None => println!("✗ No se encontró libro con ISBN {}", isbn_buscar),
    }

    // =======================================================================
    // PASO 6: BÚSQUEDA SECUENCIAL POR TÍTULO
    // =======================================================================
    println!("\n📖 PASO 6: Búsqueda secuencial de libros por título...");

    let titulo_buscar = "Lord";
    let resultados = gestor.buscar_libro_por_titulo(titulo_buscar);

    if resultados.is_empty() {
        println!("No se encontraron libros con '{}'", titulo_buscar);
    } else {
        println!(
            "Se encontraron {} libro(s) con '{}':",
            resultados.len(),
            titulo_buscar
        );
        for libro in resultados {
            println!(
                "  - '{}' por {} - {}",
                libro.titulo,
                libro.autor,
                if libro.esta_disponible() {
                    "Disponible"
                } else {
                    "Prestado"
                }
            );
        }
    }

    // =======================================================================
    // PASO 7: BÚSQUEDA DE USUARIO POR EMAIL
    // =======================================================================
    println!("\n✉️  PASO 7: Búsqueda secuencial de usuario por email...");

    let email_buscar = "maria";
    if let Some(usuario) = gestor.buscar_usuario_por_email(email_buscar) {
        println!("✓ Usuario encontrado:");
        println!("  ID: {}", usuario.id);
        println!("  Nombre: {}", usuario.nombre);
        println!("  Email: {}", usuario.email);
        println!("  Libros activos: {}", usuario.cantidad_libros());

        if let Some(libros) = gestor.listar_libros_de_usuario(usuario.id) {
            println!("  Detalle de libros:");
            for libro in libros {
                println!("    - '{}'", libro.titulo);
            }
        }
    } else {
        println!("✗ No se encontró usuario con email '{}'", email_buscar);
    }

    // =======================================================================
    // PASO 8: BÚSQUEDA DE RANGO POR ISBN
    // =======================================================================
    println!("\n📐 PASO 8: Búsqueda de libros con ISBN en rango...");

    let libros_rango = gestor.libros_por_rango_isbn(9780451524935, 9780451530776);
    println!("Se encontraron {} libros en el rango:", libros_rango.len());
    for libro in libros_rango.iter().take(3) {
        println!("  - '{}' (ISBN: {})", libro.titulo, libro.isbn);
    }

    // =======================================================================
    // PASO 9: REPORTES Y ESTADÍSTICAS
    // =======================================================================
    println!("\n📈 PASO 9: Generando reportes...");

    ReporteBiblioteca::libros_disponibles_vs_prestados(&gestor);
    ReporteBiblioteca::top_5_libros_mas_prestados(&gestor);
    ReporteBiblioteca::usuarios_mas_activos(&gestor);
    ReporteBiblioteca::libros_nunca_prestados(&gestor);

    // =======================================================================
    // RESUMEN FINAL
    // =======================================================================
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                      RESUMEN FINAL                        ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!("Total de usuarios: {}", gestor.cantidad_usuarios());
    println!("Total de libros en catálogo: {}", gestor.cantidad_libros());
    println!(
        "Solicitudes procesadas: {}",
        cola_solicitudes.obtener_historial().len()
    );
    println!(
        "Solicitudes pendientes: {}",
        cola_solicitudes.cantidad_pendientes()
    );
    println!("\n✓ Examen completado exitosamente");
}
