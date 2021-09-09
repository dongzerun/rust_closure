fn main(){
    let mut a = 1;
    let mut inc = || {a+=1;a};
    inc();
    inc();
    println!("now a is {}", a); // output 3
}
