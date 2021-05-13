use std::io;

fn main() {
    println!("Jose Alfredo Cerda Vaca");
    println!("Practica 11 de mayo");
    println!("escribe 'salir' para terminar el programa! ");
    println!();

    loop{
        
        let mut n = String::new();
        println!("Digite un numero:");

        io::stdin().read_line(&mut n).expect("Intenta de nuevo");
        println!("Resultado: ");
        if n.trim() == "salir"{
            break;
        }
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", factorial(n));println!();
        
    }
}

fn factorial (n: u32) -> u32 {
    if n <= 1 {
        1}else{
        println!("x  {} =",n);
        n*factorial(n - 1)
    }
}