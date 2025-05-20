use std::collections::{HashMap, HashSet};

use aoc_2024::read_input;
use itertools::Itertools;

fn parse_starts() -> Vec<u64> {
    read_input()
        .map(|s| s.unwrap().parse::<u64>().unwrap())
        .collect()
}

fn mix(secret: u64, other: u64) -> u64 {
    secret ^ other
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn first(secret: u64) -> u64 {
    prune(mix(secret * 64, secret))
}

fn second(secret: u64) -> u64 {
    prune(mix(secret / 32, secret))
}

fn third(secret: u64) -> u64 {
    prune(mix(secret * 2048, secret))
}

fn secret(secret: u64, iterations: usize, functions: Vec<fn(u64) -> u64>) -> u64 {
    functions
        .repeat(iterations)
        .iter()
        .fold(secret, |acc, f| f(acc))
}

fn secret_sequence(secret: u64, iterations: usize, functions: Vec<fn(u64) -> u64>) -> Vec<u64> {
    let mut sequence = vec![secret];
    let mut next_secret = secret;
    for _ in 0..iterations {
        for f in &functions {
            next_secret = f(next_secret);
        }
        sequence.push(next_secret);
    }

    sequence
}

fn sequence_prices(sequence: Vec<u64>) -> Vec<(u64, (i32, i32, i32, i32))> {
    sequence
        .windows(2)
        .map(|vs| {
            let [x, y] = vs else { unreachable!() };
            ((y % 10), ((y % 10) as i32 - (x % 10) as i32))
        })
        .collect_vec()
        .windows(4)
        .map(|vs| {
            let price = vs.last().unwrap().0;
            (price, (vs[0].1, vs[1].1, vs[2].1, vs[3].1))
        })
        .collect()
}

fn store_value(
    memo: &mut HashMap<(i32, i32, i32, i32), u64>,
    sequence_prices: &Vec<(u64, (i32, i32, i32, i32))>,
) {
    let mut seen: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    for (price, sequence) in sequence_prices {
        if seen.contains(sequence) {
            continue;
        }

        if let Some(value) = memo.get(sequence) {
            memo.insert(*sequence, value + price);
        } else {
            memo.insert(*sequence, *price);
        }

        seen.insert(*sequence);
    }
}

fn sequence_values(starts: &Vec<u64>) -> Vec<((i32, i32, i32, i32), u64)> {
    let sequence_prices = starts
        .iter()
        .map(|start| sequence_prices(secret_sequence(*start, 2000, vec![first, second, third])))
        .collect_vec();

    let mut memo = HashMap::new();
    sequence_prices
        .iter()
        .for_each(|seq| store_value(&mut memo, seq));

    memo.iter().map(|(seq, total)| (*seq, *total)).collect_vec()
}

fn main() {
    let starts = parse_starts();

    let pt1 = starts
        .iter()
        .map(|start| secret(*start, 2000, vec![first, second, third]))
        .sum::<u64>();

    println!("pt1: {:?}", pt1);

    let mut values = sequence_values(&starts);
    values.sort_by(|a, b| a.1.cmp(&b.1));
    values.reverse();

    println!("pt2: {}", values[0].1);
}
