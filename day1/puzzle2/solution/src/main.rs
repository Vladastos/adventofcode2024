

const INPUT: &str = include_str!("../../input/input");
fn main() {
    let (list1,list2) = split_input_to_sorted_lists(INPUT);

    let similarity_score = calculate_similarity_score(&list1, &list2);

    println!("similarity score: {}", similarity_score);

}

fn split_input_to_sorted_lists(input: &str) -> (Vec<i32>, Vec<i32>) {

    let mut iter = input.split("\n");
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            continue
        }
            
        let mut line_iter = line.split("   ");
        let first_str = line_iter.next().unwrap();
        let second_str = line_iter.next().unwrap();

        list1.push(first_str.parse::<i32>().unwrap());
        list2.push(second_str.parse::<i32>().unwrap());        
    }

    list1.sort();
    list2.sort();

    (list1, list2)
    
}

fn calculate_similarity_score(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
let mut similarity_score = 0;
    for i in 0..list1.len() {
        let mut occurrence_count = 0;
        for j in 0..list2.len() {    
            if list1[i] == list2[j] {
                occurrence_count += 1;
            }
        }
        println!("{}: {}", list1[i], occurrence_count);
        similarity_score += occurrence_count * list1[i];
    }
    similarity_score
}