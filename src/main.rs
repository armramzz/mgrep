use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Err(e) = mgrep::run(args) {
        println!("Error happened: {e}");
        process::exit(1);
    }
}

