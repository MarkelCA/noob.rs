use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    files: Vec<String>
}

fn main()-> std::io::Result<()> {
    let args = Args::parse();

    for f in args.files {
        print_file(f)?
    }

    Ok(())
}

fn print_file(filename: String) -> std::io::Result<()> {
    let bytes = std::fs::read(filename).unwrap();
    
    for bytes in bytes.chunks(50) {
        print!("{}", String::from_utf8_lossy(bytes));

    }

    Ok(())
}
