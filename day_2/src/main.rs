use itertools::Itertools;
use std::cmp::max;
use std::fs::read_to_string;

fn parse_report(report: &str) -> Vec<i32> {
    return report
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
}

fn check_permutations(levels: Vec<i32>) -> bool {
    if check_report(levels.clone()) {
        return true;
    };

    let level_count = levels.len();

    return levels
        .into_iter()
        .combinations(level_count - 1)
        .map(check_report)
        .any(|x| x == true);
}

fn check_report(levels: Vec<i32>) -> bool {
    let mut diffs = levels
        .clone()
        .into_iter()
        .tuple_windows()
        .map(|(x, y)| x - y);

    let positive = diffs.clone().filter(|x| x > &0 && x < &4).count();
    let negative = diffs.clone().filter(|x| x < &0 && x > &-4).count();

    // let debug_diff: Vec<i32> = diffs.clone().collect();
    // println!("{:?} {:?} {positive} {negative}", levels, debug_diff);
    return max(positive, negative) >= diffs.clone().count();
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();

    let good_reports: usize = file_text
        .lines()
        .map(parse_report)
        .map(check_permutations)
        .filter(|x| x == &true)
        .count();

    println!("good reports: {}", good_reports);
}
