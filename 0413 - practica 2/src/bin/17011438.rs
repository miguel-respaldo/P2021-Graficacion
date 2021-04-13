//Omar Alejandro Velazquez Lira
fn main() {
    println!("Result: {}", fibo(3));
}    
    
fn fibo(num: u32) -> u32 {
    if num <= 1{
        return num;
    }
    return fibo(num-1) + fibo(num-2);
}

