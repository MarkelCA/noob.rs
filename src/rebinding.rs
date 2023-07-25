// More info: https://stackoverflow.com/questions/53235334/in-rust-whats-the-difference-between-shadowing-and-mutability
pub fn mutable() {
    let mut x = 5; // x_0
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}
pub fn stores() {
    let x = 5; // x_0
    let y = &x; // y_0
    println!("The value of y is {}", y);
    let x = 6; // x_1
    println!("The value of y is {}", y);
}

pub fn stores_borrowed() {
    let mut x = 5; // x_0
    let mut y = &x; // y_0
    println!("The value of y is {}", y);
    println!("The value of x is {}", x);
    y = &6;
    println!("The value of y is {}", y);
    println!("The value of x is {}", x);
}
