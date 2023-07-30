fn print_out(item: &Vec<u32>) {
    for i in item {
        println!("{}", i);
    }
}

pub fn test1() {
    let item = vec![1,2,3];

    print_out(&item);
    print_out();
    print_out(&item);
}

#[derive(Debug)]
struct Foo {}

pub fn test2() {
    let mut vector = vec![Foo{}, Foo{}, Foo{}];
    let last_foo = vector.last();

    // vector.pop(); // Can't borrow vector as immutable (you can only have one mutable reference to a particular piece of data in a particular scope)


    println!("Last foo {:?}", last_foo);
}

#[derive(Debug)]
struct Foo2 {
    value: usize,
}

fn reverse_and_print(foo: &mut Vec<Foo2>) {
    foo.reverse();
    for f in foo.iter() {
        println!("{:?}", f)
    }
}

fn print_reversed(foo: &Vec<Foo2>) {
    for f in foo.iter().rev() {
        println!("{:?}", f)
    }
}

pub fn test3() {
    let mut vector = vec![Foo2{value: 1}, Foo2{value: 2}, Foo2{value: 3}];
    reverse_and_print(&mut vector);
    reverse_and_print(&mut vector);
}
