use std::collections::HashMap;

fn flash(g: &mut HashMap<(i8, i8), i8>, r: i8, c: i8) -> u64 {
    let mut count = 1;
    g.insert((r, c), -1);
    for (dr, dc) in [ (-1,-1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        let next = (r+dr, c+dc);
        if let Some(v) = g.get_mut(&next) {
            if *v >= 0 {
                *v += 1;
                if *v >= 10 { count += flash(g, next.0, next.1); }
            }
        }
    }
    count
}

// Test -1656 flashes
fn main() {
    let mut grid = HashMap::new();
    for (r, line) in include_str!("input11.txt").lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid.insert((r as i8, c as i8), ch as i8 - '0' as i8);
        }
    }

    let mut count = 0;
    let mut step = 0;
    let mut full_sync = false;
    loop {
        full_sync = true;
        for (_, v) in grid.iter_mut() { 
            *v += 1; 
        }

        for r in 0..10 {
            for c in 0..10 {
                if grid[&(r,c)] == 10 { 
                    count += flash(&mut grid, r, c);
                }
            }
        }

        for (_, v) in grid.iter_mut() {
            if *v < 0  { *v = 0; }
            else { full_sync = false; }
        }

        step += 1;
        if step == 100 {
            println!("Part A: {}", count);
        }
        if full_sync { break; }
    }
    println!("Part B: {}", step);
}