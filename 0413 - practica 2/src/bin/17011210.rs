//Héctor Uriel Aguilera Peña

fn main() {
    for i in 0..10{
        println!("{}, ", fibonacci(i))
    }
}

fn fibonacci(num: i32) -> i32 {
    if num == 0 || num == 1 {
        num
    }else{
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}
