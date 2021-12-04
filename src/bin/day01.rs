fn main() {
   let data = include_str!("input01.txt")
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

    let part_a = data.windows(2)
                .filter(|pair| pair[0] < pair[1])
                .count();
    println!("Part A: {}", part_a);

    let part_b = data.windows(4)
                .filter(|s| s[3]>s[0])
                .count();
    println!("Part B: {}", part_b);
}
