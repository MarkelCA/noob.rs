// More info: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#preventing-dangling-references-with-lifetimes

// pub fn test2() {
//     let r;
//
//     {
//         let x = 5;
//         r = &x; 
//     }
//
//     // println!("r: {}", r); // This line doesn't compile because of the previous variable lifetimes
// }

pub fn example_longest() {
    println!();
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {x} else {y}
}
