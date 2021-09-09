fn test() -> impl FnMut(char) {
    let mut s = String::from("董泽润的技术笔记");
    |c| { s.push(c); }
}

fn main() {
    let mut c = test();
    c('d');
    c('e');
}
