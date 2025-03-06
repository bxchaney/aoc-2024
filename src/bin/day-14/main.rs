use aoc_2024::read_input;

#[derive(Debug)]
struct Robot {
    starting_position: (i32, i32),
    velocity: (i32, i32),
}

impl From<&str> for Robot {
    fn from(s: &str) -> Self {
        let starting_position;
        let velocity;
        if let Some((pos, vel)) = s.split_once(" ") {
            let (j, i) = &pos[2..].split_once(",").unwrap();
            starting_position = (i.parse::<i32>().unwrap(), j.parse::<i32>().unwrap());

            let (dj, di) = &vel[2..].split_once(",").unwrap();
            velocity = (di.parse::<i32>().unwrap(), dj.parse::<i32>().unwrap());
        } else {
            panic!("invalid format!");
        }

        Self {
            starting_position,
            velocity,
        }
    }
}

impl Robot {
    fn terminal_position(&self, steps: i32, area_width: i32, area_height: i32) -> (i32, i32) {
        let (i, j) = self.starting_position;
        let (di, dj) = self.velocity;

        let (mut term_i, mut term_j) = (
            (i + steps * di) % area_height,
            (j + steps * dj) % area_width,
        );

        if term_i < 0 {
            term_i += area_height;
        }
        if term_j < 0 {
            term_j += area_width;
        }

        (term_i, term_j)
    }
}

fn quadrant_score(robots: &Vec<Robot>, steps: i32, area_height: i32, area_width: i32) -> i32 {
    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
    for (i, j) in robots
        .iter()
        .map(|r| r.terminal_position(steps, area_width, area_height))
    {
        if i < area_height / 2 && j < area_width / 2 {
            a += 1;
        }
        if i < area_height / 2 && j >= area_width / 2 + 1 {
            b += 1;
        }
        if i >= area_height / 2 + 1 && j < area_width / 2 {
            c += 1;
        }
        if i >= area_height / 2 + 1 && j >= area_width / 2 + 1 {
            d += 1;
        }
    }
    a * b * c * d
}

fn robot_positions(robots: &Vec<Robot>, steps: i32, area_height: i32, area_width: i32) {
    let mut field: Vec<Vec<char>> = (0..area_height)
        .map(|_| ".".repeat(area_width as usize).chars().collect())
        .collect();

    for robot in robots {
        let (i, j) = robot.terminal_position(steps, area_width, area_height);
        field[i as usize][j as usize] = '*';
    }

    for row in field {
        println!("{}", row.iter().collect::<String>());
    }
}

fn main() {
    let robots: Vec<Robot> = read_input()
        .map(|l| Robot::from(l.unwrap().as_str()))
        .collect();

    let (area_width, area_height) = (101, 103);
    let steps = 100;

    println!(
        "pt1: {}",
        quadrant_score(&robots, steps, area_height, area_width)
    );

    println!();

    for step in 6668..6669 {
        println!("{}", step);
        robot_positions(&robots, step, area_height, area_width);
        println!();
    }
}
