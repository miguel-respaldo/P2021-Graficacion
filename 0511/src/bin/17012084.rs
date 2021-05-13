// Miguel Angel Lopez Alcantar


fn factorial(x: u32) -> u32 {
    match x {
        0 => 1,
        1 => 1,
        _ => factorial(x - 1) * x,
    }
}


fn print_fact(x: u32){
	println!("El factorial de {} es: {} ", x, factorial(x));
}


fn main() {
	print_fact(4);
	print_fact(7);
	print_fact(6);
	print_fact(9);
	print_fact(2);
}