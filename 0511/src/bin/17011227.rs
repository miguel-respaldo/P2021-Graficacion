use std::io;

fn main() {
    println!("===Factorial===");
    loop{
        let mut n = String::new();
        println!("Ingresa un numero:");

        io::stdin().read_line(&mut n).expect("Fallo al leer el numero");
        println!("Resultado: ");
        
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