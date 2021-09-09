fn call(f: fn()) {    // function pointer
    f();
}

fn main() {
    let a = 1;

    let f = || println!("abc");     // anonymous function 函数指针
    let c = || println!("{}", &a);  // closure 闭包

    call(f);
    call(c);
}
