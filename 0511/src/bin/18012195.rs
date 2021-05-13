use std::io;
use std::str::FromStr;

// Nava Castro Luis Alberto

fn main() {
    println!("Ingresa un numero: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");

    let n: u128 = u128::from_str(&input.trim()).unwrap();

    println!("El factorial de {0} es: {1}", n, factorial(n));
}

fn factorial(n: u128) -> u128{
    match n{
        0 | 1 => 1,
        _     => n * factorial(n-1)
    }
}