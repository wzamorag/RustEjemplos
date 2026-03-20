use std::io;
///funcion para leer cualquier tipo de entrada
fn leer() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}
///funcion principal del programa
fn main() {
    println!("Escribe tu nombre:");
    let nombre = leer();

    println!("Escribe tu edad:");
    let edad: i32 = leer().parse().expect("No es un número");

    println!("Hola {nombre}, tenés {edad} años");
}
