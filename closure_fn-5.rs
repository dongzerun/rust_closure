fn test<T>(f: T) where
    T: Fn()
{
    f();
    f();
}

fn main() {
    let s = String::from("董泽润的技术笔记");
    let f = move || {println!("s is {}", s);};
    test(f);
    println!("{}", s);
}
