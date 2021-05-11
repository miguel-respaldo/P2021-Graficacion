//Oscar Mauricio Trujillo Enriquez

use std::io;
use std::str::FromStr;

fn main(){
    println!("Ingresa un numero: ");
    
    let mut x = String::new();
    io::stdin().read_line(&mut x);
    
    let x = x.trim(); 
    let x: u64 = x.parse().unwrap();

    println!("FACTORIAL DE {}! = {}",x,factorial(x));
}

fn factorial(y : u64) -> u64{
    if y == 0 {
        1
    } else {
        y * factorial(y - 1)
    }
}
