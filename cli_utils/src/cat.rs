use std::env::Args;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn run(args: Args) -> io::Result<()> {
    for arg in args.skip(1) {
        cat(arg)?;
    }

    Ok(())
}

fn cat(filename: String) -> io::Result<()> {

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
