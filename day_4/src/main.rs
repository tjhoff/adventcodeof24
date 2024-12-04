use itertools::Itertools;
use std::fs::read_to_string;

fn is_mas<F>(x: usize, y: usize, get_at: F) -> bool
where
    F: Fn(usize, usize) -> char,
{
    let x32: i32 = x.try_into().unwrap();
    let y32: i32 = y.try_into().unwrap();

    let (d1, d2): (String, String) = [-1, 0, 1]
        .map(|c| {
            let ix = (x32 + c).try_into().unwrap();
            let iy = (y32 + c).try_into().unwrap();
            let ixprime = (x32 + (c * -1)).try_into().unwrap();
            (get_at(ix, iy), get_at(ixprime, iy))
        })
        .into_iter()
        .unzip();

    let result = [d1.clone(), d2.clone()]
        .into_iter()
        .all(|s| s == "MAS" || s == "SAM");
    
    return result;
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();
    let data: Vec<Vec<char>> = file_text
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let w = data.len();
    let h = data[0].len();
    let get_at = |x: usize, y: usize| data[y][x];

    // Only consider the inner rows and columns for starting point
    let wrange = 1..(w - 1);
    let hrange = 1..(h - 1);

    let massy = |(x, y)| is_mas(x, y, get_at);

    let masses = wrange
        .cartesian_product(hrange)
        // Optimization for skippin' non-As
        .filter(|(x, y)| get_at(*x, *y) == 'A')
        .map(massy)
        .filter(|m| m == &true)
        .count();
    println!("{masses}")
}
