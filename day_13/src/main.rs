use std::fs::read_to_string;

use itertools::Itertools;
use regex::Regex;

const offset: isize = 10000000000000;
// const offset: isize = 0;
fn parse_coordinates(prize: &str) -> (isize, isize) {
    //Prize: X=8400, Y=5400
    let regex = Regex::new(r".*X.(?<x>\d+), Y.(?<y>\d+)").unwrap();
    let coords: Vec<(isize, isize)> = regex
        .captures_iter(prize)
        .map(|x| {
            (
                x["x"].parse::<isize>().unwrap(),
                x["y"].parse::<isize>().unwrap(),
            )
        })
        .collect();

    return coords[0];
}

fn solve_with_math(
    (apx, apy): (isize, isize),
    (bpx, bpy): (isize, isize),
    (cx, cy): (isize, isize),
) -> Option<(isize, isize)> {
    let (apx, apy) = (apx as f64, apy as f64);
    let (bpx, bpy) = (bpx as f64, bpy as f64);
    let (cx, cy) = (cx as f64, cy as f64);

    // gross!
    let g = (cx * bpy) / bpx;
    let h = (apx * bpy) / bpx;
    let v = cy - g;
    let b = apy - h;
    let a_presses = v / b;
    let b_presses = (cx / bpx) - ((a_presses * apx) / bpx);
    if (a_presses.fract() > 0.01 || b_presses.fract() > 0.01) {
        return None;
    }
    // println!("a: {apx},{apy} c: {cx},{cy} {g} {h} {v}/{b} -> {a_presses} {b_presses}");
    return Some((a_presses as isize, b_presses as isize));
}

fn solve(lines: Vec<&str>) -> isize {
    let slice: &[&str] = lines.as_slice();

    let button_a = parse_coordinates(slice[0]);
    let button_b = parse_coordinates(slice[1]);
    let (px, py) = parse_coordinates(slice[2]);
    let prize = (px + offset, py + offset);
    let Some((a_presses, b_presses)) = solve_with_math(button_a, button_b, prize) else {
        return 0;
    };
    let mut math_result = 0;
    if (
        a_presses * button_a.0 + b_presses * button_b.0,
        a_presses * button_a.1 + b_presses * button_b.1,
    ) == prize
    {
        println!("{a_presses} {b_presses}");
        math_result = a_presses * 3 + b_presses;
    } else {
        println!("Invalid result??");
    }
    return math_result;
    //     let a_pushes: std::ops::Range<usize> = 0..100;
    //     let b_pushes: std::ops::Range<usize> = 0..100;
    //     return a_pushes
    //         .cartesian_product(b_pushes)
    //         .filter(|(a, b)| is_solution(mult(button_a, *a), mult(button_b, *b), prize))
    //         .map(|(a, b)| a * 3 + b)
    //         .inspect()
    //         .sum();
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();
    let solution: isize = file_text
        .lines()
        .chunks(4)
        .into_iter()
        .map(|chunk| solve(chunk.collect()))
        .sum();
    println!("{solution}");
}
