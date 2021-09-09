fn main() {
    // let f = |x: i32| -> i32 { x + 1 };
    // let f = |x: i32| x + 1;
    // let f = |x| x+1;
    // let f = |x| x;
    let f = |x| x;
    f('a');
    //f(i);

    let add_one = |x| x+1;
    println!("{}", add_one(3800));

    let x: i8 = 126;
    println!("{}", add_one(x));
}
