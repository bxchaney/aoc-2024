use aoc_2024::read_input;

const DIRECTIONS: &'static [(i32, i32)] = &[
    // diagonal
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
    // lateral
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

const S_M: &'static [XMAS] = &[XMAS::S, XMAS::M];

#[derive(Debug, PartialEq, Eq)]
enum XMAS {
    X,
    M,
    A,
    S,
}

impl XMAS {
    fn next(&self) -> Option<XMAS> {
        match self {
            XMAS::X => Some(XMAS::M),
            XMAS::M => Some(XMAS::A),
            XMAS::A => Some(XMAS::S),
            XMAS::S => None,
        }
    }

    fn from(s: char) -> XMAS {
        match s {
            'X' => XMAS::X,
            'M' => XMAS::M,
            'A' => XMAS::A,
            'S' => XMAS::S,
            _ => panic!(),
        }
    }
}
fn out_of_bounds(board: &Vec<Vec<XMAS>>, i: i32, j: i32) -> bool {
    return i < 0 || j < 0 || i as usize >= board.len() || j as usize >= board[0].len();
}

fn check_direction(board: &Vec<Vec<XMAS>>, start: (i32, i32), direction: &(i32, i32)) -> bool {
    let mut symbol = XMAS::X;
    let (mut i, mut j) = start;

    while let Some(next_symbol) = symbol.next() {
        let next_i = i + direction.0;
        let next_j = j + direction.1;

        if out_of_bounds(board, next_i, next_j) {
            return false;
        }

        if board[next_i as usize][next_j as usize] != next_symbol {
            return false;
        }
        i = next_i;
        j = next_j;
        symbol = next_symbol;
    }

    true
}

fn is_cross(board: &Vec<Vec<XMAS>>, a: &(i32, i32), b: (i32, i32), c: (i32, i32)) -> bool {
    let b_i = a.0 + b.0;
    let b_j = a.1 + b.1;

    let c_i = a.0 + c.0;
    let c_j = a.1 + c.1;

    if out_of_bounds(board, b_i, b_j) || out_of_bounds(board, c_i, c_j) {
        return false;
    }

    let b_symbol = &board[b_i as usize][b_j as usize];
    let c_symbol = &board[c_i as usize][c_j as usize];

    b_symbol != c_symbol && S_M.contains(b_symbol) && S_M.contains(c_symbol)
}

fn is_x_mas(board: &Vec<Vec<XMAS>>, a: &(i32, i32)) -> bool {
    is_cross(board, a, (-1, -1), (1, 1)) && is_cross(board, a, (-1, 1), (1, -1))
}

fn matches(board: &Vec<Vec<XMAS>>, x: &(usize, usize)) -> i32 {
    let (i, j) = (x.0 as i32, x.1 as i32);
    DIRECTIONS
        .iter()
        .map(|d| check_direction(board, (i, j), d) as i32)
        .sum()
}

fn parse_line(l: String) -> Vec<XMAS> {
    l.chars().map(|c| XMAS::from(c)).collect()
}
fn main() {
    let crossword: Vec<Vec<XMAS>> = read_input().map(|l| parse_line(l.unwrap())).collect();

    let mut xs = vec![];
    let mut a_s = vec![];
    for i in 0..crossword.len() {
        for j in 0..crossword[i].len() {
            match crossword[i][j] {
                XMAS::X => {
                    xs.push((i, j));
                }
                XMAS::A => {
                    a_s.push((i, j));
                }
                _ => {}
            };
        }
    }
    println!("total a's: {}", a_s.len());

    println!(
        "pt1: {}",
        xs.iter().map(|x| matches(&crossword, x)).sum::<i32>()
    );

    println!(
        "pt2: {}",
        a_s.iter()
            .filter(|a| is_x_mas(&crossword, &(a.0 as i32, a.1 as i32)))
            .count()
    );
}
