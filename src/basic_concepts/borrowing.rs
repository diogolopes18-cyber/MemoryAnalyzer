pub fn borrow() {
    let mut x = 1;
    let x1 = &mut x;
    println!("x1: {}", x1);
}