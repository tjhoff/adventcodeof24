use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

struct Antenna {
    pos: (isize, isize),
    char: char,
}

fn get_coords_for_line((y, line): (usize, &str)) -> Vec<Antenna> {
    return line
        .chars()
        .enumerate()
        .filter(|(_x, char)| char != &'.')
        .map(|(x, char)| Antenna {
            pos: (x as isize, y as isize),
            char: char,
        })
        .collect();
}

// Diagonal distance between two points. Always in line
fn distance(a: (isize, isize), b: (isize, isize)) -> isize {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs();
}

fn is_in_line(point: (isize, isize), (a, b): ((isize, isize), (isize, isize))) -> bool {
    return (a.0 - point.0) * (point.1 - b.1) == (point.0 - b.0) * (a.1 - point.1);
}

fn has_antinode(point: (isize, isize), group: &Vec<Antenna>) -> bool {
    let antenna_pairs: Vec<(&Antenna, &Antenna)> = group
        .into_iter()
        .tuple_combinations()
        .collect();

    return antenna_pairs
        .into_iter()
        .any(|(a, b)| is_in_line(point, (a.pos, b.pos)))
        // .any(|(a, b)| {
        //     let a_dist = distance(point, a.pos);
        //     let b_dist = distance(point, b.pos);
        //     return a_dist == b_dist * 2 || b_dist == a_dist * 2;
        // });
}

// fn print_all_found((width, height): (usize, usize), group: &Vec<Antenna>) {
//     // print!("{esc}c", esc = 27 as char);
//     let antenna_pairs: Vec<(&Antenna, &Antenna)> = group.into_iter().combinations(2).collect();
//     let antenna_locations: HashSet<(isize, isize)> = group.into_iter().map(|a| a.pos).collect();
//     for row in 0..height {
//         for col in 0..width {
//             if antenna_locations.contains(&(col as isize, row as isize)) {
//                 print!("0");
//             } else if antenna_pairs
//                 .clone()
//                 .into_iter()
//                 .any(|(a, b)| is_in_line((col as isize, row as isize), (a.pos, b.pos)))
//             {
//                 print!("{}", "*");
//             } else {
//                 print!("{}", ".");
//             }
//         }

//         print!("\n");
//     }
// }

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_text.lines().collect();
    let height = lines.len();
    let width = lines[0].chars().count();

    let antennae_groups: HashMap<char, Vec<Antenna>> = file_text
        .lines()
        .enumerate()
        .map(get_coords_for_line)
        .flatten()
        .sorted_by_key(|a| a.char)
        .chunk_by(|a| a.char)
        .into_iter()
        .map(|(c, grp)| (c, grp.collect()))
        .collect();

    let groups: Vec<(char, Vec<Antenna>)> =
        antennae_groups.into_iter().map(|(c, a)| (c, a)).collect();

    let wrange = 0..width;
    let hrange = 0..height;

    let antinodes: HashSet<(&char, (usize, usize))> = wrange
        .into_iter()
        .cartesian_product(hrange)
        .cartesian_product(&groups)
        .filter(|(point, (_c, group))| has_antinode((point.0 as isize, point.1 as isize), group))
        .map(|(point, (c, _g))| (c, point))
        .collect();

    // println!();
    // for (c, grp) in groups {
    //     println!("{c}");
    //     print_all_found((width, height), &grp);

    //     println!();
    // }
    let unique_antinodes : HashSet<(usize, usize)> = antinodes.into_iter().map(|(c, loc)| loc).collect();
    println!("{:?}", unique_antinodes.len());
}
