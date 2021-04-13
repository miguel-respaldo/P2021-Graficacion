fn main() {
    println!("Hi! fb");

    for num:u32 in 1..15 {
        println!("fib({}) = {}", num, fib{n:num});
    }

}

fn fib (n:u32) -> u32 {
    if n <= 0 {
        return 0;
    }else if n == 1 {
        return 1;
    }

    fib(n:n-1) + fib(n:n-2)
}
