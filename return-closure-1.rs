fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let f = returns_closure();
    println!("res is {}", f(11));
}
