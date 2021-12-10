
fn get_unmatched(s:&str) -> (Option<char>, Vec<char>) {
    let mut v = Vec::new();
    for c in s.chars() {
        match c {
            ')' => if v.last()==Some(&'(') { v.pop(); } else { return (Some(c), v) },
            ']' => if v.last()==Some(&'[') { v.pop(); } else { return (Some(c), v) }, 
            '}' => if v.last()==Some(&'{') { v.pop(); } else { return (Some(c), v) },
            '>' => if v.last()==Some(&'<') { v.pop(); } else { return (Some(c), v) },
            _ => v.push(c),
        }
    }
    (None, v)
}

fn main() {
    let dat = include_str!("input10.txt").lines();

    let mut a = 0;
    let mut b_vec = Vec::new();
    for line in dat {
        let (res,  v) = get_unmatched(line);
        match res {
            Some(')') => a += 3,
            Some(']') => a += 57,
            Some('}') => a += 1197,
            Some('>') => a += 25137,
            _ =>  if res.is_none() {
                    let b = v.iter()
                                .rev()
                                .fold(0 as i64, |acc, c|  {
                                    match c {
                                        '(' => acc*5 + 1,
                                        '[' => acc*5 + 2,
                                        '{' => acc*5 + 3,
                                        '<' => acc*5 + 4,
                                        _ => panic!(),
                                    }
                                });
                    b_vec.push(b);
                }
        }
    }
    println!("Part A: {}", a);
    b_vec.sort_unstable();
    println!("Part B: {}", b_vec[b_vec.len()/2]);
}   