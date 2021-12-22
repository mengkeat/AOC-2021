use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct Cube {
    x1: i64, y1: i64, z1: i64,
    x2: i64, y2: i64, z2: i64,
    on: bool
}

impl Cube {
    fn vol(&self) -> i64 {
        (self.x2 - self.x1) * (self.y2 - self.y1) * (self.z2 - self.z1)
    }
}

fn main() {
    let mut input_cubes = vec![];
    let mut xset = HashSet::from([-50, 50]);
    let mut yset = HashSet::from([-50, 50]);
    let mut zset = HashSet::from([-50, 50]);
    // for line in include_str!("input22_test.txt").lines() {
    for line in include_str!("input22.txt").lines() {
        let (a,b) = line.trim().split_once(" ").unwrap();
        let x = b.split(",").map(|s| {
                    let (l,r) = &s[2..].split_once("..").unwrap();
                    (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap())
                }).collect::<Vec<(i64,i64)>>();
        xset.insert(x[0].0); xset.insert(x[0].1+1);
        yset.insert(x[1].0); yset.insert(x[1].1+1);
        zset.insert(x[2].0); zset.insert(x[2].1+1);
        input_cubes.push(Cube {
            x1: x[0].0.min(x[0].1), 
            y1: x[1].0.min(x[1].1), 
            z1: x[2].0.min(x[2].1),
            x2: x[0].1.max(x[0].0)+1, 
            y2: x[1].1.max(x[1].0)+1, 
            z2: x[2].1.max(x[2].0)+1,
            on: a.chars().nth(1).unwrap() == 'n'
        });
    }
    let mut xs = xset.drain().collect::<Vec<i64>>();
    let mut ys = yset.drain().collect::<Vec<i64>>();
    let mut zs = zset.drain().collect::<Vec<i64>>();
    xs.sort(); ys.sort(); zs.sort();

    let rev_xs = xs.iter().enumerate().map(|(i,x)| (x,i)).collect::<HashMap<&i64,usize>>();
    let rev_ys = ys.iter().enumerate().map(|(i,x)| (x,i)).collect::<HashMap<&i64,usize>>();
    let rev_zs = zs.iter().enumerate().map(|(i,x)| (x,i)).collect::<HashMap<&i64,usize>>();

    let compute = |minr,maxr| {    
        let mut grid: HashMap<(i64, i64, i64), Cube> = HashMap::new();
        for inp in &input_cubes {
            let (x1,x2) = (rev_xs[&inp.x1.max(minr)], rev_xs[&inp.x2.min(maxr)]);
            let (y1,y2) = (rev_ys[&inp.y1.max(minr)], rev_ys[&inp.y2.min(maxr)]);
            let (z1,z2) = (rev_zs[&inp.z1.max(minr)], rev_zs[&inp.z2.min(maxr)]);
            
            for x in x1..x2 {
                for y in y1..y2 {
                    for z in z1..z2 {
                        if x+1<xs.len() && y+1<ys.len() && z+1<zs.len() {
                            let cube = Cube {
                                x1: xs[x], y1: ys[y], z1: zs[z],
                                x2: xs[x+1], y2: ys[y+1], z2: zs[z+1],
                                on: inp.on
                            };
                            grid.insert((cube.x1, cube.y1, cube.z1), cube);
                        }
                    }
                }
            }
        }   
        let total_vol = grid.values().filter(|c| c.on).map(|c| c.vol()).sum::<i64>();
        total_vol
    };
    println!("Part A: {}", compute(-50, 50));
    println!("Part B: {}", compute(i64::min_value(), i64::max_value()));
}