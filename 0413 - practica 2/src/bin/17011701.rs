// Christopher Daniel Esparza Contreras


fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("Fibonacci 5");
    
    println!("{}", fibonacci(5));
}