use std::io;

fn main() {
    println!("Hi!");
    println!("Powered by: Rico Ramirez Luis Angel");
    println!("Powered by: Ricolino! :D ");
    println!("Fibonacci!");

    println!("escribe 'salir' para terminar el programa! ");

    loop{
        let mut n = String::new();
        println!("Inserta un entero positivo:");

        io::stdin().read_line(&mut n).expect("Fallo al leer el numero");

        if n.trim() == "salir"{
            break;
        }
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", fibonacci(n));
    }
}

fn fibonacci (n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n -2),
    }
}
