fn test<T>(f: T) where
    T: FnOnce()
{
    f();
    f();
}

fn main() {
    let s = String::from("董泽润的技术笔记");
    let f = || {let _ = s;};
    test(f);
}
