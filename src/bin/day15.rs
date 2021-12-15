use std::collections::{HashMap, BinaryHeap};

fn solve(grid: &HashMap<(i64, i64), i64>, dst_row: i64, dst_col: i64, nrows: i64, ncols: i64) -> i64 {
    let mut d: HashMap<(i64, i64), i64> = HashMap::new();
    let mut q: BinaryHeap<(i64, i64, i64)> = BinaryHeap::new();
    d.insert((0, 0), 0);
    q.push((0, 0, 0));

    while let Some((c, x, y)) = q.pop() {
        if x == dst_col && y == dst_row {
            return -c;
        }
        if -c > *d.get(&(x, y)).unwrap() { continue; }

        for (dx, dy) in &[(0,1), (0,-1), (1,0), (-1,0)] {
            let nx = x + dx;
            let ny = y + dy;

            if nx>=0 && nx <= dst_col && ny>=0 && ny<=dst_row {
                let mut grid_val = (grid.get(&(nx % ncols, ny % nrows)).unwrap() + (nx/ncols) + (ny/ncols)) % 9 ;
                if grid_val == 0 { grid_val = 9; }
                let next_cost = grid_val -c;
                if let Some(old_cost) = d.get(&(nx, ny)) {
                    if next_cost < *old_cost {
                        d.insert((nx, ny), next_cost);
                        q.push((-next_cost, nx, ny));
                    }
                } else {
                    d.insert((nx, ny), next_cost);
                    q.push((-next_cost, nx, ny));
                }
            }
        }
    }
    -1
}

fn main() {
    let grid_str = include_str!("input15.txt").lines().collect::<Vec<_>>();
    let nrows = grid_str.len() as i64;
    let ncols = grid_str[0].len() as i64;

    let grid = grid_str.iter()
        .enumerate()
        .map(|(y, l)| l.chars().enumerate()
            .map(|(x, c)| ((x as i64, y as i64), c as i64 - '0' as i64))
            .collect::<Vec<_>>())
        .flatten().collect();

    let part_a = solve(&grid, nrows-1, ncols-1, nrows, ncols);
    println!("Part A: {}", part_a);

    let part_b = solve(&grid, nrows*5-1, ncols*5-1, nrows, ncols);
    println!("Part B: {}", part_b);
}