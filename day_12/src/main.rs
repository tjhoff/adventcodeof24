use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type Coordinate = (usize, usize);

#[derive(Debug, Clone)]
struct Crop {
    position: Coordinate,
    neighbors: Vec<Coordinate>,
}

fn get_at((x, y): (isize, isize), data: &Vec<Vec<char>>) -> Option<char> {
    if x < 0 || y < 0 || x as usize >= data[0].len() || y as usize >= data.len() {
        return None;
    }

    return Some(data[y as usize][x as usize]);
}

fn get_crop((x, y): (usize, usize), c: char, data: &Vec<Vec<char>>) -> Crop {
    let neighbor_coords = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let neighbors: Vec<Coordinate> = neighbor_coords
        .into_iter()
        .map(|(nx, ny)| (x as isize + nx, y as isize + ny))
        .filter(|coordinates| get_at(*coordinates, data) == Some(c))
        .map(|(nx, ny)| (nx.try_into().unwrap(), ny.try_into().unwrap()))
        .collect();

    return Crop {
        neighbors,
        position: (x, y),
    };
}

fn dfs_tree(
    start: &Crop,
    data: &HashMap<Coordinate, Crop>,
    seen: &mut HashSet<Coordinate>,
) -> Vec<Crop> {
    // println!("{start:?}");
    if seen.contains(&start.position) {
        // println!("already seen");
        return Vec::new();
    }

    seen.insert(start.position);

    let mut neighbor_crops: Vec<Crop> = start
        .neighbors
        .clone()
        .into_iter()
        .map(|node| dfs_tree(data.get(&node).unwrap(), data, seen))
        .flatten()
        .collect();

    neighbor_crops.append(&mut vec![start.clone()]);
    return neighbor_crops;
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();
    let lines = file_text.lines();

    let data: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut seen: HashSet<Coordinate> = HashSet::new();

    let crops: HashMap<Coordinate, Crop> = data
        .clone()
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            let coord_with_char: Vec<Crop> = line
                .into_iter()
                .enumerate()
                .map(|(x, char)| get_crop((x, y), char, &data))
                .collect();
            return coord_with_char;
        })
        .flatten()
        .map(|crop| (crop.position, crop))
        .collect();

    let sections: usize = crops
        .values()
        .map(|crop| {
            if seen.contains(&crop.position) {
                return None;
            }

            return Some(dfs_tree(crop, &crops, &mut seen));
        })
        .flatten()
        // .inspect(|f| println!("{f:?}"))
        .map(|section: Vec<Crop>| -> usize {
            let area = section.len();
            let perimeter = section
                .into_iter()
                .map(|c| 4 - c.neighbors.len())
                .sum::<usize>();
            return area * perimeter;
        })
        .sum();

    println!("{sections}");
}
