use aoc_2024::read_input;

#[derive(Debug, Clone, Copy)]
struct File {
    size: u64,
    id: u64,
}

#[derive(Debug, Clone, Copy)]
enum Block {
    File(File),
    Void(u64),
}

struct FileSystem {
    blocks: Vec<Option<u64>>,
    detailed_blocks: Vec<Block>,
}

impl FileSystem {
    fn from(storage: Vec<u64>) -> Self {
        let mut blocks = vec![];
        let mut detailed_blocks = vec![];
        let mut id: u64 = 0;

        let mut dense_blocks = storage.iter();
        while let Some(file) = dense_blocks.next() {
            let free = dense_blocks.next();

            detailed_blocks.push(Block::File(File { size: *file, id }));
            for _ in 0..*file {
                blocks.push(Some(id));
            }
            match free {
                None => {
                    break;
                }
                Some(amount) => {
                    detailed_blocks.push(Block::Void(*amount));
                    for _ in 0..*amount {
                        blocks.push(None);
                    }
                }
            }
            id += 1;
        }

        Self {
            blocks,
            detailed_blocks,
        }
    }
}

fn reorganize_storage(blocks: &Vec<Option<u64>>) -> Vec<u64> {
    let mut reverse_storage = blocks.iter().rev();
    let free_spaces = blocks
        .iter()
        .filter(|e| match *e {
            None => true,
            Some(_) => false,
        })
        .count();

    let mut filled_spaces = 0;
    let mut storage = vec![];
    'outer: for block in blocks {
        if filled_spaces > free_spaces {
            break;
        }
        if filled_spaces == free_spaces {
            if storage.len() == blocks.len() - free_spaces {
                break;
            }
            storage.push(block.unwrap());
            continue;
        }
        match block {
            Some(id) => storage.push(*id),
            None => 'inner: loop {
                if filled_spaces >= free_spaces {
                    break 'outer;
                }
                let next_reverse = reverse_storage.next();
                match next_reverse {
                    None => {
                        panic!("exhausted reverse!");
                    }
                    Some(data) => match *data {
                        None => {
                            filled_spaces += 1;
                        }
                        Some(rev_id) => {
                            storage.push(rev_id);
                            filled_spaces += 1;
                            break 'inner;
                        }
                    },
                }
            },
        }
    }
    storage
}

fn attempt_shift(blocks: &mut Vec<Block>, file: File) {
    // if we find a space large enough for the file, insert the file there and
    // adjust the mount of empty space.
    // then, ensure that the file is replaced with empty space equal to its
    // size
    let mut swapped = false;
    for (i, block) in blocks.clone().iter().enumerate() {
        if !swapped {
            match block {
                Block::File(f) => {
                    if f.id == file.id {
                        break;
                    }
                }
                Block::Void(cap) => {
                    if *cap < file.size {
                        continue;
                    }
                    blocks.insert(i, Block::File(file));
                    blocks[i + 1] = Block::Void(*cap - file.size);
                    swapped = true;
                }
            }
        } else {
            match block {
                Block::Void(_) => {}
                Block::File(f) => {
                    if f.id == file.id {
                        blocks[i + 1] = Block::Void(file.size);
                    }
                }
            }
        }
    }
}

fn reorganize_files(blocks: &Vec<Block>) -> Vec<Block> {
    let mut new_blocks = blocks.clone();
    let file_ids = blocks
        .iter()
        .filter_map(|e| match *e {
            Block::File(f) => Some(f),
            Block::Void(_) => None,
        })
        .rev();

    for f in file_ids {
        attempt_shift(&mut new_blocks, f);
    }
    new_blocks
}

fn encode_blocks(blocks: &Vec<Block>) -> Vec<Option<u64>> {
    let mut encoded_blocks = vec![];
    for block in blocks {
        match block {
            Block::File(f) => {
                for _ in 0..f.size {
                    encoded_blocks.push(Some(f.id));
                }
            }
            Block::Void(cap) => {
                for _ in 0..*cap {
                    encoded_blocks.push(None);
                }
            }
        }
    }
    encoded_blocks
}

fn main() {
    let filesystem = FileSystem::from(
        read_input()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<u64>>()
            })
            .flatten()
            .collect(),
    );

    let organized_storage = reorganize_storage(&filesystem.blocks);

    println!(
        "pt1: {}",
        organized_storage
            .iter()
            .enumerate()
            .map(|(i, e)| i as u64 * e)
            .sum::<u64>()
    );

    let new_checksum = encode_blocks(&reorganize_files(&filesystem.detailed_blocks))
        .iter()
        .enumerate()
        .map(|(i, e)| match e {
            Some(id) => i as u64 * id,
            None => 0,
        })
        .sum::<u64>();

    println!("pt2: {}", new_checksum);
}
