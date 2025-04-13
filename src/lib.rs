use std::{
    env,
    fs::File,
    io::prelude::*,
    io::{BufReader, Lines},
    path::Path,
};

use num_traits::{AsPrimitive, PrimInt, Zero};

pub type InputFileBuffer = Lines<BufReader<File>>;

/// Assuming that the input file will be the first command line arg
pub fn read_input() -> InputFileBuffer {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough args!");
    }

    let path = Path::new(&args[1]);
    if let Ok(file) = File::open(&path) {
        return BufReader::new(file).lines();
    } else {
        panic!("failed to open file");
    }
}

pub fn out_of_bounds<T, U>(collection: &Vec<Vec<T>>, i: U, j: U) -> bool
where
    U: AsPrimitive<usize> + PrimInt,
{
    i < Zero::zero()
        || j < Zero::zero()
        || i.as_() >= collection.len()
        || j.as_() >= collection[0].len()
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn vector(&self) -> (i32, i32) {
        match *self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Right => (0, 1),
            Self::Left => (0, -1),
        }
    }

    pub fn turn_clockwise(&self) -> Self {
        match *self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn turn_counter_clockwise(&self) -> Self {
        match *self {
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
            Self::Right => Self::Up,
        }
    }

    pub fn opposite(&self) -> Self {
        match *self {
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
            Self::Right => Self::Left,
        }
    }
}

impl From<(i32, i32)> for Direction {
    fn from(vector: (i32, i32)) -> Self {
        match vector {
            (0, 1) => Self::Right,
            (0, -1) => Self::Left,
            (-1, 0) => Self::Up,
            (1, 0) => Self::Down,
            _ => panic!("invalid input!"),
        }
    }
}

impl From<&(i32, i32)> for Direction {
    fn from(vector: &(i32, i32)) -> Self {
        match *vector {
            (0, 1) => Self::Right,
            (0, -1) => Self::Left,
            (-1, 0) => Self::Up,
            (1, 0) => Self::Down,
            _ => panic!("invalid input!"),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("unrecognized character!"),
        }
    }
}
