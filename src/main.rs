use std::fs;

fn main() {
    let path = "./languages-on-github.yml";
    let contents = read_file(path);
    let lines = split_into_lines(contents);
    let blocks = construct_blocks(lines);
    println!("{}", blocks[0]);
}

fn construct_blocks(lines: Vec<String>) -> Vec<String> {
    let mut block = String::new();
    let mut blocks = Vec::<String>::new();
    for line in lines {
        if line.chars().nth(0) != Some(' ') {
            blocks.push(block);
            block = String::from("");
            block += &line;
        }
        else {
            block += &line;
        }
    }
    blocks
        .into_iter()
        .filter(|block| block.len() > 10)
        .collect::<Vec<String>>()
}

fn split_into_lines(file: String) -> Vec<String> {
    let contents: Vec<&str> = file.split("\n").collect();
    let language_contents = contents
        .into_iter()
        .filter(|line| line.chars().nth(0) != Some('#'))
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    language_contents
}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(lines) => lines,
        Err(_)    => String::new(),
    }
}