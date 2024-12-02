use std::f32::consts::E;

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
        // compare each element with the next
        let mut is_safe = true;
        let is_ascending: bool = line[0] < line[1];

        for i in 0..line.len() - 1 {
            if i + 1 == line.len() {
                break;
            }
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
        
        if is_safe {
            println!("{:?} is safe", line);
            safe_count += 1;
        } else {
            println!("{:?} is NOT safe", line);
        }
    }

    safe_count
}
