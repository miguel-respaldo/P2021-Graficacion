fn factorial(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

fn main() {
    let x = factorial(30);
    println!("El factorial es: {} ", x);
}
