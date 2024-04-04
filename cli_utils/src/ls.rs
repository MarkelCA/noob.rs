use std::fs;


pub fn run(args: impl Iterator<Item = String>) -> std::io::Result<()> {
    for arg in args {
        let paths = fs::read_dir(arg).unwrap();

        for path in paths {
            println!("{}", path.unwrap().path().display())
        }

    }
    Ok(())
}
