fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Fibonacci generator");
    println!("Type \"quit\" to end the program");

    loop {
        let mut n = String::new();

        println!("\nEnter a positive integer:");

        if n.trim() == "quit" {
            break;
        }

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fibonacci(n));
    }
}