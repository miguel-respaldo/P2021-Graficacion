fn factor(num: u64) -> u64 
{
  match num {

      0 => 1,
      1 => 1,
      _ => factor(num - 1) * num,
      
  }
}


fn main() {
  let x = factor(11);
  println!("El factorial es:  { } ", x);
} 
//Julio Cesar Baspineiro Arancibia
//17011436