use std::io;
use std::process;

pub struct Config {
    pub index: u8,
}

impl Config {
    pub fn build() -> Config {
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
                println!("The {index}th number in the Fibonacci sequence is so large that it cannot be stored in a 128-bit integer.");
                println!("The largest 'n' that the program can output is the 186th number in the sequence.");
                println!("Please input another number between 0 and 186.");
                continue;
            }

            return Config{ index };
        }
    }
}

pub fn run(config: Config) {
    if config.index == 0 {
        println!("The {}th number in the Fibonacci sequence is: 0", config.index);
        process::exit(0);
    }

    let fib = fib_count(config.index);
    let fib = fib_commas(fib);

    if config.index == 1 {
        println!("The {}st number in the Fibonacci sequence is: {}", config.index, fib);
    } else if config.index == 2 {
        println!("The {}nd number in the Fibonacci sequence is: {}", config.index, fib);
    } else if config.index == 3 {
        println!("The {}rd number in the Fibonacci sequence is: {}", config.index, fib);
    } else {
        println!("The {}th number in the Fibonacci sequence is: {}", config.index, fib);
    }
}

pub fn fib_count(index: u8) -> u128 {
    let mut fib: u128 = 1;
    let mut l = 0;
    let mut r = 1;

    for _ in 2..= index {
        fib = l + r;
        l = r;
        r = fib;
    }

    fib
}

pub fn fib_commas(fib: u128) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_one_is_one() {
        let index = 1;

        assert_eq!(1, fib_count(index));
    }

    #[test]
    fn position_two_is_one() {
        let index = 2;

        assert_eq!(1, fib_count(index));
    }

    #[test]
    fn position_three_is_two() {
        let index = 3;

        assert_eq!(2, fib_count(index));
    }

    #[test]
    fn position_four_is_three() {
        let index = 4;

        assert_eq!(3, fib_count(index));
    }

    #[test]
    fn position_five_is_five() {
        let index = 5;

        assert_eq!(5, fib_count(index));
    }

    #[test]
    fn position_six_is_eight() {
        let index = 6;

        assert_eq!(8, fib_count(index));
    }

    #[test]
    fn position_seven_is_thirteen() {
        let index = 7;

        assert_eq!(13, fib_count(index));
    }

    #[test]
    fn commas_test_one() {
        let index = 42;
        let fib = fib_count(index);

        assert_eq!("267,914,296", fib_commas(fib));
    }

    #[test]
    fn commas_test_two() {
        let index = 69;
        let fib = fib_count(index);

        assert_eq!("117,669,030,460,994", fib_commas(fib));
    }
}
