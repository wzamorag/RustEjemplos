fn main() {
    // let texto = String::from("Hola Mundo"); //esto si funciona porque le estamos diciendo que este viva en entrada
    // let mensaje = devolver_texto(&texto); //y tambien en la salida de la funcion
    // println!("{}", mensaje); //justo despues de imprimir en pantalla muere la referencia
    //
    // let mensaje = devolver_texto();
    // println!("{}", mensaje);

    // let resultado = dividir(12.0, 0.0);
    // let valor = resultado.expect("La division fallo");
    // println!("{}", valor);
    let variable: i8 = 8;
    let mut var = 9;
    var = 15;
    println!("{}", var);
    let numeros = vec![1, 2, 3, 4, 5, 6];
    let multi: Vec<_> = numeros.iter().map(|valor| valor * 3).collect();
    println!("{:#?}", multi);
}

// fn devolver_texto<'esta_vivo>(entrada: &'esta_vivo String) -> &'esta_vivo str {
//     &entrada[..] //los dos puntos es para devolver todo lo que reciba
// }

//esto da error por el lifetime
// fn devolver_texto() -> &str {
//     let texto = String::from("Hola Inside");
//     &texto; //no ponemos ; porque es el valor a devolver y podemos evitar el return
// }
// fn dividir(a: f32, b: f32) -> Result<f32, String> {
//     if b == 0.0 {
//         Err(String::from("No se puede dividir por cero"))
//     } else {
//         Ok(a / b)
//     }
// }
