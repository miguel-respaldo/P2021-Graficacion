use std::io;

fn factorial(n:u128) -> u128 {
    match n {
        0 | 1 => 1,
        _     => factorial(n-1) * n
    }
}

fn main() {

    println!("Escribe el un número para calcular su factorial: ");

    let mut numero = String::new();

    io::stdin()
        .read_line(&mut numero)
        .expect("Error al leer el número");

    let numero = numero
        .trim()           // Quita espacios (el Enter)
        .to_string()      // Lo hace String
        .parse()          // Lo convierte a Número
        .unwrap_or(0);   // Si hay un error, devuelve -1

    if numero == 0 {
        println!("No se ingreso un número correctamente");
    } else {

        let resultado = factorial(numero);

        println!("El resultado es {}", resultado);
    }
}
