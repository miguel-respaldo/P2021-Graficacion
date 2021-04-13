
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n -2),
    }
}


fn main() {
    println!("Fibonacci de 5 es {}", fibonacci(5));
}