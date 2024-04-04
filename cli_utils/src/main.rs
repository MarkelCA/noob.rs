mod cat;
use std::env;


fn main() -> std::io::Result<()>{
    cat::run(env::args())
}
