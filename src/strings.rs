pub fn test() {
    let text: String = String::from("blablabla");

    print!("{}", text);
}

pub fn mutate() {
    // String (owned, mutable)
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s); // Output: "Hello, World!"

    // String slice (borrowed, immutable)
    let string_literal = "Hello, World!";
    let slice: &str = &string_literal[0..5];

    println!("{}", slice); // Output: "Hello"
}
