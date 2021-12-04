

fn main() {
    let inp = include_str!("input02.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .collect::<Vec<_>>();

    let part_a = inp.iter().fold( (0,0), 
                    |(horiz, depth), (instr, n)| {
                        match (*instr, n.parse::<i32>().unwrap()) {
                            ("forward", n) => (horiz + n, depth),
                            ("down", n) => (horiz, depth + n),
                            ("up", n) => (horiz, depth-n),
                            _ => unreachable!()
                        }
                    });
    println!("Part A:: {}", part_a.0 * part_a.1);

    let part_b = inp.iter().fold( (0,0,0), 
                    |(horiz, depth, aim), (instr, n)| {
                        match (*instr, n.parse::<i32>().unwrap()) {
                            ("forward", n) => (horiz + n, depth+aim*n, aim),
                            ("down", n) => (horiz, depth, aim+n),
                            ("up", n) => (horiz, depth, aim-n),
                            _ => unreachable!()
                        }
                   });
    println!("Part B:: {}", part_b.0 * part_b.1);
}