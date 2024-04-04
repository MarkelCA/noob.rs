mod cat;
use std::env::{self, args};

fn main() -> std::io::Result<()>{
    let myargs = env::args();
    let command = args().nth(1).unwrap();

    match command.as_str() {
        "cat" => cat::run(myargs.skip(2)), // skip the first two arguments (the command and the filename)
        _ => {
            println!("{}: command not found", command);
            Ok(())
        }
    }
}
