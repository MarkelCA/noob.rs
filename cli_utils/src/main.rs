mod cat;
mod ls;
use std::env::{self, args};

fn main() -> std::io::Result<()>{
    let cli_args = env::args();
    let command = args().nth(1).unwrap();

    match command.as_str() {
        "cat" => cat::run(cli_args.skip(2)), // skip the first two arguments (the command and the filename)
        "ls" => ls::run(cli_args.skip(2)),
        _ => {
            println!("{}: command not found", command);
            Ok(())
        }
    }
}
