mod operaciones_numericas;
use operaciones_numericas::OperacionesNumericas;

fn main() {
    let mut operaciones = OperacionesNumericas::new();
    operaciones.mostrar_menu();
}
