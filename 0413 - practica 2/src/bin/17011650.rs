fn main() {
    let x:i32 = 0, y:i32 = 0, z:i32 = 0, w:i32 = 1;

    for x in 3 0..10 {
    
      y=z+w;
      z=w;
      w=y;
      
      println!("{}",y); 
      
      
    }

}


