mod grep;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    recursive: bool,
    text: String,
    file_path: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    grep::run(args)
}
