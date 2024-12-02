const INPUT: &str = include_str!("../../input/input");

fn main() {
    let input = split_input(INPUT);
    println!("{}", solve(&input));
}

fn split_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn solve(input: &Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;
    for line in input {

        let is_safe = is_safe_line(line);

        if is_safe {
            safe_count += 1;
        } else {
            if is_safe_with_dampener(line) {
                safe_count += 1;
            } else {
            }
        }

    }

    safe_count
}

fn is_safe_with_dampener (line: &Vec<i32>) -> bool {
    for i in 0..line.len() {
        // create a vector without the current element
        let mut current_line = line.clone();
        current_line.remove(i);
        if is_safe_line(&current_line) {
            return true;
        }
    }

    false
}


fn is_safe_line(line: &Vec<i32>) -> bool {
    let is_ascending: bool = line[0] < line[1];
    let mut is_safe = true;
    for i in 0..line.len() - 1 {
        
        let current = line[i];
        let next = line[i + 1];

        if current == next {
            is_safe = false;
            break;
        }

        if is_ascending && current > next {
            is_safe = false;
            break;
        }

        if !is_ascending && current < next {
            is_safe = false;
            break;
        }
        if (current - next).abs() > 3 {
            is_safe = false;
            break;
        }
    }

    is_safe
}