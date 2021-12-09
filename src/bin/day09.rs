
use std::collections::{HashMap, HashSet};

fn main() {
    let cave: HashMap<(i32, i32), i32> = include_str!("input09.txt")
        .lines()
        .enumerate()
        .map(|(x, line)| {
        line.chars()
            .enumerate()
            .map(|(y, c)| 
                ((x as i32, y as i32), (c as i32)-('0' as i32)))
            .collect::<Vec<_>>()
        }).flatten().collect();

    let mut low_pts = Vec::new();
    let mut low_pt_vals = Vec::new();

    for ((r,c), v) in cave.iter() {
        let mut lowest = true;
        for (dr, dc) in &[(0,1), (0,-1), (1,0), (-1,0)] {
            if let Some(a) = cave.get(&(r+dr, c+dc)) {
                if a <= v { lowest = false; }
            }
        }
        if lowest { 
            low_pts.push((r, c)); 
            low_pt_vals.push(*v+1);
        }
    }
    println!("Part A: {}", low_pt_vals.iter().sum::<i32>());

    let mut sz = low_pts.iter()
        .map(|(&r, &c)| {
            let mut count = 0;
            let mut stack = vec![(r, c)];
            let mut visited= HashSet::from([(r,c)]);
            
            while let Some(curr) = stack.pop() {
                count += 1;
                let curr_h = cave.get(&curr).unwrap();
                for (dr, dc) in &[(0,1), (0,-1), (1,0), (-1,0)] {
                    let new_pt = (curr.0+dr, curr.1+dc);
                    if let Some(a) = cave.get(&new_pt) {
                        if a>curr_h && *a!=9 && !visited.contains(&new_pt) {
                            stack.push(new_pt);
                            visited.insert(new_pt);
                        }
                    }
                }
            }
            count
        }).collect::<Vec<_>>();
    sz.sort_by(|a, b| b.cmp(a));
    println!("Part B: {}", sz[0]*sz[1]*sz[2]);
}