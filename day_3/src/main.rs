use std::fs::read_to_string;

use regex::Regex;

fn mult(first: &str, second: &str) -> usize {
    let first_number: usize = first.parse().unwrap();
    let second_number: usize = second.parse().unwrap();
    return second_number * first_number;
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();
    let regex = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    let ddregex = Regex::new(r"((don't\(\))|(do\(\)))").unwrap();

    let sections: Vec<_> = ddregex
        .captures_iter(file_text.as_str())
        .map(|cap| cap.get(0).unwrap())
        .map(|grp| (grp.as_str() == "do()", grp.end()))
        .collect();

    let mut slices: Vec<&str> = Vec::new();
    let mut last_index = 0;
    let mut last_should_do = true;
    for (should_do, section_start) in sections {
        if last_should_do && !should_do {
            slices.push(&file_text[last_index..section_start]);
            last_should_do = should_do
        } else if !last_should_do && should_do {
            last_should_do = should_do;
            last_index = section_start
        }
    }

    if last_should_do {
        slices.push(&file_text[last_index..])
    }

    println!("{slices:?}");

    let valid_mult = slices.join("").to_string();

    let captures = regex.captures_iter(valid_mult.as_str());
    let calculation: usize = captures.map(|x| mult(&x["first"], &x["second"])).sum();
    println!("{calculation}");
}
