use std::fs::read_to_string;

fn parse_line(line: &str) -> (usize, Vec<usize>) {
    let mut parts = line.split(":");
    let result: usize = parts.next().unwrap().parse().unwrap();

    let operators: Vec<usize> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|p| p.parse().unwrap())
        .rev()
        .collect();
    return (result, operators);
}

#[derive(Debug)]
enum Operation {
    MULT,
    ADD,
    OR,
}

fn intermed_op(
    target: usize,
    tally: usize,
    op: Operation,
    mut operators: Vec<usize>,
    depth: usize,
) -> bool {
    let my_val = operators.pop().unwrap();
    let spaces = " ".repeat(depth);
    // println!("{spaces}:{tally} {op:?} {my_val}");
    let my_tally = match op {
        Operation::ADD => tally + my_val,
        Operation::MULT => tally * my_val,
        Operation::OR => (tally.to_string() + &my_val.to_string()).parse().unwrap(),
    };
    if operators.len() == 0 {
        // println!("{target} == {my_tally}");
        return target == my_tally;
    }

    return [Operation::ADD, Operation::MULT, Operation::OR]
        .into_iter()
        .any(|new_op| intermed_op(target, my_tally, new_op, operators.clone(), depth + 1));
}

fn check_operations((result, mut operators): (usize, Vec<usize>)) -> usize {
    let first = operators.pop().unwrap();
    let is_correct = [Operation::ADD, Operation::MULT, Operation::OR]
        .into_iter()
        .any(|new_op| intermed_op(result, first, new_op, operators.clone(), 1));
    if is_correct {
        return result;
    } else {
        return 0;
    }
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();
    let result: usize = file_text
        .lines()
        .map(parse_line)
        .map(check_operations)
        .sum();
    println!("{result}");
}
