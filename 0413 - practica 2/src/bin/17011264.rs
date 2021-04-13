fn main() {
    println!("Hi!");

    for num: u32 in 0..11 {
        println!("Fib({}) ={}" , num, fib( n: num));
    }
}

fn fib(n:u32) -> u32 {
    if n <= 0 {
        return 0;
    }else if n == 1{
        return 1;
    }

    fib(n:n-1) + fib(n:n-2)
}
