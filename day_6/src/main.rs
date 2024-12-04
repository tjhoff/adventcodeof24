use std::{
    collections::HashSet,
    fs::read_to_string,
    io::{self, Write},
};

use colored::Colorize;

fn get_coords_for_line(line: &str) -> Vec<usize> {
    return line
        .chars()
        .enumerate()
        .filter(|(_x, char)| char == &'#')
        .map(|(x, _char)| x)
        .collect();
}
fn increment_direction((x, y): (usize, usize), direction: usize) -> Option<(usize, usize)> {
    return match direction {
        0 => {
            if y > 0 {
                Some((x + 0, y - 1))
            } else {
                None
            }
        }
        1 => Some((x + 1, y + 0)),
        2 => Some((x + 0, y + 1)),
        3 => {
            if x > 0 {
                Some((x - 1, y + 0))
            } else {
                None
            }
        }
        _ => Some((x, y)),
    };
}

fn print_board(
    (width, height): (usize, usize),
    current_position: (usize, usize),
    previous_steps: &HashSet<(usize, usize)>,
    obstacles: &HashSet<(usize, usize)>,
    new_obstacle: (usize, usize),
) {
    let mut buffer: Vec<u8> = Vec::new();
    print!("{esc}c", esc = 27 as char);
    for row in 0..height {
        for col in 0..width {
            let pos = (col, row);
            if pos == current_position {
                buffer.extend_from_slice(format!("{}", "o".red()).as_bytes());
            } else if obstacles.contains(&pos) {
                buffer.extend_from_slice(format!("{}", "#".blue()).as_bytes())
            } else if previous_steps.contains(&pos) {
                buffer.extend_from_slice(format!("{}", ".".white()).as_bytes())
            } else if pos == new_obstacle {
                buffer.extend_from_slice(format!("{}", "#".green()).as_bytes())
            } else {
                buffer.extend_from_slice(format!("{}", ".".truecolor(50, 50, 50)).as_bytes());
            }
        }
        buffer.extend_from_slice("\n".as_bytes());
    }

    io::stdout().write(buffer.as_slice()).unwrap();
}

fn print_all_found((width, height): (usize, usize), obstacles: &HashSet<(usize, usize)>) {
    print!("{esc}c", esc = 27 as char);
    for row in 0..height {
        for col in 0..width {
            if (obstacles.contains(&(col, row))) {
                print!("{}", "#".green());
            } else {
                print!("{}", ".");
            }
        }

        print!("\n");
    }
}
fn print_found((width, height): (usize, usize), obstacle: (usize, usize)) {
    print!("{esc}c", esc = 27 as char);
    for row in 0..height {
        for col in 0..width {
            if (obstacle == (col, row)) {
                print!("{}", "#".white());
            } else {
                print!("{}", "✓".green());
            }
        }

        print!("\n");
    }
}

fn print_fail((width, height): (usize, usize), obstacle: (usize, usize)) {
    print!("{esc}c", esc = 27 as char);
    for row in 0..height {
        for col in 0..width {
            if (obstacle == (col, row)) {
                print!("{}", "#".white());
            } else {
                print!("{}", "x".red());
            }
        }
        print!("\n");
    }
}

fn walk(
    (x, y): (usize, usize),
    obstacle_coords: &HashSet<(usize, usize)>,
    (width, height): (usize, usize),
) -> Option<Vec<(usize, usize, usize)>> {
    let mut guard_pos = (x, y);
    let mut direction: usize = 0;
    let mut steps_with_direction: Vec<(usize, usize, usize)> = Vec::new();
    // steps_with_direction.push((guard_pos.0, guard_pos.1, direction));
    loop {
        let p_w_d = (guard_pos.0, guard_pos.1, direction);

        if steps_with_direction.contains(&p_w_d) {
            // println!("Looped at {:?}", p_w_d);
            return None;
        }
        steps_with_direction.push(p_w_d);

        let Some(new_position) = increment_direction(guard_pos, direction) else {
            break;
        };
        if obstacle_coords.contains(&new_position) {
            direction = (direction + 1) % 4;
            continue;
        }

        guard_pos = new_position;

        if direction == 1 && guard_pos.0 >= width {
            break;
        }
        if direction == 2 && guard_pos.1 >= height {
            break;
        }
    }
    return Some(steps_with_direction);
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_text.lines().collect();
    let height = lines.len();
    let width = lines[0].chars().count();

    let (x, y, _) = file_text
        .clone()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let chars: Vec<(usize, usize, char)> = line
                .chars()
                .enumerate()
                .map(|(x, char)| (x, y, char))
                .collect();
            return chars;
        })
        .flatten()
        .find(|(_x, _y, char)| char == &'^')
        .unwrap();

    let obstacle_coords: HashSet<(usize, usize)> = file_text
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let line_obstacles: Vec<(usize, usize)> = get_coords_for_line(line)
                .into_iter()
                .map(|x_coord| (x_coord, y))
                .collect();
            return line_obstacles;
        })
        .flatten()
        .collect();

    let Some(steps_with_direction) = walk((x, y), &obstacle_coords, (width, height)) else {
        return;
    };
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();

    let step_count = steps_with_direction.len();

    println!("steps to exit: {step_count}");
    // println!("steps: {steps_with_direction:?}");

    for (index, (step_x, step_y, _d)) in steps_with_direction.clone().into_iter().enumerate() {
        if index % 100 == 0 {
            println!("{index}/{step_count} {}", obstacles.len());
        }
        let mut new_obstacles = obstacle_coords.clone();
        new_obstacles.insert((step_x, step_y));
        let result = walk((x, y), &new_obstacles, (width, height));

        match result {
            Some(_x) => continue,
            None => obstacles.insert((step_x, step_y)),
        };
    }

    // print_all_found((width, height), &obstacles);

    obstacles.remove(&(x, y));

    let unique_locations: HashSet<(usize, usize)> = steps_with_direction
        .into_iter()
        .map(|(x, y, _d)| (x, y))
        .collect();
    println!("{:?}", (x, y));
    println!("{} {}", unique_locations.len(), obstacles.len());
}
