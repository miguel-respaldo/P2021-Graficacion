use std::io;

fn main() {
    println!("Hi!");
    println!("Powered by: Rico Ramirez Luis Angel");
    println!("Powered by: Ricolino! :D ");
    println!("Factorial");
    println!();
    println!(" ---------------------------------------------- ");
    println!("escribe 'salir' para terminar el programa! ");
    println!(" ---------------------------------------------- ");
    println!();

    loop{
        let mut n = String::new();
        println!("Ingresa un entero positivo:");

        io::stdin().read_line(&mut n).expect("Fallo al leer el numero");
        println!("Tu numero: ");
        if n.trim() == "salir"{
            break;
        }
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", factorial(n));
        println!();
    }
}

fn factorial (n: u32) -> u32 {
 
    if n <= 1 {
        1
    }else{
        println!("x  {} =",n);
        
        n*factorial(n - 1)
    }
}