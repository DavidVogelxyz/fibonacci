use fibonacci::Config;

fn main() {
    let config = Config::build();

    fibonacci::run(config)
}
