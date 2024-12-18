const INPUT: &str = include_str!("../../input/input");

type Grid = Vec<Vec<(bool, bool)>>;

#[derive(Debug, Clone)]
struct Guard {
    x: usize,
    y: usize,
    direction: (i64, i64),
}

fn main() {
    let (grid, guard) = parse_input(INPUT);
    let result = solve(&grid, guard);
    println!("Result: {}", result);
}

fn solve(grid: &Grid, mut guard: Guard) -> i64 {
    let mut count = 1;
    let mut iteration = 0;
    let mut visited_grid = grid.clone();
    let mut loops_count = 0;

    visited_grid[guard.x][guard.y].1 = true;

    loop {
        iteration += 1;
        // If the guard's position is outside the grid, we're done.

        // Check if the cell ahead is occupied
        let x_ahead = guard.x as i64 + guard.direction.0;
        let y_ahead = guard.y as i64 + guard.direction.1;

        if x_ahead >= grid.len() as i64 || y_ahead >= grid[guard.x].len() as i64 {
            break;
        }

        // If the cell ahead is occupied, turn right
        if x_ahead < 0
            || y_ahead < 0
            || x_ahead >= grid.len() as i64
            || y_ahead >= grid[guard.x].len() as i64
        {
            break;
        }
        if grid[x_ahead as usize][y_ahead as usize].0 {
            guard.direction = match guard.direction {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("Invalid direction"),
            };
        } else {
            // Before moving forward, check if putting an obstacle in the next cell would cause a
            // loop
            if check_for_loop(visited_grid.clone(), guard.clone()) {
                loops_count += 1;
            }
            // If the cell ahead is not occupied, move forward
            guard.x = x_ahead as usize;
            guard.y = y_ahead as usize;

            if !visited_grid[guard.x][guard.y].1 {
                count += 1;
                visited_grid[guard.x][guard.y].1 = true;
            }
        }
    }

    println!("Loops count: {}", loops_count);

    count
}

fn check_for_loop(mut grid: Grid, mut guard: Guard) -> bool {
    let mut number_of_turns = 0;
    let mut steps_since_last_turn = 0;
    let starting_position = (guard.x, guard.y);
    let mut loop_found = false;
    // Put an obstacle in the cell ahead
    let x_ahead = guard.x as i64 + guard.direction.0;
    let y_ahead = guard.y as i64 + guard.direction.1;

    // If the cell ahead has already been visited, we're done
    if grid[x_ahead as usize][y_ahead as usize].1 {
        return false;
    }

    grid[x_ahead as usize][y_ahead as usize].0 = true;

    guard.direction = match guard.direction {
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        _ => panic!("Invalid direction"),
    };

    loop {
        // If the guard's position is outside the grid, we're done.
        let x_ahead = guard.x as i64 + guard.direction.0;
        let y_ahead = guard.y as i64 + guard.direction.1;

        if x_ahead < 0
            || y_ahead < 0
            || x_ahead >= grid.len() as i64
            || y_ahead >= grid[guard.x].len() as i64
        {
            break;
        }

        // If the cell ahead is occupied, turn right
        if grid[x_ahead as usize][y_ahead as usize].0 {
            // Before turning, check if current position is the same as the starting position

            // If the current position is the same as the starting position, we're in a loop
            if guard.x == starting_position.0
                && guard.y == starting_position.1
                && steps_since_last_turn > 0
            {
                loop_found = true;
                break;
            }
            steps_since_last_turn = 0;
            number_of_turns += 1;
            guard.direction = match guard.direction {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("Invalid direction"),
            };
        } else {
            // Before moving forward, check if putting an obstacle in the next cell would cause a
            // loop

            // If the cell ahead is not occupied, move forward
            guard.x = x_ahead as usize;
            guard.y = y_ahead as usize;
            steps_since_last_turn += 1;
        }
        if number_of_turns >= 10000 {
            break;
        }
    }

    loop_found
}

fn parse_input(input: &str) -> (Grid, Guard) {
    let mut grid = Vec::new();
    let mut guard = Guard {
        x: 0,
        y: 0,
        direction: (0, -1),
    };

    for (y, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for (_, c) in line.chars().enumerate() {
            if c == '#' {
                grid[y].push((true, false));
            } else if c == '^' {
                guard.x = y;
                guard.y = grid[y].len();
                guard.direction = (-1, 0);
                grid[y].push((false, false));
            } else {
                grid[y].push((false, false));
            }
        }
    }
    (grid, guard)
}

fn display(grid: &Grid, guard: &Guard) {
    for row in grid {
        for cell in row {
            if cell.1 {
                print!("X");
                continue;
            }
            print!("{}", if cell.0 { '#' } else { '.' });
        }
        println!();
    }
}
