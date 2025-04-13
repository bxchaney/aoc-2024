use std::collections::HashSet;

use aoc_2024::{out_of_bounds, read_input, InputFileBuffer};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

#[derive(PartialEq)]
enum Terrain {
    Path,
    Wall,
}

impl From<char> for Terrain {
    fn from(value: char) -> Self {
        match value {
            '.' | 'E' | 'S' => Self::Path,
            '#' => Self::Wall,
            _ => panic!("invalid terrain type"),
        }
    }
}

struct Course {
    start: (i32, i32),
    finish: (i32, i32),
    course: Vec<Vec<Terrain>>,
}

impl From<InputFileBuffer> for Course {
    fn from(lines: InputFileBuffer) -> Self {
        let mut start = (0, 0);
        let mut finish = (0, 0);
        let course = lines
            .enumerate()
            .map(|(i, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == 'S' {
                            start = (i as i32, j as i32);
                        }
                        if c == 'E' {
                            finish = (i as i32, j as i32)
                        }
                        return Terrain::from(c);
                    })
                    .collect()
            })
            .collect();

        Self {
            start,
            finish,
            course,
        }
    }
}

type Distances = Vec<Vec<i32>>;

#[derive(Debug)]
struct Cheat {
    at: (i32, i32),
    to: (i32, i32),
    cost: i32,
}

fn build_memo(course: &Course) -> Distances {
    let mut dist: Distances = vec![vec![0; course.course[0].len()]; course.course.len()];

    let mut cost = 0;
    let mut frontier = vec![course.finish];
    let mut explored: HashSet<(i32, i32)> = HashSet::new();

    while frontier.len() > 0 {
        let (i, j) = frontier.pop().unwrap();
        dist[i as usize][j as usize] = cost;
        cost += 1;
        explored.insert((i, j));
        for (di, dj) in DIRECTIONS {
            let (next_i, next_j) = (i + di, j + dj);
            if !explored.contains(&(next_i, next_j))
                && course.course[next_i as usize][next_j as usize] == Terrain::Path
            {
                frontier.push((next_i, next_j));
            }
        }
    }

    dist
}

fn cheat(course: &Course, dist: &Distances) -> Vec<Cheat> {
    let mut cheats = vec![];
    let start_i = course.start.0 as usize;
    let start_j = course.start.1 as usize;

    let mut frontier = vec![course.start];
    let mut explored: HashSet<(i32, i32)> = HashSet::new();
    while frontier.len() > 0 {
        let (i, j) = frontier.pop().unwrap();
        explored.insert((i, j));
        // add next step
        for (di, dj) in DIRECTIONS {
            let (next_i, next_j) = (i + di, j + dj);
            if !explored.contains(&(next_i, next_j))
                && course.course[next_i as usize][next_j as usize] == Terrain::Path
            {
                frontier.push((next_i, next_j));
            }

            // check for cheats
            let (cheat_i, cheat_j) = (i + 2 * di, j + 2 * dj);
            if !out_of_bounds(&course.course, cheat_i, cheat_j)
                && course.course[cheat_i as usize][cheat_j as usize] == Terrain::Path
                && !explored.contains(&(cheat_i, cheat_j))
            {
                let cost = dist[cheat_i as usize][cheat_j as usize] - dist[i as usize][j as usize]
                    + dist[start_i][start_j]
                    + 2;
                cheats.push(Cheat {
                    at: (i, j),
                    to: (cheat_i, cheat_j),
                    cost,
                });
            }
        }
    }

    cheats
}

fn cheat_savings(dist: &Distances, cheat_to: &(i32, i32), cheat_from: &(i32, i32)) -> i32 {
    let (i, j) = cheat_from;
    let (cheat_i, cheat_j) = cheat_to;
    let cheat_distance = (cheat_to.0 - cheat_from.0).abs() + (cheat_to.1 - cheat_from.1).abs();

    dist[*i as usize][*j as usize] - dist[*cheat_i as usize][*cheat_j as usize] - cheat_distance
}

fn more_cheating(course: &Course, dist: &Distances, threshold: i32) -> Vec<Cheat> {
    let mut cheats = vec![];

    let mut frontier = vec![course.start];
    let mut explored: HashSet<(i32, i32)> = HashSet::new();
    while frontier.len() > 0 {
        let (i, j) = frontier.pop().unwrap();
        explored.insert((i, j));
        // add next step
        for (di, dj) in DIRECTIONS {
            let (next_i, next_j) = (i + di, j + dj);
            if !explored.contains(&(next_i, next_j))
                && course.course[next_i as usize][next_j as usize] == Terrain::Path
            {
                frontier.push((next_i, next_j));
            }
        }

        for cheat_i in i - 20..=i + 20 {
            for cheat_j in j - 20..=j + 20 {
                if (cheat_i - i).abs() + (cheat_j - j).abs() > 20 {
                    continue;
                }
                if !out_of_bounds(&course.course, cheat_i, cheat_j)
                    && course.course[cheat_i as usize][cheat_j as usize] == Terrain::Path
                {
                    let savings = cheat_savings(dist, &(cheat_i, cheat_j), &(i, j));
                    if savings >= threshold {
                        cheats.push(Cheat {
                            at: (i, j),
                            to: (cheat_i, cheat_j),
                            cost: savings,
                        })
                    }
                }
            }
        }
    }

    cheats
}

fn parse_input() -> Course {
    Course::from(read_input())
}

fn main() {
    // we first parse the input data and store the coordinates of the start
    // and end. Then, we backtrack from the end to store the regular remaining
    // cost to get from any spot on the track to the end.
    // Then, we navigate the course from the start and attempt to cheat at
    // every opportunity, tracking the cheat location, and the total cost after
    // the cheat.

    let course = parse_input();
    let dist = build_memo(&course);
    let mut cheats = cheat(&course, &dist);

    cheats.sort_by(|a, b| a.cost.cmp(&b.cost));
    let (start_i, start_j) = course.start;
    let start_cost = &dist[start_i as usize][start_j as usize];
    let big_savings: Vec<&Cheat> = cheats
        .iter()
        .filter(|c| start_cost - &c.cost >= 100)
        .collect();

    println!("pt1: {}", big_savings.len());

    let more_cheats = more_cheating(&course, &dist, 100);
    println!("pt2: {}", more_cheats.len());
}
