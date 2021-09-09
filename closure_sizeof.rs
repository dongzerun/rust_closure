fn main() {
    let v1 = 100;
    let v2 = 100;
    let a = |x: i32| x;
    let b = |x: i32| x + v1;
    let c = |x: i32| x + v1 + v2;
    
    assert_eq!(size_of(&a), 0);
    assert_eq!(size_of(&b), 8);
    assert_eq!(size_of(&c), 16);
}

fn size_of<T>(_: &T) -> usize {
    std::mem::size_of::<T>()
}
