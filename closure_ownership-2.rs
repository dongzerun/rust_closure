fn main(){
    let s = String::from("test");
    let f = || {let _s = s;println!("{}", _s)};
    f();
    f(); // can't run, because s dropped
}
