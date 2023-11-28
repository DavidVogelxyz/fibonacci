use std::error::Error;
use std::io;
use std::process;

pub struct Config {
    pub index: u8,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("User input should be a valid string.");

            let index: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number between 0 and 255.");
                    continue;
                }
            };

            if index > 186 {
                println!("The {input}th number in the Fibonacci sequence is so large that it cannot be stored in a 128-bit integer.");
                println!("The largest 'n' that the program can output is the 186th number in the sequence.");
                println!("Please input another number between 0 and 186.");
                continue;
            }

            return Ok(Config { index });
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.index == 0 {
        println!("The {}th number in the Fibonacci sequence is: 0", config.index);
        process::exit(0);
    }

    let fib = fib_count(config.index);

    if config.index == 1 {
        println!("The {}st number in the Fibonacci sequence is: {}", config.index, fib);
    } else if config.index == 2 {
        println!("The {}nd number in the Fibonacci sequence is: {}", config.index, fib);
    } else if config.index == 3 {
        println!("The {}rd number in the Fibonacci sequence is: {}", config.index, fib);
    } else {
        println!("The {}th number in the Fibonacci sequence is: {}", config.index, fib);
    }

    Ok(())
}

pub fn fib_count(user_input: u8) -> String {
    let mut fib: u128 = 1;
    let mut l = 0;
    let mut r = 1;

    for _ in 2..= user_input {
        fib = l + r;
        l = r;
        r = fib;
    }

    let fib = fib.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",");

    fib
}
