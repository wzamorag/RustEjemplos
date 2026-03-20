use std::collections::{HashMap, HashSet};

struct Persona {
    nombre: String,
    edad: u16,
}
impl Persona {
    fn saludar(&self) {
        println!("Hola {}", self.nombre);
    }
    fn es_adulto(&self) -> bool {
        self.edad >= 18
    }
}
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}
enum Mensaje {
    Texto(String),
    Numero(i32),
}
enum Semaforo {
    Rojo,
    Amarillo,
    Verde,
}
fn accion_semaforo(color: Semaforo) {
    match color {
        Semaforo::Rojo => println!("Detente"),
        Semaforo::Amarillo => println!("Precaucion"),
        Semaforo::Verde => println!("Avanza"),
    }
}
const PI: f32 = 3.1415;
///Este es un ejemplo
/// de documentacion de ejemplo
/// para que se pueda ver prueba
fn main() {
    //declaracion de variables str
    let mensaje: &str = "Hola Rust";
    println!("{}", mensaje);
    //mutable
    let mut contador: i8 = 0;
    //inmutable
    let numero = 10;
    //especifica tipo
    let dec1: f32 = 12.34;
    //infiere el tipo flotante
    let decimal = 3.56;
    contador += 1;
    println!("{}", contador);
    //tipo boolean
    let es_verdad: bool = true;
    let verda = true;
    let es_falso: bool = false;
    //tipo char
    let letra: char = 'a';
    let emoji: char = '🦀';
    println!("{}", emoji);
    // tipos compuestos
    // tupla es un conjuto de elementos
    let tupla: (i32, f32, bool, char) = (32, 4.54, true, 'k');
    println!("{:#?}", tupla);
    // arrays
    let arreglo: [i8; 5] = [1, 3, 5, 7, 9];
    println!("{:#?}", arreglo); //arreglo completo
    println!("{}", arreglo[2]);
    // inferencia de tipos
    let num = 40; //infiere el tipo de dato
    let num1 = 4.32; //tipo de datos
    // const se declaran fuera para tener un alcance global
    println!("{}", PI);
    //sombra de variables
    // es una forma de declarar una variable con el mismo nombre en el mismo ambito ocultando la variable anterior
    let numero: i8 = 9;
    let numero = 12;
    println!("{}", numero);
    // operadores artimetios +, -, *, /, %
    let num4 = 10;
    let num5 = 12;
    println!("la suma es: {}", num4 + num5);
    // operadores logicos &&, ||, !
    let a = true;
    let b = false;
    println!("operador and {}", a && b);
    println!("operador or {}", a || b);
    println!("operador not {}", !a);
    // camparar ==, !=,>,< ,>=,<=
    println!("{}", a == b);

    // control de flujo if, match
    let num4 = 9;
    let num5 = 10;
    if num4 > 10 && num5 < 10 {
        println!("El numero es mayor");
    } else {
        println!("El numero es menor");
    }
    // match
    let tupla: (i32, f32, bool, char) = (1, 4.54, true, 'k');

    let numero1 = 6;
    match tupla.0 {
        1 => println!("El numero es 1"),
        2 => println!("El numero es 2"),
        3 => println!("El numero es 3"),
        _ => println!("Es otro numero"),
    };

    // bubles loop, for, while
    let mut contador: i16 = 0;
    let valor = 2;
    loop {
        // println!("Estoy ejecutando el programa");
        contador += 1;
        println!("{}x{}={}", valor, contador, valor * contador);
        if contador == 10 {
            break;
        }
    }
    // while
    let mut contador = 0;
    while contador < 4 {
        println!("Ejemplo con while");
        contador += 1;
    }
    //bucle for
    for valores in 1..5 {
        println!("{}", valores);
    }

    // comentarios de linea, bloque y documentacion
    // documentacion cargo doc antes de cada funcion

    //concepto de ownership mover y copiar
    //cada valor en rust tiene un dueno
    // ejemplo saludo
    // let saludo = String::from("Hola"); //saludo es el propietario de hola
    // //que pasa si creo una nueva variable y le asigno saludo
    // let mensaje = saludo; //aca estamos transfierendo(moviendo) a mensaje el propietario de hola y daria error el imprimir saludo
    // println!("{}", saludo); //error debemos utilizar mensaje
    // ejemplo de copia se usa en enteros, flotantes , char,bool
    let num1 = 10;
    let num2 = num1;
    println!("num2 {}", num2); //estamos haciendo una copia y podemos utilizar num1 o num2 y no daria error
    println!("num1 {}", num1);
    // referencias prestamo
    // esto se refiere a que voy a utilizar el valor sin ser su propietario
    let texto = String::from("hola que tal como estas");
    imprime_longitud(&texto); //pasamos la referencia inmmutable
    println!("{}", texto);
    // ahora referencias mutables
    let mut texto = String::from("Hola");
    modifica_referencia(&mut texto); //modificamos la referencia agregando una palabra
    println!("{}", texto);
    // puedo tener todas las referencias inmutables que quiera
    // puedo tener una referencia mutable
    // no puedo tener una mezcla de referencias mutables e imnutables al mismo tiempo.
    // lifetime
    // el tiempo durante el cual una referencia sigue siendo validad
    // ejemplo |Hola|Como|Estas|v|v|v|v|v|v|v| Memoria Ram
    // referencia a hola ->|hola|
    // que pasa si la libero |    |Como|Estas|||||
    // referencia a hola -> | | Error
    // Ejemplo erroneo:
    // let r;
    // {
    //     let x = 5;
    //     r = &x; //esto da error porque x no existe fuera de ese bloque
    //     //esto es un problema para otros lenguajes debido a que podemos
    //     // enviar referencias de variables que no existes o que ya fueron liberadas
    // }
    // println!("{}", r);
    // ejemplo funcional
    let x = 5;
    let r = &x; //aca funciona porque esta viva la referencia
    println!("{}", r);
    // ahora con lifetime
    // creamos una funcion devolver texto y probamos imprimir el texto de referencia
    // esto dara un error
    // let mensaje = devolver_texto();
    // println!("{}", mensaje);
    // ahora si correcto:
    let texto = String::from("Hola Mundo"); //esto si funciona porque le estamos diciendo que este viva en entrada
    let mensaje = devolver_texto(&texto); //y tambien en la salida de la funcion
    println!("{}", mensaje); //justo despues de imprimir en pantalla muere la referencia
    //
    // Colecciones String y &str
    // String es mutable y &str es inmutable
    let mut texto = String::from("Hola"); //string no es copiable, y no es de tamano fijo
    texto.push_str(" Mundo");
    println!("el texto es:{} y su tamanio es:{} ", texto, texto.len());
    let mensaje: &str = "Hola Mundo con str"; //esta es una referencia, si es copiable y es de tamano fijo
    println!("{}", mensaje);
    let mensaje1 = "Esto es otro str";
    println!(
        "Empieza con:{} y termina con: {} ",
        mensaje1.starts_with("es"),
        mensaje1.ends_with("str")
    );
    //veamos el slicing [..]
    let nuevo = &texto[3..6]; //si lo quiero todo solo pongo ..
    println!("{}", nuevo);
    // vector
    //
    let mut numeros: Vec<i32> = Vec::new();
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);
    println!("{:?}", numeros);
    let frutas = vec!["manzana", "pera", "jocote"];
    println!("{:?}", frutas);
    println!("{}", frutas[0]);
    println!("{:?}", frutas.get(1)); //el resultado es un option
    //recorrer el vector
    for valor in frutas {
        println!("{}", valor);
    }
    // Hasmap y Hashset
    // usamos std:collections::HashMap;
    let mut edades = HashMap::new();
    edades.insert("ana", 30);
    edades.insert("luis", 25);
    edades.insert("pedro", 31);
    println!("{:?}", edades);
    println!("{:?}", edades.get("luis"));
    //iterando
    for (nombre, edad) in &edades {
        println!("{} {}", nombre, edad);
    }
    // HashSet
    let mut colores = HashSet::new();
    colores.insert("rojo");
    colores.insert("verde");
    colores.insert("azul");
    colores.insert("azul");
    println!("{:?}", colores); //solo imprime rojo verde azul y quita el duplicado

    //iteradores y clouser .iter(), .map(), .collect(),
    let frutas = vec!["manzana", "pera", "jocote"];
    for fruta in frutas.iter() {
        //recorro los elementos
        println!("{}", fruta);
    }
    let numeros = vec![1, 2, 3, 4, 5]; //map y clousere es un funcion flecha
    let multi: Vec<_> = numeros.iter().map(|f| f * 2).collect();
    println!("{:?}", multi);

    let pares: Vec<_> = numeros
        .iter()
        .cloned() //copia cada elemento
        .filter(|x| x % 2 == 0)
        .collect();
    println!("{:?}", pares);

    //struct, enum
    //
    let persona = Persona {
        nombre: String::from("anita"),
        edad: 35,
    };
    println!("{} tiene,{} anios", persona.nombre, persona.edad);
    persona.saludar();
    if persona.es_adulto() {
        println!("Es mayor de edad");
    } else {
        println!("Es menor de edad")
    }

    //enum es una enumeracion
    // Option, Result
    //
    let dir = Direccion::Norte;
    match dir {
        Direccion::Norte => println!("Vas al norte"),
        Direccion::Este => println!("Vas al este"),
        _ => println!("vas sin direccion"),
    }
    let men = Mensaje::Texto(String::from("Hola"));
    match men {
        Mensaje::Texto(t) => println!("Mensaje de Texto: {} ", t),
        Mensaje::Numero(n) => println!("Numero: {}", n),
    }
    //option, Result
    //
    let semaforo = Semaforo::Verde;
    accion_semaforo(semaforo);
    //manejo de errores
    // Result<T,E> unwrap, expect, ?, match
    // enum Result<T,E>{
    //  Ok(T),
    // Err(E)
    // }
}

//esto da error por el lifetime
// fn devolver_texto() -> &str {
//     let texto = String::from("Hola");
//     &texto //no ponemos ; porque es el valor a devolver y podemos evitar el return
// }
//
fn devolver_texto<'esta_vivo>(entrada: &'esta_vivo String) -> &'esta_vivo str {
    &entrada[..] //los dos puntos es para devolver todo lo que reciba
}
fn imprime_longitud(palabra: &String) {
    println!("La longitud es: {}", palabra.len());
}
fn modifica_referencia(palabra: &mut String) {
    palabra.push_str(" mundo de la clase");
}
