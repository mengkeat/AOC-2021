use std::collections::{HashSet};

// Encode one of 48 transformations into a transform
fn do_transform(x: &(i32, i32, i32), n: usize) -> (i32, i32, i32) {
    let (mut a, mut b, mut c) = match n/8 {
        0 => (x.0, x.1, x.2),
        1 => (x.0, x.2, x.1),
        2 => (x.1, x.0, x.2),
        3 => (x.1, x.2, x.0),
        4 => (x.2, x.0, x.1),
        5 => (x.2, x.1, x.0),
        _ => panic!(),
    };
    if n%8 & 1 == 1 { a *= -1; }
    if (n%8)>>1 & 1 == 1 { b *= -1; }
    if (n%8)>>2 & 1 == 1 { c *= -1; }
    (a, b, c)
}

struct Scanner{
    obs: HashSet<(i32, i32, i32)>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { obs: HashSet::new(), }
    }
    
    // Gets transformation that puts points from others to self
    // Gets (Transform number, offset dx, offset dy, offset dz)
    fn merge(&mut self, other: &Scanner) -> Option<(usize, i32, i32, i32)> {
        for rot_num in  0..48 {
            let rot_pts = other.obs.iter().map(|x| do_transform(x, rot_num)).collect::<Vec<_>>();
            let tx: Vec<(i32, i32, i32)> = self.obs.iter().map(|a| 
                    rot_pts.iter().map(|b| (a.0-b.0, a.1-b.1, a.2-b.2) )
                )
                .flatten().collect();
            for (dx, dy, dz) in tx {
                let new_pts = rot_pts.iter().map(|&(a,b,c)| (a+dx, b+dy, c+dz)).collect::<Vec<_>>();
                if new_pts.iter().filter(|&x| self.obs.contains(x)).count() >= 12 {
                    self.obs.extend(new_pts.clone().iter());
                    return Some((rot_num, dx, dy, dz))
                }
            }
        }
        None
    }
}

fn merge_all_scans(scans: &Vec<Scanner>) -> Scanner {
    let mut master_scan = Scanner::new();
    master_scan.obs = scans[0].obs.clone();
    let mut used = vec![false; scans.len()];
    used[0] = true;

    let mut all_tx = vec![(0,0,0)];
    while used.iter().fold(true, |a, b| a & b) == false {
        for (i, s) in scans.iter().enumerate() {
            if !used[i] {
                if let Some((rot_num, dx, dy, dz)) = master_scan.merge(s) {
                    used[i] = true;
                    println!("Merged in scan {}", i);
                    all_tx.push((dx, dy, dz));
                }
            }
        }
    }   
    let mut dists = vec![];    
    for i in 1..all_tx.len()-1 {
        for j in i+1..all_tx.len() {
            dists.push( (all_tx[i].0-all_tx[j].0).abs() + (all_tx[i].1-all_tx[j].1).abs() + (all_tx[i].2-all_tx[j].2).abs() );
        }
    }
    println!("Part B: {}", dists.iter().max().unwrap());

    master_scan
}

fn main() {
    let mut scanners: Vec<Scanner> = Vec::new();
    
    // include_str!("input19_test.txt").lines().for_each(|line| {
    include_str!("input19.txt").lines().for_each(|line| {
        if line.len()>0 {
            if &line[0..3] == "---" {
                scanners.push(Scanner::new());
            }
            else {
                let l = scanners.len();
                let c = line.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                scanners[l-1].obs.insert((c[0], c[1], c[2]));
            }
        }
    });
    let final_scan = merge_all_scans(&scanners);
    println!("Part A: {}", final_scan.obs.len());
}