use aoc_2024::read_input;

fn parse_numbers(nums: String) -> Vec<i32> {
    nums.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn is_increasing(nums: &Vec<i32>) -> bool {
    let mut prior = &nums[0];
    for current in &nums[1..] {
        if !(prior < current && (prior - current).abs() <= 3) {
            return false;
        }
        prior = current;
    }
    return true;
}

fn is_decreasing(nums: &Vec<i32>) -> bool {
    let mut prior = &nums[0];
    for current in &nums[1..] {
        if !(prior > current && (prior - current).abs() <= 3) {
            return false;
        }
        prior = current;
    }
    return true;
}

fn safe(nums: &Vec<i32>) -> bool {
    is_increasing(&nums) || is_decreasing(&nums)
}

fn dampened(nums: &Vec<i32>) -> bool {
    if safe(nums) {
        return true;
    }
    for (i, _) in nums.iter().enumerate() {
        let num_subset = [&nums[..i], &nums[i + 1..]].concat();
        if safe(&num_subset) {
            return true;
        }
    }
    false
}

fn main() {
    let numbers: Vec<Vec<i32>> = read_input().map(|l| parse_numbers(l.unwrap())).collect();

    println!("pt1: {}", numbers.iter().filter(|nums| safe(nums)).count());
    println!(
        "pt2: {}",
        numbers.iter().filter(|nums| dampened(nums)).count()
    );
}
