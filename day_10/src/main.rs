use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

fn parse_line(line: &str) -> Vec<Option<u32>> {
    return line.chars().map(|char| char.to_digit(10)).collect();
}

type Coordinate = (usize, usize);
type Lookup = HashMap<Coordinate, Vec<Coordinate>>;
type HeightMap = Vec<Vec<Option<u32>>>;
fn travel_path(coordinate: (usize, usize), cells: HeightMap, paths: Lookup, depth: usize) -> usize {
    let cell_height = cells[coordinate.1][coordinate.0];
    let spaces = " ".repeat(depth);

    println!("{spaces}{coordinate:?} {cell_height:?}");
    if cell_height == Some(9) {
        return 1;
    }

    let Some(paths_at_node) = paths.get(&coordinate) else {
        return 0;
    };
    return paths_at_node
        .into_iter()
        .map(|new_coord| travel_path(new_coord.clone(), cells.clone(), paths.clone(), depth + 1))
        .sum();
}

fn main() {
    let filename = "test1.txt";
    let file_text = read_to_string(filename).unwrap();
    let lines = file_text.lines();

    let data: Vec<Vec<Option<u32>>> = lines.map(|line| parse_line(line)).collect();

    let w = data[0].len() as isize;
    let h = data.len() as isize;

    let mut paths: Lookup = HashMap::new();
    let mut starts: Vec<Coordinate> = Vec::new();

    for (y, row) in data.clone().into_iter().enumerate() {
        for (x, pot_height) in row.into_iter().enumerate() {
            let Some(cell_height) = pot_height else {
                continue;
            };

            if (cell_height == 0) {
                starts.push((x, y));
            }

            let ix: isize = x as isize;
            let iy: isize = y as isize;
            let neighbors: Vec<Coordinate> = [-1, 0, 1]
                .into_iter()
                .cartesian_product([-1, 0, 1])
                .filter_map(|(nx, ny)| {
                    let target_x = ix + nx;
                    let target_y = iy + ny;
                    if target_x < 0 || target_y < 0 || target_x >= w || target_y >= h {
                        return None;
                    }
                    let Some(height) = data[target_y as usize][target_x as usize] else {
                        return None;
                    };
                    println!("{x}, {y}: {height:?} > {cell_height:?}");
                    if height == (cell_height + 1) {
                        return Some((target_x as usize, target_y as usize));
                    }
                    return None;
                })
                .collect();

            paths.insert((x, y), neighbors);
        }
    }
    println!("{data:?}");
    println!("{paths:?}");

    let score: usize = starts
        .into_iter()
        .map(|start| travel_path(start, data.clone(), paths.clone(), 0))
        .sum();

    println!("{score}");
}
