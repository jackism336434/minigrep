use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
}

struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("No enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}
