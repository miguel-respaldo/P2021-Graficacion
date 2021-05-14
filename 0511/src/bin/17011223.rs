use std::io,



fn main (){

    let _i:i32=1;
    
    let mut num = String::new();
    println!("Calculadora de factoriales"\n 
    "ingresa el número a calcular");
    io::stdin().read_line(&mut num);

    let num=num.trim();
    let num:i32=num.parse().unwrap();

     //   for elem in iter {
            
       // }
    for i in 1..num{
        let num=num*i;
        println! ("RESULTADO:", num);

    }

}