use std::process;

use fibonacci::Config;

fn main() {
    println!("This program will take your input, 'n', and return the 'n'th number in the Fibonacci sequence.");
    println!("Please enter 'n' now.");

    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {err}");
        process::exit(1);
    });

    if let Err(e) = fibonacci::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
