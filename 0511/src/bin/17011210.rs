fn main() {
    for i in 1..9 {
        println!("{}", fact(i));
    }
}

fn fact(num: i32) -> i32 {
    if num == 0 {
        1
    }else{
        num * fact(num - 1)
    }
}