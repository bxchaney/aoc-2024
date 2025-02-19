use aoc_2024::read_input;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Elevation {
    Trailhead,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Elevation {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::Trailhead,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            _ => panic!("invalid input"),
        }
    }

    fn next(&self) -> Option<Self> {
        match *self {
            Self::Trailhead => Some(Self::One),
            Self::One => Some(Self::Two),
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Four),
            Self::Four => Some(Self::Five),
            Self::Five => Some(Self::Six),
            Self::Six => Some(Self::Seven),
            Self::Seven => Some(Self::Eight),
            Self::Eight => Some(Self::Nine),
            Self::Nine => None,
        }
    }
}

fn out_of_bounds(topo: &Vec<Vec<Elevation>>, i: i32, j: i32) -> bool {
    i < 0 || j < 0 || i as usize >= topo.len() || j as usize >= topo[0].len()
}

fn adjacent(topo: &Vec<Vec<Elevation>>, point: &(usize, usize)) -> Vec<(usize, usize)> {
    [(0, 1), (1, 0), (-1, 0), (0, -1)]
        .iter()
        .filter_map(|(di, dj)| {
            let (i, j) = (point.0 as i32 + di, point.1 as i32 + dj);
            if out_of_bounds(&topo, i, j) {
                None
            } else {
                Some((i as usize, j as usize))
            }
        })
        .collect()
}

fn peaks(topo: &Vec<Vec<Elevation>>, point: &(usize, usize)) -> Vec<(usize, usize)> {
    let elevation = &topo[point.0][point.1];
    if let Some(next) = elevation.next() {
        adjacent(&topo, &point)
            .iter()
            .filter_map(|p| {
                if topo[p.0][p.1] == next {
                    Some(peaks(&topo, p))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    } else {
        vec![*point]
    }
}

fn score(topo: &Vec<Vec<Elevation>>, point: (usize, usize)) -> usize {
    HashSet::<(usize, usize)>::from_iter(peaks(&topo, &point)).len()
}

fn rating(topo: &Vec<Vec<Elevation>>, point: (usize, usize)) -> usize {
    peaks(&topo, &point).len()
}

fn main() {
    let topo: Vec<Vec<Elevation>> = read_input()
        .map(|l| l.unwrap().chars().map(|c| Elevation::from(c)).collect())
        .collect();

    let pt1 = topo
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, e)| {
                if *e == Elevation::Trailhead {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .flatten()
        .map(|p| score(&topo, p))
        .sum::<usize>();

    println!("pt1: {}", pt1);

    let pt2 = topo
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, e)| {
                if *e == Elevation::Trailhead {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .flatten()
        .map(|p| rating(&topo, p))
        .sum::<usize>();

    println!("pt2: {}", pt2);
}
