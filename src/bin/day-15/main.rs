use aoc_2024::{out_of_bounds, read_input, Direction};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Terrain {
    Robot,
    Wall,
    Box,
    Void,
}

impl From<char> for Terrain {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Void,
            '@' => Self::Robot,
            '#' => Self::Wall,
            'O' => Self::Box,
            _ => panic!("unrecognized terrain type!"),
        }
    }
}

impl ToString for Terrain {
    fn to_string(&self) -> String {
        match *self {
            Self::Wall => String::from("#"),
            Self::Box => String::from("O"),
            Self::Robot => String::from("@"),
            Self::Void => String::from("."),
        }
    }
}

fn robot_starting_position(map: &Vec<Vec<Terrain>>) -> (i32, i32) {
    for (i, row) in map.iter().enumerate() {
        for (j, terr) in row.iter().enumerate() {
            match *terr {
                Terrain::Robot => {
                    return (i as i32, j as i32);
                }
                _ => {}
            }
        }
    }
    panic!();
}

// attempts to move robot and returns new robot position
fn move_robot(
    map: &mut Vec<Vec<Terrain>>,
    position: (i32, i32),
    direction: &Direction,
) -> (i32, i32) {
    // find first void in robot path
    // if no voids, no movement
    let (mut i, mut j) = position;
    let (di, dj) = direction.vector();
    let mut first_void = None;
    let mut first_box = None;
    loop {
        (i, j) = (i + di, j + dj);
        if out_of_bounds(map, i, j) {
            break;
        }
        let tile = &map[i as usize][j as usize];
        match tile {
            Terrain::Wall => {
                break;
            }
            Terrain::Void => {
                first_void = Some((i, j));
                break;
            }
            Terrain::Box => {
                if first_box == None {
                    first_box = Some((i, j))
                }
            }
            _ => {}
        }

        match (first_void, first_box) {
            (Some(_), Some(_)) => {
                break;
            }
            _ => {}
        }
    }

    // find first box in robot path
    // if no box then no worries
    match (first_void, first_box) {
        (Some((vi, vj)), Some((bi, bj))) => {
            let (ri, rj) = position;
            map[ri as usize][rj as usize] = Terrain::Void;
            map[bi as usize][bj as usize] = Terrain::Robot;
            map[vi as usize][vj as usize] = Terrain::Box;
            (bi, bj)
        }
        (Some((vi, vj)), None) => {
            let (ri, rj) = position;
            map[ri as usize][rj as usize] = Terrain::Void;
            map[vi as usize][vj as usize] = Terrain::Robot;
            (vi, vj)
        }
        (None, _) => position,
    }
}

fn simulate(map: &mut Vec<Vec<Terrain>>, protocol: &Vec<Direction>) {
    let mut robot_position = robot_starting_position(map);

    for direction in protocol {
        robot_position = move_robot(map, robot_position, direction);
    }
}

fn score(map: &Vec<Vec<Terrain>>) -> i32 {
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, tile)| match tile {
                    Terrain::Box => Some((i, j)),
                    _ => None,
                })
        })
        .flatten()
        .map(|(i, j)| (i * 100 + j) as i32)
        .sum::<i32>()
}

fn show_map(map: &Vec<Vec<Terrain>>) {
    for row in map {
        println!("{}", row.iter().map(|t| t.to_string()).join(""));
    }
    println!();
}

fn main() {
    let mut input = read_input();

    let mut map: Vec<Vec<Terrain>> = vec![];
    while let Some(l) = input.next() {
        let line = l.unwrap();
        if line.len() == 0 {
            break;
        }
        map.push(line.chars().map(|c| Terrain::from(c)).collect());
    }

    let mut directions: Vec<Direction> = vec![];
    while let Some(l) = input.next() {
        directions.extend(l.unwrap().chars().map(|c| Direction::from(c)));
    }

    simulate(&mut map, &directions);

    println!("pt1: {}", score(&map));
}
