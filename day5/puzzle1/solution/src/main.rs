const INPUT: &str = include_str!("../../input/input");
fn main() {
    let (rules, lines) = process_input(INPUT);

    let valid_lines = filter_lines(rules, lines);

    let sum = sum_central_elements(valid_lines);

    println!("Sum: {:?}", sum);
}

fn process_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules, lines) = input.split_once("\n\n").unwrap();

    // Split the rules
    let processed_rules: Vec<(i32, i32)> = rules
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .collect();

    // Split the lines
    let processed_lines: Vec<Vec<i32>> = lines
        .lines()
        .map(|line| {
            let res = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            res
        })
        .collect();

    return (processed_rules, processed_lines);
}

fn filter_lines(rules: Vec<(i32, i32)>, lines: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut valid_lines: Vec<Vec<i32>> = vec![];
    for line in &lines {
        let mut is_valid = true;
        for rule in &rules {
            if !is_valid {
                break;
            }

            let mut first_index: Option<usize> = None;
            let mut second_index: Option<usize> = None;

            // get the indexes of the numbers that match the rule
            if line.contains(&rule.0) {
                first_index = line.iter().position(|&x| x == rule.0);
            }

            if line.contains(&rule.1) {
                second_index = line.iter().position(|&x| x == rule.1);
            }

            // if both indexes are found, compare the indexes
            if first_index.is_some() && second_index.is_some() {
                if first_index.unwrap() > second_index.unwrap() {
                    println!("Line is not valid: {:?}", line);
                    is_valid = false;
                }
            }
        }

        if is_valid {
            valid_lines.push(line.clone());
        }
    }
    return valid_lines;
}

fn sum_central_elements(lines: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for line in lines {
        // Find the middle element
        let middle = line.len() / 2;
        sum += line[middle];
    }
    return sum;
}
