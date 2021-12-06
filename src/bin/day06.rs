
fn evolve(nums: &[usize], d: i64) -> i64 {
    let mut day_count: [i64; 9]  = [0; 9];
    for n in nums { day_count[*n] += 1; }

    for _ in 0..d {
        let tmp = day_count[0];
        for i in 0..8 { day_count[i] = day_count[i+1]; }
        day_count[8] = tmp;
        day_count[6] += tmp;
    }
    day_count.iter().sum::<i64>()
}

fn main() {
    let nums = include_str!("input06.txt").trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!("Part A: {}", evolve(&nums, 80));
    println!("Part B: {}", evolve(&nums, 256));
}