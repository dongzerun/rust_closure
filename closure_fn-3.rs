fn test<T>(mut f: T) where
    T: FnMut()
{
    f();
}

fn main() {
    let mut s = String::from("董泽润的技术笔记");
    let f = || {s.push_str("不错");};
    test(f);
}
