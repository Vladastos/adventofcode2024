const INPUT: &str = include_str!("../../input/input");
const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];
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
            for direction in DIRECTIONS {
                result += search_occurences(matrix.clone(), i, j, "XMAS", 0, direction);
            }
        }
    }
    result
}

fn search_occurences(
    matrix: Vec<Vec<char>>,
    i: usize,
    j: usize,
    word: &str,
    word_index: usize,
    direction: (i32, i32),
) -> u32 {
    let mut result = 0;
    // Check if the element matrix[i][j] is equal to the char word[word_index]
    // If not, return 0
    if matrix[i][j] != word.chars().nth(word_index).unwrap() {
        return result;
    }
    if word_index == word.len() - 1 {
        return 1;
    }
    // Increment i and j
    let new_i = (i as i32 + direction.0) as usize;
    let new_j = (j as i32 + direction.1) as usize;

    // If one of the new values is out of bounds, return 0
    if new_i >= matrix.len() || new_j >= matrix[0].len() {
        return result;
    }

    // If word_index is equal to word.len() - 1, return 1
    result = search_occurences(matrix, new_i, new_j, word, word_index + 1, direction);
    result
}
