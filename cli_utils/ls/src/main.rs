use std::fs;
use std::env::{self};

fn main() -> std::io::Result<()>{
    let cli_args = env::args();
    ls(cli_args.skip(1))
}


pub fn ls(args: impl Iterator<Item = String>) -> std::io::Result<()> {
    for arg in args {
        let paths = fs::read_dir(arg).unwrap();

        for path in paths {
            println!("{}", path.unwrap().path().display())
        }

    }
    Ok(())
}
