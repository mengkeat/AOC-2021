
// target area: x=139..187, y=-148..-89
const xmin: i64 = 139;
const xmax: i64 = 187;
const ymin: i64 = -148;
const ymax: i64 = -89;

fn main() {
    let mut min_dx: i64 = 1;
    for i in 1..xmin {
        if i/2*(1+i) > xmin { 
            min_dx = i-1;
            break;
        }
    } 
    let mut global_max_y = 0;
    let mut num_soln = 0;
    for start_dx in min_dx..=xmax+1 {
        for start_dy in ymin..i64::abs(ymin).max(i64::abs(ymax)) {
            let mut dx = start_dx;
            let mut dy = start_dy;
            let mut x = 0;
            let mut y = 0;
            let mut traj_max_y = 0;
            let mut hit_target = false;
            while y > ymin {
                x += dx;
                y += dy;
                // not specializing for general case. Our input is such that dx > 0
                dx = if dx > 0 { dx-1 } else { 0 }; 
                dy -= 1;
                traj_max_y = if y>traj_max_y { y } else { traj_max_y };
                if x>=xmin && x<=xmax && y>=ymin && y<=ymax {
                    hit_target = true;
                    num_soln += 1;
                    break;
                }
            }
            if hit_target && (traj_max_y>global_max_y) {
                global_max_y = traj_max_y;
            }
        }
    }
    println!("Part A: {}", global_max_y);
    println!("Part B: {}", num_soln);
}