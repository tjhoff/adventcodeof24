use std::{collections::LinkedList, fs::read_to_string, str::Chars};

use itertools::{Chunks, Itertools};

#[derive(Clone)]
enum BlockType {
    FILE { id: usize },
    EMPTY,
}

#[derive(Clone)]
struct Block {
    size: u32,
    block: BlockType,
}

fn find_containing_block(
    blocks: &Vec<Block>,
    current: &Block,
    index: usize,
) -> Option<usize> {
    return Some(
        blocks[0..index]
            .into_iter()
            .find_position(|b| b.size < current.size)?
            .0,
    );
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();

    let fill_with = |num: u32, item: Block| -> Vec<_> {
        { 1..=num }.into_iter().map(|_x| item.clone()).collect()
    };

    let blocks: Vec<Block> = file_text
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
        .map(|(id, chunk)| {
            let parse_digit = |c: char| c.to_digit(10).unwrap();

            let fill_amount = parse_digit(chunk.next().unwrap());

            let file_block = Block {
                size: fill_amount,
                block: BlockType::FILE { id: id },
            };

            let blocks = vec![file_block];
            if let Some(possible_empty) = chunk.next() {
                let empty_amount = parse_digit(possible_empty);
                let empty_block = Block {
                    size: empty_amount,
                    block: BlockType::EMPTY,
                };
                blocks.push(empty_block);
            }
            return blocks;
        })
        .flatten()
        .collect();

    let reordered_blocks = blocks.iter().rev();

    let size = blocks.len();
    for (index_from_back, block) in reordered_blocks.enumerate() {
        let Some(swap_block) = find_containing_block(&blocks, block, index) else {
            continue;
        };

        let block_with_room = blocks.get(swap_block).unwrap();

        let remaining_room = block_with_room.size - block.size;
        block_with_room.size = remaining_room;
        blocks.insert(swap_block, block.clone());
    }

    // println!("{defragged:?}");

    let checksum: usize = blocks
        .into_iter()
        .enumerate()
        .map(|(idx, id)| idx * id)
        .inspect(|x| println!("{x}"))
        .sum();

    println!("CHK {checksum}");
}
