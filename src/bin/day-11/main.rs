use aoc_2024::read_input;
use std::collections::HashMap;

fn numbers(l: &str) -> Vec<u64> {
    l.split_whitespace()
        .filter(|s| *s != "")
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn blink(num: u64) -> Vec<u64> {
    if num == 0 {
        vec![1]
    } else if num.ilog10() % 2 == 1 {
        let top_half = num / 10_u64.pow((num.ilog10() + 1) / 2);
        let bottom_half = num - (top_half * 10_u64.pow((num.ilog10() + 1) / 2));
        vec![top_half, bottom_half]
    } else {
        vec![num * 2024]
    }
}

fn children(num: u64, steps: u64, mut memo: &mut HashMap<(u64, u64), Vec<u64>>) -> Vec<u64> {
    if let Some(v) = memo.get(&(num, steps)) {
        return v.to_vec();
    }

    if steps == 0 {
        return vec![1];
    }

    let blinks = blink(num);
    let mut tiers = vec![blinks.len() as u64];

    if blinks.len() == 1 {
        tiers.extend(
            blinks
                .iter()
                .map(|child| children(*child, steps - 1, &mut memo))
                .flatten(),
        );
    } else {
        let descendents: Vec<Vec<u64>> = blinks
            .iter()
            .map(|child| children(*child, steps - 1, &mut memo))
            .collect();
        tiers.extend(
            descendents[0]
                .iter()
                .zip(&descendents[1])
                .map(|(a, b)| a + b),
        );
    }

    memo.insert((num, steps), tiers.clone());
    return tiers;
}

fn copies(nums: &Vec<u64>, steps: u64) -> u64 {
    let mut memo: HashMap<(u64, u64), Vec<u64>> = HashMap::new();
    nums.iter()
        .map(|num| {
            let descendents = children(*num, steps, &mut memo);
            descendents[descendents.len() - 1]
        })
        .sum()
}

fn main() {
    let nums: Vec<u64> = read_input()
        .map(|l| numbers(&l.unwrap()))
        .flatten()
        .collect();

    let pt1 = copies(&nums, 25);
    println!("pt1: {}", pt1);

    let pt2 = copies(&nums, 75);
    println!("pt1: {}", pt2);
}
