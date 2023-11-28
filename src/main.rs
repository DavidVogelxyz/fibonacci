use fibonacci::Config;

fn main() {
    println!("This program will take your input, 'n', and return the 'n'th number in the Fibonacci sequence.");
    println!("Please enter 'n' now.");

    let config = Config::build();

    fibonacci::run(config)
}
