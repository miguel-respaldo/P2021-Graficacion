//Abraham Atzin Lomeli Grimaldo

fn fibo(x: u32) -> u32 {
    match x {
        0 => 1,
        1 => 1,
        _ => fibo(x-1) + fibo(x-2),
    }
}
fn main() {
    println!("Sequence {}", fibo(5));
}

