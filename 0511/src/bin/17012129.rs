fn factorial(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

fn main() {
    println!("Soy José Ricardo Siordia Alatorre");
    let x = factorial(21);
    println!("El valor del factorial de 21 es:{} ", x);
}
