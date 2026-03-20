use std::io::{self, BufRead, Write};

pub struct OperacionesNumericas {
    numeros: Vec<i32>,
}

impl OperacionesNumericas {
    // Constructor
    pub fn new() -> Self {
        println!("Objeto OperacionesNumericas creado!");
        OperacionesNumericas {
            numeros: Vec::new(),
        }
    }

    // Leer números desde la entrada del usuario
    pub fn leer_numeros(&mut self) {
        println!("=== LEER NUMEROS ===");
        print!("Ingrese numeros separados por espacios (ej: 5 2 8 1): ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut entrada = String::new();
        stdin.lock().read_line(&mut entrada).unwrap();

        self.numeros.clear();

        for parte in entrada.trim().split_whitespace() {
            if let Ok(numero) = parte.parse::<i32>() {
                self.numeros.push(numero);
            }
        }

        println!("Se han leido {} numeros correctamente.", self.numeros.len());
    }

    // Mostrar los números almacenados
    pub fn mostrar_numeros(&self) {
        if self.numeros.is_empty() {
            println!("No hay numeros almacenados.");
            return;
        }

        print!("Numeros almacenados: ");
        for num in &self.numeros {
            print!("{} ", num);
        }
        println!();
    }

    // Encontrar divisores de un número
    pub fn encontrar_divisores(&self, numero: i32) {
        print!("Divisores de {}: ", numero);
        for i in 1..=numero {
            if numero % i == 0 {
                print!("{} ", i);
            }
        }
        println!();
    }

    // Ordenar con burbuja (ascendente o descendente)
    fn ordenar_burbuja_modo(&self, ascendente: bool) {
        if self.numeros.is_empty() {
            println!("No hay numeros para ordenar.");
            return;
        }

        let mut temp = self.numeros.clone();
        let n = temp.len();

        for i in 0..n - 1 {
            for j in 0..n - i - 1 {
                let condicion = if ascendente {
                    temp[j] > temp[j + 1]
                } else {
                    temp[j] < temp[j + 1]
                };

                if condicion {
                    temp.swap(j, j + 1);
                }
            }
        }

        let modo = if ascendente {
            "ascendente"
        } else {
            "descendente"
        };
        print!("Numeros ordenados {}: ", modo);
        for num in &temp {
            print!("{} ", num);
        }
        println!();
    }

    // Verificar si un número es palíndromo
    pub fn es_palindromo(&self, numero: i32) -> bool {
        let s = numero.to_string();
        let reverso: String = s.chars().rev().collect();
        s == reverso
    }

    // Verificar palíndromos en todos los números almacenados
    pub fn verificar_palindromos(&self) {
        if self.numeros.is_empty() {
            println!("No hay numeros para verificar.");
            return;
        }

        println!("Verificacion de palindromos:");
        for num in &self.numeros {
            let resultado = if self.es_palindromo(*num) {
                "ES PALINDROMO"
            } else {
                "No es palindromo"
            };
            println!("{} -> {}", num, resultado);
        }
    }

    // Menú principal
    pub fn mostrar_menu(&mut self) {
        loop {
            println!("\n=== MENU PRINCIPAL ===");
            println!("1. Leer numeros");
            println!("2. Mostrar numeros");
            println!("3. Ordenar ascendente (Burbuja)");
            println!("4. Ordenar descendente (Burbuja)");
            println!("5. Verificar palindromos");
            println!("6. Encontrar divisores de un numero");
            println!("0. Salir");
            print!("Seleccione una opcion: ");
            io::stdout().flush().unwrap(); // fuerza que aparezca en pantalla por haber usado print!

            let mut entrada = String::new();
            io::stdin().lock().read_line(&mut entrada).unwrap();
            // io::stdin().lock().read_line(&mut entrada).unwrap();
            //   1          2        3          4           5

            // **1. `io::stdin()`**
            // Obtiene el handle de la entrada estándar del sistema
            // (lo que el usuario escribe en la terminal). Equivale al `cin` de C++.

            // **2. `.lock()`**
            // Bloquea el stdin para uso exclusivo del hilo actual.
            // Esto evita que otros hilos lean al mismo tiempo.
            //  Es necesario porque Rust es seguro con hilos por defecto.

            // **3. `.read_line(&mut entrada)`**
            // Lee una línea completa (hasta que el usuario presiona Enter)
            // y la guarda en la variable `entrada`. El `&mut` es porque necesita
            //  modificar la variable para escribir en ella.

            // **4. `&mut entrada`**
            // Es una referencia mutable a `entrada`. En Rust no puedes modificar
            // una variable desde afuera sin declararlo explícitamente con `&mut`.

            // **5. `.unwrap()`**
            // `read_line` devuelve un `Result<usize, Error>` porque puede fallar
            // (ej: si la terminal se cierra). El `.unwrap()` dice: *"si funcionó,
            //  dame el valor; si falló, pánica y detén el programa"*. En producción
            // sería mejor manejar el error con `?` o `match`.

            let opcion: i32 = match entrada.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Opcion no valida. Intente de nuevo.");
                    continue;
                }
            };

            match opcion {
                1 => self.leer_numeros(),
                2 => self.mostrar_numeros(),
                3 => self.ordenar_burbuja_modo(true),
                4 => self.ordenar_burbuja_modo(false),
                5 => self.verificar_palindromos(),
                6 => {
                    print!("Ingrese un numero para encontrar sus divisores: ");
                    io::stdout().flush().unwrap(); // fuerza que aparezca en pantalla por haber usado print!

                    let mut buf = String::new();
                    io::stdin().lock().read_line(&mut buf).unwrap();

                    match buf.trim().parse::<i32>() {
                        Ok(num) => self.encontrar_divisores(num),
                        Err(_) => println!("Numero no valido."),
                    }
                }
                0 => {
                    println!("Saliendo del programa...");
                    break;
                }
                _ => println!("Opcion no valida. Intente de nuevo."),
            }
        }
    }
}

// Drop equivale al destructor de C++
impl Drop for OperacionesNumericas {
    fn drop(&mut self) {
        println!("Objeto OperacionesNumericas destruido!");
    }
}
