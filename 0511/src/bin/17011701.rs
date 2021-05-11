fn factorial(x: u32) -> u32 {
    if x < 2 {
        1
    } else {
        x * factorial(x - 1)
    }
}

fn main() {
    println!("{}",factorial(5));
  }