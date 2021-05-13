// Miguel Angel Lopez Alcantar

fn fib(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n-1) + fib(n -2),
    }
}


fn print_fb(n: u32){
	println!("Fibonacci de {} es {}", n, fib(n));
}


fn main() {
	print_fb(10);
	print_fb(4);    
	print_fb(20);
	print_fb(5);
}