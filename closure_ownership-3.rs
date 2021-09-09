fn main(){
    let mut s = String::from("test");
    let mut f = || {s.push('a');println!("{}", s)};
    f();
    f();
}
