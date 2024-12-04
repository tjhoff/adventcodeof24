use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string, iter::zip};

fn extract_numbers(line: &str) -> (i32, i32) {
    let mut iter = line.split_whitespace();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    return (first.parse().unwrap(), second.parse().unwrap());
}

fn main() {
    let filename = "test.txt";
    let file_text = read_to_string(filename).unwrap();
    let mapped_numbers = file_text.lines().into_iter().map(extract_numbers);
    let (mut first, second): (Vec<_>, Vec<_>) = mapped_numbers.unzip();

    first.sort();
    let counts = second.into_iter().counts();
    let get_count = |val| -> i32 {
        match counts.get(&val) {
            None => 0,
            Some(x) => x.clone().try_into().unwrap(),
        }
    };
    let sum: i32 = first
        .into_iter()
        .map(|x: i32| -> i32 { x * get_count(x) })
        .sum();
    // let sum: i32 = zip(first, second).map(|(a, b)| {(a-b).abs()}).sum();
    println!("sum: {}", sum);
}
