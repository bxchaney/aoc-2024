use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use aoc_2024::read_input;

type Follower = HashMap<i32, HashSet<i32>>;

type Leader = HashMap<i32, HashSet<i32>>;

fn parse_record(l: &str) -> Vec<i32> {
    l.split(",").map(|s| s.parse::<i32>().unwrap()).collect()
}

fn add_follower(followers: &mut Follower, leaders: &mut Leader, s: &str) {
    if let Some((first_str, follower_str)) = s.split_once("|") {
        let first = first_str.parse::<i32>().unwrap();
        let follower = follower_str.parse::<i32>().unwrap();

        if let Some(entry) = followers.get_mut(&first) {
            entry.insert(follower);
        } else {
            let mut set = HashSet::new();
            set.insert(follower);

            followers.insert(first, set);
        }

        if let Some(entry) = leaders.get_mut(&follower) {
            entry.insert(first);
        } else {
            let mut set = HashSet::new();
            set.insert(first);

            leaders.insert(follower, set);
        }
    } else {
        panic!("invalid string format: -{}-", s);
    }
}

fn valid_record(followers: &Follower, leaders: &Leader, record: &Vec<i32>) -> bool {
    let mut preceding_set = HashSet::new();
    let mut following_set: HashSet<i32> = HashSet::from_iter(record.clone());

    // a record is valid if for every number, none of the things that should
    // follow appear ahead, and none of the things that should appear ahead, follow
    for num in record {
        following_set.remove(num);
        if let Some(follower) = followers.get(&num) {
            if follower.intersection(&preceding_set).count() > 0 {
                return false;
            }
        }
        if let Some(leader) = leaders.get(&num) {
            if leader.intersection(&following_set).count() > 0 {
                return false;
            }
        }
        preceding_set.insert(*num);
    }
    true
}

/// returns true if a should be after b
fn after(followers: &Follower, a: i32, b: i32) -> bool {
    // a should be after be if a is in b's followers set or if b is in a's leader
    if let Some(follower) = followers.get(&b) {
        if follower.contains(&a) {
            return true;
        }
    }
    // a should not be after b if b is in a's follower set
    // default to no swap
    false
}

fn bubble_sort(followers: &Follower, record: &Vec<i32>) -> i32 {
    let mut sorted_record = record.clone();
    loop {
        let mut swapped = false;
        for i in 0..(sorted_record.len() - 1) {
            if after(followers, sorted_record[i], sorted_record[i + 1]) {
                let a = sorted_record[i];
                sorted_record[i] = sorted_record[i + 1];
                sorted_record[i + 1] = a;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    sorted_record[sorted_record.len() / 2]
}

fn main() {
    let mut followers: Follower = HashMap::new();
    let mut leaders: Leader = HashMap::new();

    let mut lines = read_input();

    while let Some(res) = lines.next() {
        let s = res.unwrap();
        if s.len() == 0 {
            break;
        }
        add_follower(&mut followers, &mut leaders, &s);
    }

    let records: Vec<Vec<i32>> = lines.map(|l| parse_record(&l.unwrap())).collect();

    println!(
        "pt1: {}",
        records
            .iter()
            .filter(|r| valid_record(&followers, &leaders, *r))
            .map(|r| r[r.len() / 2])
            .sum::<i32>()
    );

    // for vec in records
    //     .iter()
    //     .filter(|r| !valid_record(&followers, &leaders, *r))
    // {
    //     println!("{:?}", bubble_sort(&followers, vec));
    // }

    println!(
        "pt2: {}",
        records
            .iter()
            .filter(|r| !valid_record(&followers, &leaders, *r))
            .map(|r| bubble_sort(&followers, r))
            .sum::<i32>()
    );
}
