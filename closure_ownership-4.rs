fn main(){
    let s = String::from("test");
    let f = move || {println!("{}", s)};
    f();
    f();
}
