use std::fs;
use std::env;

fn main() {
    let path = "./languages-on-github.yml";
    let contents = read_file(path);
    println!("Contents: {}", contents);
}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(lines) => lines,
        Err(_) => String::new(),
    }
}