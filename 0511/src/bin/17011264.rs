//Alan Yair Perez Alvarez

fn factor(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => factor(num - 1) * num,
    }
}

fn main() {
    let x = factor(11);
    println!("El valor del facotiral es { } ", x);
} 