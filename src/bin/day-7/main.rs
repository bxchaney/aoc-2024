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

fn is_valid_helper(calibration: &Calibration, i: usize, acc: u64, include_concat: bool) -> bool {
    if i == calibration.operands.len() {
        return acc == calibration.value;
    }
    if acc > calibration.value {
        return false;
    }

    (include_concat
        && is_valid_helper(
            calibration,
            i + 1,
            concat(acc, calibration.operands[i]),
            include_concat,
        ))
        || is_valid_helper(
            calibration,
            i + 1,
            acc + calibration.operands[i],
            include_concat,
        )
        || is_valid_helper(
            calibration,
            i + 1,
            acc * calibration.operands[i],
            include_concat,
        )
}

fn is_valid(calibration: &Calibration) -> bool {
    is_valid_helper(calibration, 1, calibration.operands[0], false)
}

fn is_valid_concat(calibration: &Calibration) -> bool {
    is_valid_helper(calibration, 1, calibration.operands[0], true)
}

fn main() {
    let calibrations: Vec<Calibration> = read_input()
        .map(|l| Calibration::from(&l.unwrap()))
        .collect();

    let pt1 = calibrations
        .iter()
        .filter_map(|cal| {
            if is_valid(&cal) {
                Some(cal.value)
            } else {
                None
            }
        })
        .sum::<u64>();

    println!("pt1: {}", pt1);

    let pt2 = calibrations
        .iter()
        .filter(|cal| !is_valid(&cal))
        .filter_map(|cal| {
            if is_valid_concat(&cal) {
                Some(cal.value)
            } else {
                None
            }
        })
        .sum::<u64>();

    println!("pt2: {}", pt1 + pt2);
}
