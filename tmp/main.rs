#[derive(Debug)]
struct Bob {
    numbers: Vec<i32>
}

impl Bob {
    fn new() -> Bob {
        Bob { numbers: Vec::new() }
    }
}

fn func_1() {
    let mut a = Bob::new();
    let a1 = &mut a;
    // let a2 = &a;
    func_2(a1);
    // func_3(a2);
    println!("{:?}", a);
}

fn func_2(bob: &mut Bob) {
    bob.numbers.push(66);
    println!("{}", bob.numbers[0]);
}

fn func_3(bob: &Bob) {
    println!("{}", bob.numbers[1]);
}


fn main() {
    func_1();
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

