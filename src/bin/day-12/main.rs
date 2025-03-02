use std::collections::{HashMap, HashSet};

use aoc_2024::{out_of_bounds, read_input, Direction};

#[derive(Debug, Clone, Copy)]
struct Edge {
    visited: bool,
}

#[derive(Clone)]
struct Plot {
    position: (usize, usize),
    perimeter: usize,
    edges: HashMap<Direction, Edge>,
}

#[derive(Clone)]
struct PlotGroup {
    plots: Vec<Plot>,
    points: HashSet<(usize, usize)>,
}

impl PlotGroup {
    fn cost(&self) -> usize {
        let area = self.plots.len();
        area * self.plots.iter().map(|p| p.perimeter).sum::<usize>()
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn corners(&mut self, garden: &Vec<Vec<char>>) -> i32 {
        let mut corners = 0;
        loop {
            // points with top perimeter that has not been visited
            let candidate_starts = self.plots.iter().filter_map(|p| {
                if let Some(edge) = p.edges.get(&Direction::Up) {
                    if !edge.visited {
                        return Some(p.position);
                    }
                    None
                } else {
                    None
                }
            });

            if candidate_starts.clone().count() == 0 {
                break;
            }

            let start = candidate_starts.clone().next().unwrap();
            corners += walkaround(garden, self, start);
        }

        corners
    }
}

fn neighbors(point: (usize, usize)) -> Vec<(Direction, (i32, i32))> {
    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .map(|(di, dj)| {
            (
                Direction::from((*di, *dj)),
                (point.0 as i32 + di, point.1 as i32 + dj),
            )
        })
        .collect()
}

fn build_edges(external_directions: &Vec<Direction>) -> HashMap<Direction, Edge> {
    let mut edges = HashMap::new();
    for dir in external_directions {
        edges.insert(*dir, Edge { visited: false });
    }
    edges
}

fn visit(garden: &Vec<Vec<char>>, start: (usize, usize)) -> PlotGroup {
    let mut frontier = vec![start];
    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    let mut plots = vec![];
    let plot_type = garden[start.0][start.1];
    while frontier.len() > 0 {
        let next = frontier.pop().unwrap();

        if explored.contains(&next) {
            continue;
        }
        // Determine how many external edges the plot has and add to plots. Then,
        // add its neighbors that are the same plot type to frontier
        let adj = neighbors(next);
        let external_adj: Vec<Direction> = adj
            .iter()
            .filter_map(|(d, (i, j))| {
                // is external edge if neighbor is out of bounds, or if neighbor is
                // a different plot type
                if out_of_bounds(&garden, *i, *j) || garden[*i as usize][*j as usize] != plot_type {
                    Some(*d)
                } else {
                    None
                }
            })
            .collect();
        let adj_friend: Vec<(Direction, (usize, usize))> = adj
            .iter()
            .filter_map(|(d, (i, j))| {
                if !out_of_bounds(&garden, *i, *j) && garden[*i as usize][*j as usize] == plot_type
                {
                    Some((*d, (*i as usize, *j as usize)))
                } else {
                    None
                }
            })
            .collect();

        let edges = build_edges(&external_adj);

        plots.push(Plot {
            position: next,
            perimeter: adj.len() - adj_friend.len(),
            edges,
        });

        frontier.extend(adj_friend.iter().map(|(_, p)| p));
        explored.insert(next);
    }
    let points = HashSet::from_iter(plots.iter().map(|p| p.position));
    PlotGroup { plots, points }
}

fn explore(garden: &Vec<Vec<char>>) -> Vec<PlotGroup> {
    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    let mut plot_groups = vec![];
    for (i, row) in garden.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if explored.contains(&(i, j)) {
                continue;
            }
            let group = visit(&garden, (i, j));
            explored.extend(group.points.iter());
            plot_groups.push(group);
        }
    }

    plot_groups
}

fn should_turn_left(
    garden: &Vec<Vec<char>>,
    group: &PlotGroup,
    direction: &Direction,
    position: &(i32, i32),
) -> bool {
    let (di, dj) = direction.vector();
    let (next_i, next_j) = (position.0 + di, position.1 + dj);
    if out_of_bounds(garden, next_i, next_j) {
        return false;
    }

    // next position is in bounds
    // if next position is in the garden group, then turn left, otherwise, do not
    group.points.contains(&(next_i as usize, next_j as usize))
}

