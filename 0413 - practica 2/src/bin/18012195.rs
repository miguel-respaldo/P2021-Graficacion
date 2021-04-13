fn main() {
    println!("Result: {}", fibo(9));
}

//Nava Castro Luis Alberto

fn fibo(num: u32) -> u32 {
    if num <= 1{
        return num;
    }
    return fibo(num-1) + fibo(num-2);
}