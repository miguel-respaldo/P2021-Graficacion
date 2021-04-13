//Oscar Mauricio Trujillo Enriquez

fn main(){
    let x = 10;
    println!("El numero de fibonacci de {}! = {}", x,fibonacci(x));
}

fn fibonacci(n:u32) -> u32{
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}