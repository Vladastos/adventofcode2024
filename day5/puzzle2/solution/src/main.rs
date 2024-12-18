use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/input");
fn main() {
    let (rules, lines) = parse_input(INPUT);

    let result = solve(rules, lines);

    println!("Result: {:?}", result);
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
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

fn solve(rules: Vec<(i32, i32)>, lines: Vec<Vec<i32>>) -> i32 {

    // Select the invalid lines from the input
    let invalid_lines = filter_lines(rules.clone(), lines);

    // Sort the invalid lines
    let sorted_lines = sort_invalid_lines(invalid_lines, rules);
    return sum_central_elements(sorted_lines);

}

fn filter_lines(rules: Vec<(i32, i32)>, lines: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut invalid_lines: Vec<Vec<i32>> = vec![];
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
                    is_valid = false;
                }
            }
        }

        if !is_valid {
            invalid_lines.push(line.clone());
        }
    }
    return invalid_lines;
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

fn sort_invalid_lines(lines: Vec<Vec<i32>>, rules: Vec<(i32, i32)>) -> Vec<Vec<i32>> {
    
    let mut sorted_lines: Vec<Vec<i32>> = vec![];

    for mut line in lines {
        // Select only the relevant rules
        let mut relevant_rules: Vec<(i32, i32)> = vec![];
        for rule in &rules {
            if line.contains(&rule.0) && line.contains(&rule.1) {
                relevant_rules.push(*rule);
            }
            let graph = generate_graph(relevant_rules.clone());
            let transitive_graph = get_transitive_graph(graph.clone());
            let topological_order = get_topological_order(transitive_graph);
            
            // Sort the line
            line.sort_by(|a, b| topological_order.iter().position(|&x| x == *a).cmp(&topological_order.iter().position(|&x| x == *b)));
        }
        sorted_lines.push(line);
    }
    return sorted_lines;
}

fn generate_graph(rules: Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rules {
        if !graph.iter().any(|x| *x.0 == rule.0) {
            graph.insert(rule.0, vec![]);
        }
        if !graph.iter().any(|x| *x.0 == rule.1) {}
        graph
            .iter_mut()
            .find(|x| *x.0 == rule.0)
            .unwrap()
            .1
            .push(rule.1);
    }
    return graph;
}

fn get_transitive_graph(graph: HashMap<i32, Vec<i32>>) -> HashMap<i32, Vec<i32>> {

    let mut transitive_graph: HashMap<i32, Vec<i32>> = graph.clone();
    let mut result_graph: HashMap<i32, Vec<i32>> = transitive_graph.clone();

    loop{
        let mut has_changed = false;
        for (key, value) in transitive_graph.clone() {
        
            for element in value {
                if !result_graph.contains_key(&element) {
                 
                    result_graph.insert(element, vec![]);
                }

                for transitive_element in transitive_graph.get(&element).unwrap_or(&vec![]) {
         
                    if !result_graph.get(&key).unwrap_or(&vec![]).contains(transitive_element) {
                        has_changed = true;
                        result_graph.get_mut(&key).unwrap().push(*transitive_element);
                        
                    }
    
                }
            }
            transitive_graph = result_graph.clone();
        }


        if !has_changed {
            break;
        }
    }
    return transitive_graph;
}

fn get_topological_order(graph: HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let binding = graph.clone();
    let mut ordered_graph =binding.iter().collect::<Vec<(&i32, &Vec<i32>)>>();
    ordered_graph.sort_by(|a,b| a.1.len().cmp(&b.1.len()));
    let mut order: Vec<i32> = vec![];
    for (key, _) in ordered_graph {
        order.push(*key);
    }
    order
}