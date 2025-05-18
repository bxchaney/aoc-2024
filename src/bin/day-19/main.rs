use aoc_2024::read_input;

use std::collections::{HashMap, HashSet};

struct PossibleComposite {
    _memo: HashMap<String, bool>,
}

impl From<&HashSet<String>> for PossibleComposite {
    fn from(options: &HashSet<String>) -> Self {
        Self {
            _memo: HashMap::from_iter(options.iter().map(|base| (base.clone(), true))),
        }
    }
}

impl PossibleComposite {
    pub fn is_possible(&mut self, comp: &String) -> bool {
        if comp.len() == 0 {
            return true;
        }

        if let Some(res) = self._memo.get(comp) {
            return *res;
        }

        if comp.len() == 1 {
            self._memo.insert(comp.clone(), false);
            return false;
        }

        for i in 0..comp.len() - 1 {
            let (prefix, suffix) = (&comp[0..i + 1].to_string(), &comp[i + 1..].to_string());
            if self.is_possible(&prefix) && self.is_possible(&suffix) {
                self._memo.insert(comp.clone(), true);
                return true;
            }
        }

        // could not find match
        self._memo.insert(comp.clone(), false);
        return false;
    }
}

fn parse_options(options: String) -> HashSet<String> {
    HashSet::from_iter(options.split(", ").map(|s| s.to_string()))
}

fn update_base_counts(
    options: &HashSet<String>,
    memo: &mut HashMap<String, u64>,
    possibility: &mut PossibleComposite,
) {
    let mut bases: Vec<String> = memo.iter().map(|(base, _)| base.clone()).collect();
    bases.sort_by(|a, b| a.len().cmp(&b.len()));

    // updating bases from smallest to largest in length
    for base in bases {
        let mut count = 1;
        for i in 0..base.len() - 1 {
            let (prefix, suffix) = (base[0..i + 1].to_string(), base[i + 1..].to_string());
            if options.contains(&prefix) {
                count += count_combinations(&suffix, options, memo, possibility)
            }
        }
        memo.insert(base, count);
    }
}

fn count_combinations(
    comp: &String,
    options: &HashSet<String>,
    memo: &mut HashMap<String, u64>,
    possibility: &mut PossibleComposite,
) -> u64 {
    if !possibility.is_possible(comp) {
        return 0;
    }

    if comp.len() == 0 {
        return 1;
    }

    if let Some(count) = memo.get(comp) {
        return *count;
    }

    let mut count = 0;
    for i in 0..comp.len() - 1 {
        let (prefix, suffix) = (comp[0..i + 1].to_string(), comp[i + 1..].to_string());
        if options.contains(&prefix) && possibility.is_possible(&suffix) {
            count += count_combinations(&suffix, options, memo, possibility)
        }
    }

    memo.insert(comp.clone(), count);
    count
}

fn combinations(
    options: &HashSet<String>,
    mut possibilities: &mut PossibleComposite,
    composites: &Vec<String>,
) -> Vec<(String, u64)> {
    let mut memo = HashMap::from_iter(options.iter().map(|base| (base.clone(), 1)));

    update_base_counts(options, &mut memo, possibilities);

    composites
        .iter()
        .map(|comp| {
            (
                comp.clone(),
                count_combinations(comp, &options, &mut memo, &mut possibilities),
            )
        })
        .collect()
}

fn main() {
    let mut lines = read_input();
    let options = parse_options(lines.next().unwrap().unwrap());

    lines.next();

    let composites: Vec<String> = lines.map(|line| line.unwrap()).collect();

    let mut possibilities = PossibleComposite::from(&options);

    let possible_composites = composites
        .iter()
        .filter(|comp| possibilities.is_possible(comp))
        .count();

    println!("pt1: {}", possible_composites);

    let combos = combinations(&options, &mut possibilities, &composites);

    println!("pt2: {}", combos.iter().map(|(_, i)| i).sum::<u64>());
}
