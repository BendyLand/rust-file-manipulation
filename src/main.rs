use std::fs;

fn main() {
    let path = "./languages-on-github.yml";
    let contents = read_file(path);
    let lines = split_into_lines(contents);
    let blocks = construct_blocks(lines);
    let formatted_blocks = blocks
        .into_iter()
        .map(|block| format_block(&block))
        .collect::<Vec<String>>();
    println!("{}", formatted_blocks[0]);
}

fn format_block(block: &String) -> String {
    let mut tokens = block
        .split_whitespace()
        .collect::<Vec<&str>>();
    let language = extract_language(&mut tokens);
    let formatted_details = format_details(&tokens);
    format!("{}{}", language, formatted_details)
}

fn extract_language(tokens: &mut Vec<&str>) -> String {
    let mut result = String::new();
    if tokens[0].contains(':') {
        result = tokens[0].to_string();
        *tokens = tokens[1..].to_vec();
    }
    else {
        result = format!("{} {}", tokens[0], tokens[1]);
        *tokens = tokens[2..].to_vec();
    }
    result
}

fn format_details(tokens: &[&str]) -> String {
    let mut pairs = Vec::<String>::new();
    let mut pair = String::new();
    for token in tokens {
        if token.contains(':') {
            pairs.push(pair);
            pair = String::from("");
            pair += format!("{} ", token).as_str();
        }
        else {
            pair += format!("{} ", token).as_str();
        }
    }
    pairs.join("\n")
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