
fn count_ones(lines: &Vec<&str>) -> Vec<usize> {
    let mut one_count = vec![0usize; lines[0].len()];
    lines.iter().for_each(|line| {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                one_count[i] += 1;
            }
        }
    });
    one_count
}

fn filter_common<'a>(i: usize, v: &[&'a str], is_common: bool) -> Vec<& 'a str> {
    let mut one_count = 0;
    for line in v {
        if line.chars().nth(i).unwrap() == '1' {
            one_count += 1;
        }        
    }
    let zero_count = v.len() - one_count;

    let to_keep = if is_common {
        if one_count >= zero_count { '1' } else {'0'}       
    }
    else {
        if one_count < zero_count { '1' } else { '0' }
    };

    return v.iter()
     .filter(|line| line.chars().nth(i).unwrap() == to_keep)
     .map(|e| *e)
     .collect();
}

fn main() {
    let lines: Vec<&str> = include_str!("input03.txt").lines().collect();
    let numchars = lines[0].len();
    let th = lines.len()/2;

    let one_count = count_ones(&lines);
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, c) in one_count.iter().rev().enumerate() {
        if *c > th {
            gamma += 1<<i;
        }
        else {
            epsilon += 1<<i;
        }
    }
    println!("Part A: {}", gamma * epsilon);

    let mut common_lines: Vec<&str> = lines.clone();
    let mut uncommon_lines: Vec<&str> = lines.clone();
    for i in 0..numchars {
        if common_lines.len()>1 {
            common_lines = filter_common(i, &common_lines, true); 
        }
        if uncommon_lines.len()>1 {
            uncommon_lines = filter_common(i, &uncommon_lines, false);
        }
    }
    println!("Part B: {}", i64::from_str_radix(common_lines[0], 2).unwrap() * i64::from_str_radix(uncommon_lines[0], 2).unwrap());
}