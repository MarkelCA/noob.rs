#[derive(Debug)]
struct Bob {
    numbers: Vec<i32>
}

impl Bob {
    fn new() -> Bob {
        Bob { numbers: Vec::new() }
    }
}


fn main() {
    let x = 5;
    let y = x;
    println!("{}", x);


    // let a = Bob::new();
    // let b = a;
    // println!("{:?}", a);
}
//
// fn main2() {
//     let mut x = 5;
//     let mut y = &x;
//
//
//     println!("{:p}", &x);
//     println!("{:p}", &y);
// }
/


fn main() {
    let a = 1;
    let b = 2;
    let b = 3;

    println!("{}", a);
    println!("{}", b);
    println!("{}", b);
}
