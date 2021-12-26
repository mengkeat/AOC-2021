
fn evolve_till_stop(g: &Vec<Vec<char>>) -> i64 {
    let mut g1 = g.clone();
    let mut g2 = g.clone();
    let nrows = g.len();
    let ncols = g[0].len();
    
    let mut nsteps = 0;
    let mut moved = true;
    while moved {
        moved = false;
        for r in 0..nrows {
            for c in 0..ncols {
                match g1[r][c] {
                    '>' =>  {
                        let nc = (c+1) % ncols;
                        if g1[r][nc] == '.' {
                            g2[r][nc] = '>';
                            g2[r][c] = '.';
                            moved = true;
                        }
                    },
                    _ => (), 
                }
            }
        }
        g1 = g2.clone();
        for r in 0..nrows {
            for c in 0.. ncols {
                match g1[r][c] {
                    'v' => {
                        let nr = (r+1) % nrows;
                        if g1[nr][c] == '.' {
                            g2[nr][c] = 'v';
                            g2[r][c] = '.';
                            moved = true;
                        }
                    },
                    _ => (),
                }
            }
        }
        g1 = g2.clone();
        if moved { nsteps +=1; }

    }
    nsteps + 1
}

fn main() {
    let grid = include_str!("input25.txt").lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

    println!("Part A: {}", evolve_till_stop(&grid));
}