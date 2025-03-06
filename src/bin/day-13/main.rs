use aoc_2024::read_input;

#[derive(Debug)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Claw {
    a: Button,
    b: Button,
    prize: (i64, i64),
}

impl Claw {
    fn solution(&self) -> Option<(i64, i64)> {
        let det = self.a.y * self.b.x - self.b.y * self.a.x;
        let num = self.prize.0 * self.a.y - self.prize.1 * self.a.x;

        if num % det != 0 {
            return None;
        }

        let b = num / det;
        let a = (self.prize.0 - self.b.x * b) / self.a.x;

        if b < 0 || a < 0 || b > 100 || a > 100 {
            None
        } else {
            Some((a, b))
        }
    }

    fn expanded_solution(&self) -> Option<(i64, i64)> {
        let (new_x, new_y) = (
            self.prize.0 + 10_000_000_000_000,
            self.prize.1 + 10_000_000_000_000,
        );
        let det = self.a.y * self.b.x - self.b.y * self.a.x;
        let num = new_x * self.a.y - new_y * self.a.x;

        if num % det != 0 {
            return None;
        }

        let b = num / det;
        let a = (new_x - self.b.x * b) / self.a.x;

        if a * self.a.x != new_x - self.b.x * b {
            println!("division error");
            return None;
        }

        if b < 0 || a < 0 {
            None
        } else {
            Some((a, b))
        }
    }

    fn cost(&self) -> Option<i64> {
        match self.solution() {
            None => None,
            Some((a, b)) => Some(3 * a + b),
        }
    }

    fn expanded_cost(&self) -> Option<i64> {
        match self.expanded_solution() {
            None => None,
            Some((a, b)) => Some(3 * a + b),
        }
    }
}

fn parse_button(l: &str) -> Button {
    let x = l[12..14].parse::<i64>().unwrap();
    let y = l[18..].parse::<i64>().unwrap();
    Button { x, y }
}

fn parse_prize(l: &str) -> (i64, i64) {
    let s = &l[9..];
    let non_numeric = s.find(|c: char| !c.is_numeric());
    let (x_s, rest) = s.split_at(non_numeric.unwrap());
    let x = x_s.parse::<i64>().unwrap();
    let y = rest
        .chars()
        .skip_while(|c| !c.is_numeric())
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    (x, y)
}

fn main() {
    let mut claws = vec![];
    let mut lines = read_input();
    while let Some(l) = lines.next() {
        let a = parse_button(&l.unwrap());
        let b = parse_button(&lines.next().unwrap().unwrap());
        let prize = parse_prize(&lines.next().unwrap().unwrap());
        lines.next();
        claws.push(Claw { a, b, prize });
    }

    let pt1 = claws.iter().filter_map(|c| c.cost()).sum::<i64>();
    println!("pt1: {}", pt1);

    let pt2 = claws.iter().filter_map(|c| c.expanded_cost()).sum::<i64>();
    println!("pt2: {}", pt2);
    for (i, claw) in claws.iter().enumerate() {
        println!("{}: {:?}", i, claw.expanded_solution());
    }
}
