fn test<T>(mut f: T) where
    T: FnMut()
{
    f(); // 我需要 FnMut 但是你可以不改
    f();
}

fn main() {
    let s = String::from("董泽润的技术笔记");
    let f = || {println!("s is {}", s);}; // 你可以不修改 s
    test(f);
}
