use regex::{Captures, Regex};

const INPUT: &str = include_str!("../../input/input");

fn main() {
    let filtered_input = parse_input(INPUT);
    let input = parse_input_phase2(filtered_input.as_str());
    println!("Final answer {}", calculate_result(input));
}


fn parse_input(input: &str) -> String {
    let input_string = String::from("do()") + &String::from(input);
    let mut result =String::new();

    let mut do_count = 0;
    // Parse the content of the input file, appending to the result string only the parts
    // delimited by "do()" and "don't()".

    // To do that, we will iterate over the input string and look for "do()" and "don't()".
    // If we find "do()", we will append the content of the input string until we find "don't()".

    let mut i = 0;
        // Check if the current character is the start of a "do()" block
        loop {

            let (do_block, end_index) = find_next_do_block(&input_string[i..]);
            if end_index == 0 || end_index > input_string.len() {
                break;
            }
            result += &do_block;
            do_count += 1;
            i += end_index -1;
        }
  
    result
}

fn find_next_do_block(input: &str) -> (String,usize) {
    
    // Find the next "do()" block in the input string.
    // Return the content of the block and the index of the last character of the block.
    for (i, c) in input.chars().enumerate() {
        if c == 'd' && input.chars().nth(i+1) == Some('o') && input.chars().nth(i+2) == Some('(') && input.chars().nth(i+3) == Some(')') {
            for (j, c) in input.chars().enumerate().skip(i+4) {
                if j == input.len() -1 {
                    println!("End of input");
                    return (input[i..j].to_string(), j);
                }
                if c == 'd' && input.chars().nth(j+1) == Some('o') && input.chars().nth(j+2) == Some('n') && input.chars().nth(j+3) == Some('\'') && input.chars().nth(j+4) == Some('t') && input.chars().nth(j+5) == Some('(') && input.chars().nth(j+6) == Some(')') {
                    println!("Found end of do block");
                    return (input[i..j].to_string(), j+6);
                }
            }
        }
    }
    (String::from(""), 0)
}
fn parse_input_phase2(input: &str) -> Vec<Captures> {
    // Apply a regex to the input
    let re = Regex::new(r"(mul\()(\d{1,3}),(\d{1,3})(\))").unwrap();
    let captures = re.captures_iter(input);
    captures.collect()
}

fn calculate_result(input: Vec<Captures>) -> u32 {
    let mut result = 0;
    for capture in input {
        let a = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let b = capture.get(3).unwrap().as_str().parse::<u32>().unwrap();
        // println!("First number: {}, Second number: {}", a, b);
        // println!("Result: {}", a * b);
        // println!("Matched string: {}", capture.get(0).unwrap().as_str());
        
        result += a * b
    }
    result
}
