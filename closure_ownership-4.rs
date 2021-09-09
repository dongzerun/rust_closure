fn main(){
    let s = String::from("test");
    let f = move || {println!("{}", s)}; // s move to f
    f();
    f();
}