fn should_turn_right(
    garden: &Vec<Vec<char>>,
    group: &PlotGroup,
    direction: &Direction,
    position: &(i32, i32),
) -> bool {
    let (di, dj) = direction.vector();
    let (next_i, next_j) = (position.0 + di, position.1 + dj);

    let (right_i, right_j) = direction.turn_clockwise().vector();

    // if next step on the right would be out of bounds, turn right
    if out_of_bounds(garden, next_i + right_i, next_j + right_j) {
        return true;
    }

    // turn right if the square to the right of the next square is
    // not in the group.
    !group
        .points
        .contains(&((next_i + right_i) as usize, (next_j + right_j) as usize))
}

fn turn_left(position: (i32, i32), direction: &Direction) -> ((i32, i32), Direction) {
    (position, direction.turn_counter_clockwise())
}

fn turn_right(position: (i32, i32), direction: &Direction) -> ((i32, i32), Direction) {
    let (i, j) = position;
    let (di, dj) = direction.vector();
    let (right_i, right_j) = direction.turn_clockwise().vector();
    (
        (i + di + right_i, j + dj + right_j),
        direction.turn_clockwise(),
    )
}

fn go_straight(position: (i32, i32), direction: &Direction) -> ((i32, i32), Direction) {
    let (i, j) = position;
    let (di, dj) = direction.vector();
    ((i + di, j + dj), *direction)
}

fn mark_visited(group: &mut PlotGroup, direction: &Direction, position: (i32, i32)) {
    let (i, j) = position;
    let (di, dj) = direction.turn_clockwise().vector();
    let visited = ((i + di) as usize, (j + dj) as usize);
    let plot = group
        .plots
        .iter_mut()
        .filter(|p| p.position == visited)
        .next()
        .unwrap();
    let edge_direction = direction.turn_clockwise().opposite();
    plot.edges.insert(edge_direction, Edge { visited: true });
}

fn visited(group: &mut PlotGroup, direction: &Direction, position: (i32, i32)) -> bool {
    let (i, j) = position;
    let (di, dj) = direction.turn_clockwise().vector();
    let visited = ((i + di) as usize, (j + dj) as usize);
    let plot = group
        .plots
        .iter()
        .filter(|p| p.position == visited)
        .next()
        .unwrap();
    let edge_direction = direction.turn_clockwise().opposite();
    if let Some(edge) = plot.edges.get(&edge_direction) {
        edge.visited
    } else {
        false
    }
}

/// walks the perimeter of a garden group and counts the number of corners
fn walkaround(garden: &Vec<Vec<char>>, group: &mut PlotGroup, start: (usize, usize)) -> i32 {
    // assume that we start above the top left element of a garden group, by
    // construction
    let (top_i, top_j) = start;

    // position above the first plot square. This could be out of bounds
    let mut walker = (top_i as i32 - 1, top_j as i32);
    let mut direction = Direction::Right;
    let mut corners = 0;

    // at every step, we can either walk forward, turn left, or turn right
    // we turn left if the next tile is in th garden group -> meaning we have
    // run into a wall
    // we turn right if we would walk out of bounds, or if there would not be
    // a group element to the right if we go straigh
    //
    // we mark the perimeter edge that we were touching as visited
    loop {
        if visited(group, &direction, walker) {
            break;
        }
        mark_visited(group, &direction, walker);
        if should_turn_left(&garden, &group, &direction, &walker) {
            (walker, direction) = turn_left(walker, &direction);
            corners += 1;
        } else if should_turn_right(&garden, &group, &direction, &walker) {
            (walker, direction) = turn_right(walker, &direction);
            corners += 1;
        } else {
            (walker, direction) = go_straight(walker, &direction);
        }
    }

    corners
}

fn main() {
    let garden: Vec<Vec<char>> = read_input().map(|l| l.unwrap().chars().collect()).collect();

    let groups = explore(&garden);

    let pt1 = groups.iter().map(|g| g.cost()).sum::<usize>();
    println!("pt1: {}", pt1);

    let mut pt2 = 0;
    for group in groups {
        let mut group_clone = group.clone();
        let area = group.area();
        let corners = group_clone.corners(&garden) as usize;
        // println!("area={}, corners={}", area, corners);
        pt2 += area * corners;
    }
    println!("pt2: {}", pt2);
}
