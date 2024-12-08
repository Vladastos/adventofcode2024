const INPUT: &str = include_str!("../../input/test");
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

    println!("{:?}", invalid_lines);

    // Sort the invalid lines
    let invalid_lines = sort_invalid_lines(invalid_lines, rules);
    return sum_central_elements(invalid_lines);
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
    // Get a topological order of the rules
    let order = get_topological_order(rules);

    // Sort the elements of each line based on the size of the
    // associated vector in the topological order
    let mut lines = lines;
    for line in &mut lines {
        line.sort_by(|a, b| {
            order
                .iter()
                .position(|x| x == a)
                .cmp(&order.iter().position(|x| x == b))
        });
    }
    return lines;
}

fn get_topological_order(rules: Vec<(i32, i32)>) -> Vec<i32> {
    // Generate the adjacency list from the rules
    let graph = generate_graph(rules);

    let mut ordered_graph: Vec<(i32, Vec<i32>)> = vec![];
    for (key, value) in &graph {
        ordered_graph.push((*key, value.clone()));
    }

    // Order the keys of the hashMap by the size of the associated vector
    ordered_graph.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    ordered_graph.iter().for_each(|x| println!("{:?}", x));
    let topological_order = ordered_graph.iter().map(|x| x.0).collect();

    println!("{:?}", topological_order);
    return topological_order;
}

fn generate_graph(rules: Vec<(i32, i32)>) -> Vec<(i32, Vec<i32>)> {
    let mut graph: Vec<(i32, Vec<i32>)> = vec![];

    for rule in rules {
        if !graph.iter().any(|x| x.0 == rule.0) {
            graph.push((rule.0, vec![]));
        }
        if !graph.iter().any(|x| x.0 == rule.1) {
            graph.push((rule.1, vec![]));
        }
        graph
            .iter_mut()
            .find(|x| x.0 == rule.0)
            .unwrap()
            .1
            .push(rule.1);
    }
    return graph;
}
