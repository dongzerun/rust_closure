fn inc(x: Option<i32>) -> i32 {
    let a = match x {
        Some(v) => v,
        None => 100,
    };
    a + 1
}

fn inc2(x: Option<i32>) -> i32 {
    let a = x.unwrap_or(100);
    a + 1
}

fn inc3(x: Option<i32>) -> (i32, i32) {
    let a = x.unwrap_or(100);
    (a + 1, a-1)
}

fn main() {
    let x = 1;
    fn test() {
        println!("{}", x);
    }
    test();
}
