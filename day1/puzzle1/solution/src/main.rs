

const INPUT: &str = include_str!("../../input/input");
fn main() {
    let (list1,list2) = split_input_to_sorted_lists(INPUT);

    let difference = calculate_difference(&list1, &list2);
    println!("difference: {}", difference);

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

fn calculate_difference(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut difference = 0;
    for i in 0..list1.len() {
        let line_difference = (list1[i] - list2[i]).abs();
        difference += line_difference;
        println!("element 1: {}, element 2: {}, difference: {}", list1[i], list2[i], line_difference);
    }
    difference
}