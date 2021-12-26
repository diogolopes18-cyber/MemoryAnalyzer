pub fn shadowing() {
    let x = 1;
    println!("This is the value of x: {}", x);

    let x = 2;
    println!("This is the value of x: {}", x);
}