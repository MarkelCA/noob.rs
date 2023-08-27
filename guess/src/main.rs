use rand::{self, Rng};
use std::cmp::Ordering;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solution = rand::thread_rng().gen_range(1..=10);
    let stdin = io::stdin();

    let mut buffer = String::new();

    loop {
        stdin.read_line(&mut buffer)?;
        let input = buffer.trim();

        if input == "exit" {
            break;
        }

        let guess = match input.parse::<i32>() {
            Ok(x) => x,
            Err(_) => {
                println!("Number it's not valid, try again");
                buffer.clear();
                continue;
            }
        };

        let result = match solution.cmp(&guess) {
            Ordering::Less => "lower",
            Ordering::Greater => "greater",
            Ordering::Equal => {
                println!("Well done!");
                break;
            }
        };
        println!("The number is {}, try again.", result);
        buffer.clear();
    }

    Ok(())
}
