//FACTORIAL
/* Alumno: Hugo Emmanuel Gonzalez Gomez
   Materia: Graficacion 
   Semestre: 8vo*/

   use std::io;

   fn main(){
       let _i:i32 = 1;
   
       let mut numero = String::new();
       println!("Escribe un numero: ");
       io::stdin().read_line(&mut numero);
       let numero = numero.trim();
       let numero:i32 = numero.parse().unwrap();
   
       for i in 1..numero{
         let numero = numero * i;
         println!("El factorial es: {}", numero);
       }
   
   }