use regex::{Captures, Regex};

const INPUT: &str = include_str!("../../input/input");

const REGEX: &str = r"/(mul\()(\d{1,3}),(\d{1,3})(\))/gm";
fn main() {
    let input = parse_input(INPUT);
    println!("Matches found: {}", input.len());
    println!("{}", calculate_result(input));
}


fn parse_input(input: &str) -> Vec<Captures> {
    // Apply a regex to the input
    let re = Regex::new(r"(mul\()(\d{1,3}),(\d{1,3})(\))").unwrap();
    let captures = re.captures_iter(input);
    captures.collect()
}

fn calculate_result(input: Vec<Captures>) -> u32 {
    let mut result = 0;
    for capture in input {
        println!("Matched string: {}", capture.get(0).unwrap().as_str());
        let a = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let b = capture.get(3).unwrap().as_str().parse::<u32>().unwrap();
        println!("First number: {}, Second number: {}", a, b);
        println!("Result: {}", a * b);
        result += a * b
    }
    result
}
