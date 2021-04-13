//Abraham Lomeli 17012453

fn main() {
    let mut x1 = 0;
    let mut x2 = 1;
    let mut z = 0;
//    let mut i = 0;
    for i in 0..5{
        z = x1+x2;
    println!("Valor x: {}", z);
    x1=x2;
    x2=z;
  }
}
