use aoc_2024::read_input;

struct Calibration {
    value: u64,
    operands: Vec<u64>,
}

impl Calibration {
    fn from(s: &str) -> Self {
        if let Some(split) = s.split_once(":") {
            let value = split.0.parse::<u64>().unwrap();
            let operands = split
                .1
                .split_whitespace()
                .filter_map(|s| match s {
                    "" => None,
                    _ => Some(s.parse::<u64>().unwrap()),
                })
                .collect();
            return Calibration { value, operands };
        } else {
            panic!("invalid format!");
        }
    }
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn mult(a: u64, b: u64) -> u64 {
    a * b
}

fn concat(a: u64, b: u64) -> u64 {
    let order;
    if b < 10 {
        order = 1;
    } else if b < 100 {
        order = 2;
    } else {
        order = 3;
    }

    a * 10_u64.pow(order) + b
}

fn is_valid(calibration: &Calibration, functions: &Vec<fn(u64, u64) -> u64>) -> bool {
    let target = calibration.value;
    let mut stack = vec![(calibration.operands[0], &calibration.operands[1..])];
    while let Some(next) = stack.pop() {
        if next.1.len() == 0 {
            if next.0 == target {
                return true;
            } else {
                continue;
            }
        }

        if next.0 > target {
            continue;
        }

        stack.extend(
            functions
                .iter()
                .map(|f| (f(next.0, next.1[0]), &next.1[1..])),
        )
    }

    false
}

fn total_calibrations(calibrations: &Vec<Calibration>, methods: &Vec<fn(u64, u64) -> u64>) -> u64 {
    let validation = |cal| is_valid(cal, methods);
    calibrations
        .iter()
        .filter_map(|cal| {
            if validation(cal) {
                Some(cal.value)
            } else {
                None
            }
        })
        .sum::<u64>()
}

fn main() {
    let calibrations: Vec<Calibration> = read_input()
        .map(|l| Calibration::from(&l.unwrap()))
        .collect();

    let pt1_methods = vec![add, mult];

    let pt1 = total_calibrations(&calibrations, &pt1_methods);

    println!("pt1: {}", pt1);

    let pt2_methods = vec![add, mult, concat];

    let pt2 = total_calibrations(&calibrations, &pt2_methods);

    println!("pt2: {}", pt2);
}
