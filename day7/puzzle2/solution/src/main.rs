const INPUT: &str = include_str!("../../input/input");
fn main() {
    let input = parse_input(INPUT);

    let result = solve(input);

    println!("{:?}", result);
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(':').unwrap();
            let key = key.parse().unwrap();
            let value = value
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (key, value)
        })
        .collect()
}

fn solve(input: Vec<(i64, Vec<i64>)>) -> i64 {
    let mut sum = 0;
    for (key, value) in input {
        if is_value_obtainable(key, value) {
            sum += key;
        }
    }

    sum
}

fn is_value_obtainable(key: i64, value: Vec<i64>) -> bool {
    if value.len() == 2 {
        if value[0] + value[1] == key {
            return true;
        }
        if value[0] * value[1] == key {
            return true;
        }

        if concat_numbers(value[0], value[1]) == key {
            return true;
        }

        return false;
    }

    // Pop the first element
    let first = value[0];

    let mut rest_addition = value[1..].to_vec();
    let mut rest_multiplication = value[1..].to_vec();
    let mut rest_concat = value[1..].to_vec();
    rest_addition[0] += first;
    rest_multiplication[0] *= first;
    rest_concat[0] = concat_numbers(first, rest_concat[0]);

    if is_value_obtainable(key, rest_addition) {
        return true;
    }

    if is_value_obtainable(key, rest_multiplication) {
        return true;
    }

    if is_value_obtainable(key, rest_concat) {
        return true;
    }
    false
}

fn concat_numbers(first: i64, second: i64) -> i64 {
    let digits = format!("{}{}", first, second);
    let result = digits.parse::<i64>().unwrap();
    result
}
