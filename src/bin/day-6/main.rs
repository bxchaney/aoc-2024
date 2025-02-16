use std::collections::{HashMap, HashSet};

use aoc_2024::read_input;

struct History {
    history: HashMap<(usize, usize), HashSet<Direction>>,
}

impl History {
    fn new() -> Self {
        History {
            history: HashMap::new(),
        }
    }

    fn add_history(&mut self, position: &(usize, usize), direction: Direction) {
        if let Some(collection) = self.history.get_mut(position) {
            collection.insert(direction);
        } else {
            let mut set = HashSet::new();
            set.insert(direction);
            self.history.insert(*position, set);
        }
    }

    fn check_history(&self, position: &(usize, usize), direction: &Direction) -> bool {
        if let Some(entry) = self.history.get(position) {
            return entry.contains(&direction);
        }
        false
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn vector(&self) -> (i32, i32) {
        // vector is the (i, j) offset for a direction
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TileType {
    Void,
    Block,
}

impl TileType {
    fn from(s: char) -> Self {
        match s {
            '.' | '^' => Self::Void,
            '#' => Self::Block,
            _ => {
                panic!("uknown input");
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    visited: bool,
    tile_type: TileType,
}

fn out_of_bounds(tiles: &Vec<Vec<Tile>>, position: (i32, i32)) -> bool {
    position.0 < 0
        || position.1 < 0
        || position.0 as usize >= tiles.len()
        || position.1 as usize >= tiles[0].len()
}

fn next(
    tiles: &Vec<Vec<Tile>>,
    position: (usize, usize),
    direction: Direction,
) -> Option<((usize, usize), Direction)> {
    let vector = direction.vector();

    if out_of_bounds(
        tiles,
        (position.0 as i32 + vector.0, position.1 as i32 + vector.1),
    ) {
        return None;
    }

    let next_position = (
        (position.0 as i32 + vector.0) as usize,
        (position.1 as i32 + vector.1) as usize,
    );

    match tiles[next_position.0][next_position.1].tile_type {
        TileType::Void => Some((next_position, direction)),
        TileType::Block => Some((position, direction.next())),
    }
}

fn visit(tiles: &mut Vec<Vec<Tile>>, start: (usize, usize)) {
    tiles[start.0][start.1].visited = true;
    let mut position = start;
    let mut direction = Direction::Up;
    while let Some((next_position, next_direction)) = next(tiles, position, direction) {
        direction = next_direction;
        position = next_position;
        tiles[position.0][position.1].visited = true;
    }
}

fn contains_loop(tiles: &Vec<Vec<Tile>>, start: (usize, usize)) -> bool {
    let mut position = start;
    let mut direction = Direction::Up;
    let mut history = History::new();

    while let Some((next_position, next_direction)) = next(tiles, position, direction) {
        direction = next_direction;
        position = next_position;
        if history.check_history(&position, &direction) {
            return true;
        }
        history.add_history(&position, direction);
    }

    false
}

fn loops(tiles: &Vec<Vec<Tile>>, start: (usize, usize)) -> i32 {
    let mut counter = 0;
    for i in 0..tiles.len() {
        for j in 0..tiles[i].len() {
            if tiles[i][j].visited {
                let mut board = tiles.clone();
                board[i][j].tile_type = TileType::Block;
                if contains_loop(&board, start) {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn main() {
    let mut guard = vec![];
    let mut tiles: Vec<Vec<Tile>> = read_input()
        .enumerate()
        .map(|(i, l)| {
            l.unwrap()
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '^' {
                        guard.push((i, j));
                    }
                    return Tile {
                        visited: false,
                        tile_type: TileType::from(c),
                    };
                })
                .collect()
        })
        .collect();

    if guard.len() != 1 {
        panic!("error finding guard");
    }

    visit(&mut tiles, guard[0]);

    println!(
        "pt1: {}",
        tiles.iter().flatten().filter(|t| t.visited).count()
    );

    println!("pt2 {}", loops(&tiles, guard[0]));
}
