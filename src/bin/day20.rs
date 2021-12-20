use std::collections::HashSet;

fn enhance(img: &HashSet<(i64, i64)>, dict: &Vec<char>, on: bool) -> HashSet<(i64, i64)> {
    let mut out_img = HashSet::new();
    let min_r = img.iter().map(|(r, _)| *r).min().unwrap();
    let max_r = img.iter().map(|(r, _)| *r).max().unwrap();
    let min_c = img.iter().map(|(_, c)| *c).min().unwrap();
    let max_c = img.iter().map(|(_, c)| *c).max().unwrap();

    let get = |r_, c_| { if img.contains(&(r_, c_)) == on { 1i64 } else { 0i64 } };

    for r in min_r-10..=max_r+10 {
        for c in min_c-10..=max_c+10 {
            let n = (get(r-1, c-1)<<8) + (get(r-1, c)<<7) + (get(r-1, c+1)<<6) +
                        (get(r,   c-1)<<5) + (get(r,   c)<<4) + (get(r,   c+1)<<3) +
                        (get(r+1, c-1)<<2) + (get(r+1, c)<<1) + (get(r+1, c+1));
            if (dict[n as usize] == '#') == !on {
                out_img.insert((r,c));
            }
        }
    }
    out_img
}

fn main() {
    let dat = include_str!("input20.txt").lines().collect::<Vec<_>>();

    let dict = dat[0].trim().chars().collect::<Vec<_>>();
    let mut img = HashSet::new();
    for (y, line) in dat.iter().skip(2).enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '#' {
                img.insert((y as i64, x as i64)); 
            }
        }
    }
    let mut new_img= enhance(&enhance(&img, &dict, true), &dict, false);
    println!("Part A: {}", new_img.len());

    let mut on = true;
    for _ in 0..48 {
        new_img = enhance(&new_img, &dict, on);
        on = !on;
    }
    println!("Part B: {}", new_img.len());
}