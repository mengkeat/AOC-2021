use std::collections::HashMap;

fn evolve(rules: &HashMap<(char, char), char>, poly_map: &HashMap<(char, char), i64>) -> HashMap<(char, char), i64> {
    let mut new_poly_map: HashMap<(char, char), i64> = HashMap::new();
    for (&(c1, c2), &cnt) in poly_map {
        let new_char = rules.get(&(c1, c2)).unwrap();
        *new_poly_map.entry((c1, *new_char)).or_default() += cnt;
        *new_poly_map.entry((*new_char, c2)).or_default() += cnt;
    }
    new_poly_map
}

fn quantity(poly_map: &HashMap<(char, char), i64>, last_char: char) -> i64 {
    let mut hist: HashMap<char, i64> = HashMap::new();
    hist.insert( last_char,  1);
    for (&(c1, _), &cnt) in poly_map {
        *hist.entry(c1).or_default() += cnt;
    }
    let max_ = hist.values().max().unwrap();
    let min_ = hist.values().min().unwrap();
    max_ - min_
}

fn main() {
    let (polymer_str, rules_str) = include_str!("input14.txt").split_once("\n\n").unwrap();
    let mut poly_map: HashMap<(char, char), i64> = HashMap::new();
    for r in  polymer_str.chars().collect::<Vec<_>>().windows(2) {
        *poly_map.entry((r[0], r[1])).or_default() += 1;
    }
    let rules: HashMap<(char, char), char> = rules_str.lines()
        .map(|l | ((l.chars().nth(0).unwrap(), l.chars().nth(1).unwrap()), l.chars().nth(6).unwrap()) )
        .collect();

    for _ in 0..10 {
        poly_map = evolve(&rules, &poly_map);
    }
    println!("Part A: {}", quantity(&poly_map, polymer_str.chars().last().unwrap()));

    for _ in 0..30 {
        poly_map = evolve(&rules, &poly_map);
    }
    println!("Part B: {}", quantity(&poly_map, polymer_str.chars().last().unwrap()));
}