use rand::{self, Rng};
use std::cmp::Ordering;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = rand::thread_rng().gen_range(1..=10);
    let stdin = io::stdin();

    let mut buffer = String::new();
    
    loop {
        stdin.read_line(&mut buffer)?;
        let input = buffer.trim();

        println!("The string: {}", input);

        if input == "exit" {
            break;
        }

        let y = input.parse::<i32>().unwrap(); 

        let result = match x.cmp(&y) {
            Ordering::Less => "lower",
            Ordering::Greater => "greater",
            Ordering::Equal => break,
        };

        println!("The number is {}, try again.", result);

        buffer.clear();
    }

    println!("Well done!");

    Ok(())
}
