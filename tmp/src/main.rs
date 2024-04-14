use std::io;

fn main() -> std::io::Result<()> {

    let mut buffer = String::new();
    let stdin = io::stdin(); 
    stdin.read_line(&mut buffer)?;

    println!("{}",first_word(&buffer));

    Ok(())
}

fn first_word(text: &str) -> &str {
    let mut split = text.split(' ');
    split.nth(0).unwrap_or(&text)
}
