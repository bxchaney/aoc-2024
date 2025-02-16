use aoc_2024::read_input;

use regex::Regex;

enum Instruction {
    Mul((i32, i32)),
    Do,
    DoNot,
}

fn extract(expression: &str) -> (i32, i32) {
    let nums: Vec<i32> = expression[4..expression.len() - 1]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    (nums[0], nums[1])
}

fn parse_patterns(expression: &String) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    re.find_iter(&expression)
        .map(|exp| extract(exp.as_str()))
        .collect()
}

fn sum_product(operands: Vec<(i32, i32)>) -> i32 {
    operands.iter().map(|op| op.0 * op.1).sum()
}

fn parse_expression(exp: &str) -> Instruction {
    if exp.chars().next().unwrap() == 'm' {
        return Instruction::Mul(extract(exp));
    }
    if exp.len() == 4 {
        return Instruction::Do;
    }
    return Instruction::DoNot;
}

fn parse_patters_do_dont(expression: &String) -> Vec<Instruction> {
    let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();

    re.find_iter(&expression)
        .map(|exp| parse_expression(exp.as_str()))
        .collect()
}

fn main() {
    // for l in read_input() {
    //     parse_patterns(l.unwrap());
    // }

    let patterns: i32 = read_input()
        .map(|l| sum_product(parse_patterns(&l.unwrap())))
        .sum();

    println!("pt1: {}", patterns);

    let do_dont_patterns: Vec<Instruction> = read_input()
        .map(|l| parse_patters_do_dont(&l.unwrap()))
        .flatten()
        .collect();
    let mut enabled = true;
    let mut mul: Vec<(i32, i32)> = vec![];
    for pattern in do_dont_patterns {
        match pattern {
            Instruction::Mul(op) => {
                if enabled {
                    mul.push(op);
                }
            }
            Instruction::Do => {
                enabled = true;
            }
            Instruction::DoNot => {
                enabled = false;
            }
        }
    }
    println!("pt2: {}", sum_product(mul));
}
