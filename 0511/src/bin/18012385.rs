fn factor(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => factor(num - 1) * num,
    }
}

fn main() {
    let x = factor(30);
    println!("El valor del facotiral es { } ", x);
} 