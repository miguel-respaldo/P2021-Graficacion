fn factorial(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}
//Codigo que da el factorial del numero puesto
fn main() {
    let x = factorial(10);//10
    println!("El factorial es: {} ", x);
}