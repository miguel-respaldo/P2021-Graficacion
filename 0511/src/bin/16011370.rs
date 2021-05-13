fn factor(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => factor(num - 1) * num,
    }
}

fn main() {
    let x = factor(8);
    println!("Tu factorial es: { } ", x);
} 