use std::{collections::LinkedList, fs::read_to_string};

use itertools::Itertools;

#[derive(Clone)]
enum Block {
    FILE { id: usize },
    EMPTY,
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();

    let parse_digit = |c: char| c.to_digit(10).unwrap();

    let fill_with = |num: u32, item: Block| -> Vec<_> {
        { 1..=num }.into_iter().map(|_x| item.clone()).collect()
    };

    let blocks: Vec<Block> = file_text
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
        .map(|(id, (mut chunk))| {
            let fill_amount = parse_digit(chunk.next().unwrap());

            let mut full = fill_with(fill_amount, Block::FILE { id: id });
            if let Some(possible_empty) = chunk.next() {
                let empty_amount = parse_digit(possible_empty);
                let mut empties = fill_with(empty_amount, Block::EMPTY);
                full.append(&mut empties);
            }
            return full;
        })
        .flatten()
        .collect();

    let mut llblocks: LinkedList<Block> = blocks.into_iter().collect();

    let mut defragged: Vec<usize> = Vec::new();

    loop {
        let Some(block) = llblocks.pop_front() else {
            break;
        };

        // println!("{defragged:?}");

        match block {
            Block::FILE { id } => defragged.push(id),
            Block::EMPTY => {
                // find the first not empty block and push that instead.
                loop {
                    let Some(end_block) = llblocks.pop_back() else {
                        break;
                    };

                    match end_block {
                        Block::FILE { id } => {
                            defragged.push(id);
                            break;
                        }
                        Block::EMPTY => continue,
                    }
                }
            }
        }
    }

    // println!("{defragged:?}");

    let checksum: usize = defragged
        .into_iter()
        .enumerate()
        .map(|(idx, id)| idx * id)
        .inspect(|x| println!("{x}"))
        .sum();

    println!("CHK {checksum}");
}
