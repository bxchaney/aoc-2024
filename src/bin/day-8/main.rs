use gcd::gcd;
use itertools::{all, Itertools};
use std::{
    collections::{HashMap, HashSet},
    hash::RandomState,
};

use aoc_2024::read_input;

mod gcd;

struct Antennas {
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl Antennas {
    fn new() -> Self {
        Self {
            antennas: HashMap::new(),
        }
    }

    fn add_point(&mut self, c: char, point: (usize, usize)) {
        if let Some(entry) = self.antennas.get_mut(&c) {
            entry.push(point);
        } else {
            let vec = vec![point];
            self.antennas.insert(c, vec);
        }
    }
}

fn point_antinodes(a: &(usize, usize), b: &(usize, usize)) -> Vec<(i32, i32)> {
    let vertical_offset = a.0 as i32 - b.0 as i32;
    let horizontal_offset = a.1 as i32 - b.1 as i32;

    vec![
        (a.0 as i32 + vertical_offset, a.1 as i32 + horizontal_offset),
        (b.0 as i32 - vertical_offset, b.1 as i32 - horizontal_offset),
    ]
}

fn antinodes(points: &Vec<(usize, usize)>) -> Vec<(i32, i32)> {
    points
        .iter()
        .combinations(2)
        .map(|v| point_antinodes(v[0], v[1]))
        .flatten()
        .collect()
}

fn out_of_bounds(map: &Vec<Vec<char>>, point: &(i32, i32)) -> bool {
    point.0 < 0 || point.1 < 0 || point.0 as usize >= map.len() || point.1 as usize >= map[0].len()
}

fn reduce(a: i32, b: i32) -> (i32, i32) {
    let factor = gcd(a.abs() as u32, b.abs() as u32) as i32;
    (a / factor, b / factor)
}

fn extended_point_entinodes(
    map: &Vec<Vec<char>>,
    a: &(usize, usize),
    b: &(usize, usize),
) -> Vec<(i32, i32)> {
    let vertical_offset = a.0 as i32 - b.0 as i32;
    let horizontal_offset = a.1 as i32 - b.1 as i32;

    let (di, dj) = reduce(vertical_offset, horizontal_offset);

    let mut points = vec![];
    let mut i = a.0 as i32;
    let mut j = a.1 as i32;
    loop {
        i += di;
        j += dj;
        if out_of_bounds(map, &(i, j)) {
            break;
        }
        points.push((i, j));
    }
    i = a.0 as i32;
    j = a.1 as i32;
    loop {
        i -= di;
        j -= dj;
        if out_of_bounds(map, &(i, j)) {
            break;
        }
        points.push((i, j));
    }

    points
}

fn extended_antinodes(map: &Vec<Vec<char>>, points: &Vec<(usize, usize)>) -> Vec<(i32, i32)> {
    let antinodes = points
        .iter()
        .combinations(2)
        .map(|v| extended_point_entinodes(&map, v[0], v[1]))
        .flatten();

    if points.len() > 2 {
        return points
            .iter()
            .map(|p| (p.0 as i32, p.1 as i32))
            .chain(antinodes)
            .collect();
    }
    antinodes.collect()
}

fn main() {
    let mut antennas = Antennas::new();
    let map: Vec<Vec<char>> = read_input().map(|l| l.unwrap().chars().collect()).collect();

    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' {
                antennas.add_point(*c, (i, j));
            }
        }
    }

    let all_antinodes = antennas
        .antennas
        .iter()
        .map(|(_, v)| antinodes(v))
        .flatten()
        .filter(|p| !out_of_bounds(&map, p));

    let unique: HashSet<(i32, i32), RandomState> = HashSet::from_iter(all_antinodes);

    println!("pt1: {}", unique.len());

    let all_extended_antinodes = antennas
        .antennas
        .iter()
        .map(|(_, v)| extended_antinodes(&map, v))
        .flatten()
        .filter(|p| !out_of_bounds(&map, p));

    let extended_unique: HashSet<(i32, i32), RandomState> =
        HashSet::from_iter(all_extended_antinodes);

    println!("pt2: {}", extended_unique.len());
}
