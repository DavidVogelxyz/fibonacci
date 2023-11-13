use std::io;

fn main() {
    fibonacci();
}

fn fibonacci() {
    println!("This program will take your input, 'n', and return the 'n'th number in the Fibonacci sequence.");

    println!("Please enter 'n' now.");

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input.");

        let user_input: u8 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number between 0 and 255.");
                continue;
            }
        };

        if user_input == 0 {
            println!("The {user_input}th number in the Fibonacci sequence is: 0");
            break;
            //continue;
        } else if user_input > 186 {
            println!("The {user_input}th number in the Fibonacci sequence is so large that it cannot be stored in a 128-bit integer.");
            println!("The largest 'n' that the program can output is the 186th number in the sequence.");
            println!("Please input another number between 0 and 186.");
            continue;
        }

        let user_input = user_input - 1;

        let mut count = 0;
        let mut fib: u128 = 1;
        let mut l = 0;
        let mut r = 1;

        while count < user_input {
            fib = l + r;
            l = r;
            r = fib;
            count += 1;
            // to watch the program iterate, uncomment the following:
            //println!("{fib}");
        }

        let user_input = user_input + 1;

        let fib = fib.to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",");

        if user_input == 1 {
            println!("The {user_input}st number in the Fibonacci sequence is: {fib}");
        } else if user_input == 2 {
            println!("The {user_input}nd number in the Fibonacci sequence is: {fib}");
        } else if user_input == 3 {
            println!("The {user_input}rd number in the Fibonacci sequence is: {fib}");
        } else {
            println!("The {user_input}th number in the Fibonacci sequence is: {fib}");
        }

        break;
        //continue;
    }
}
