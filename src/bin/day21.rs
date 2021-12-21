use std::collections::HashMap;

fn part_a(p1: i64, p2: i64) {
    let (mut p1_score, mut p2_score) = (0, 0);
    let (mut p1_pos, mut p2_pos) = (p1-1, p2-1);

    let mut nrolls = 0;
    let mut d = (-2, -1, 0);
    loop {
        d = (d.0+3, d.1+3, d.2+3);
        nrolls += 3;
        p1_pos = (p1_pos + d.0+d.1+d.2) % 10;
        p1_score += p1_pos+1;
        if p1_score>=1000 { break; }

        d = (d.0+3, d.1+3, d.2+3);
        nrolls += 3;
        p2_pos = (p2_pos + d.0+d.1+d.2) % 10;
        p2_score += p2_pos+1;
        if p2_score >= 1000 { break; }
    }
    let score = if p1_score>p2_score { p2_score * nrolls } else { p1_score * nrolls };
    println!("Part A: {}", score);
}

// Gets the 2 winner counts. Order does not matter
type DPTABLE = HashMap<(i64, i64, i64, i64), (i64, i64)>;
fn wins(p1: i64, p2: i64, s1: i64, s2: i64, dp: &mut DPTABLE) -> (i64, i64) {
    if dp.contains_key(&(p1, p2, s1, s2)) {
        return *dp.get(&(p1, p2, s1, s2)).unwrap()
    }
    else if s1 >= 21 {
        return (1, 0)
    }
    else if s2 >= 21 {
        return (0, 1)
    }
    else {
        let mut sofar = (0, 0);
        for a in 1..=3 {
            for b in 1..=3 {
                for c in 1..=3 {
                    let new_p1 = (p1 + a + b + c) % 10;
                    let (w1, w2) = wins(p2, 
                            new_p1, 
                            s2, 
                            s1 + new_p1 + 1, 
                            dp);
                    sofar = (sofar.0+w2, sofar.1+w1);
                }
            }
        }
        dp.insert((p1, p2, s1, s2), sofar);
        sofar
    }
}

fn main() {
    let p1_pos = 10;
    let p2_pos = 1;
    part_a(p1_pos, p2_pos);

    let (w1, w2) = wins(p1_pos-1, p2_pos-1, 0, 0, &mut HashMap::new());
    println!("Part B: {}", w1.max(w2));
}