use std::{collections::HashMap, fs::read_to_string};

type StoneWithIteration = (usize, usize);

fn get_cache_or_calculate(
    stone: usize,
    remaining_iterations: usize,
    computed_stone_count: &mut HashMap<StoneWithIteration, usize>,
) -> usize {
    let precomputed = computed_stone_count.get(&(stone, remaining_iterations));
    match precomputed {
        Some(computed) => return *computed,
        None => {
            let computed = handle_stone(stone, remaining_iterations - 1, computed_stone_count);
            computed_stone_count.insert((stone, remaining_iterations), computed);
            return computed;
        }
    }
}

fn handle_stone(
    stone: usize,
    remaining_iterations: usize,
    computed_stone_count: &mut HashMap<StoneWithIteration, usize>,
) -> usize {
    if remaining_iterations == 0 {
        return 1;
    }
    if stone == 0 {
        return get_cache_or_calculate(1, remaining_iterations, computed_stone_count);
    }

    let chars: Vec<char> = stone.to_string().chars().collect();
    let str_len = chars.len();
    if str_len % 2 == 0 {
        let start: String = (&chars[0..str_len / 2]).iter().collect();
        let end: String = (&chars[str_len / 2..]).iter().collect();
        let left_stones = get_cache_or_calculate(
            start.parse().unwrap(),
            remaining_iterations,
            computed_stone_count,
        );
        let right_stones = get_cache_or_calculate(
            end.parse().unwrap(),
            remaining_iterations,
            computed_stone_count,
        );
        return left_stones + right_stones;
    }
    return get_cache_or_calculate(stone * 2024, remaining_iterations, computed_stone_count);
}

fn iterate_stones(
    stones: Vec<usize>,
    remaining_iterations: usize,
    computed_stone_count: &mut HashMap<StoneWithIteration, usize>,
) -> usize {
    return stones
        .into_iter()
        .map(|stone| handle_stone(stone, remaining_iterations, computed_stone_count))
        .sum();
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();

    let stones: Vec<usize> = file_text
        .split(" ")
        .map(|part| part.parse().unwrap())
        .collect();

    let num_iterations = 75;

    let mut computed_stone_count: HashMap<StoneWithIteration, usize> = HashMap::new();
    let sum = iterate_stones(stones, num_iterations, &mut computed_stone_count);
    println!("{}", sum);
}
