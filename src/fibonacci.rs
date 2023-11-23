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
