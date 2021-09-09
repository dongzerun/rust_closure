fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let f = returns_closure();
    println!("res is {}", f(11));
}
