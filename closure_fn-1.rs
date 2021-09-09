fn test<T>(f: T) where
    T: Fn()
{
    f();
}

fn main() {
    let s = String::from("董泽润的技术笔记");
    let f = || {println!("{}", s);};
    test(f);
}
