const INPUT: &str = include_str!("../../input/input");
fn main() {
    let matrix = parse_input(INPUT);
    let result = calculate_result(matrix);

    println!("Result: {}", result);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn calculate_result(matrix: Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            result += search_x_mas(matrix.clone(), i, j)
        }
    }
    result
}

fn search_x_mas(matrix: Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut result = 0;
    let mut first_diagonal = false;
    let mut second_diagonal = false;
    if matrix[i][j] != 'A' {
        return result;
    }

    // check if diagonally adjacent elements are out of bounds
    if i < 1 || j < 1 || i >= matrix.len() - 1 || j >= matrix[0].len() - 1 {
        return result;
    }

    if matrix[i - 1][j - 1] == 'M' {
        if matrix[i + 1][j + 1] == 'S' {
            first_diagonal = true;
        }
    }

    if matrix[i - 1][j - 1] == 'S' {
        if matrix[i + 1][j + 1] == 'M' {
            first_diagonal = true;
        }
    }

    if matrix[i - 1][j + 1] == 'M' {
        if matrix[i + 1][j - 1] == 'S' {
            second_diagonal = true;
        }
    }

    if matrix[i - 1][j + 1] == 'S' {
        if matrix[i + 1][j - 1] == 'M' {
            second_diagonal = true;
        }
    }

    if first_diagonal && second_diagonal {
        result = 1;
    }
    result
}
