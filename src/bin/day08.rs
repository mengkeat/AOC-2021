use std::collections::HashMap;

fn encode(s: &str) -> i8 {
    s.chars().map(|c|  1<<(c as i8)-('a' as i8)).sum()
}

fn decode(x: &i8) -> String {
    (0..8).filter(|i| (1<<i)&x > 0).map(|i| (i+('a' as i8)) as u8 as char).collect()
}

fn check_valid(dict: &HashMap<char, char>, nums: &Vec<&str>, truth: &HashMap<i8, i8>) -> bool {
    let mut all_nums: i16  = 0;
    for s in nums {
        let s2 = s.chars().map(|c| dict.get(&c).unwrap()).collect::<String>();
        if let Some(encoded_num) = truth.get(&encode(&s2)) {
            all_nums += 1<<encoded_num;
        }
    }
    return all_nums == 1023 // all set to ones
}

fn permute8(_1: i8, _4: i8, _7: i8, _8: i8, a: &Vec<&str>, dict: &mut HashMap<char, char>,
            truth: &HashMap<i8, i8>) -> bool 
{
    let x = _8 & !(_4 | _7) & _8;
    let _8_permute: Vec<char> = decode(&x).chars().collect();
    dict.insert(_8_permute[0], 'e');
    dict.insert(_8_permute[1], 'g');
    if check_valid(dict, a, &truth) { return true }
    else { 
        dict.insert(_8_permute[1], 'e');
        dict.insert(_8_permute[0], 'g');
        return check_valid(dict, a, &truth) 
    }
}

fn permute4(_1: i8, _4: i8, _7: i8, _8: i8, a: &Vec<&str>, dict: &mut HashMap<char, char>, 
            truth: &HashMap<i8, i8>) -> bool 
{
    let x = _4 & !_1 & _4;
    let _4_permute: Vec<char> = decode(&x).chars().collect();
    dict.insert(_4_permute[0], 'b');
    dict.insert(_4_permute[1], 'd');
    if permute8(_1, _4, _7, _8, a, dict, &truth) { return true }
    else {
        dict.insert(_4_permute[1], 'b');
        dict.insert(_4_permute[0], 'd');
        return permute8(_1, _4, _7, _8, a, dict, &truth);
    }
}

fn solve(a: &Vec<&str>, dict: &mut HashMap<char, char>, truth: &HashMap<i8, i8>) {
    let mut _1: i8 = 0;
    let mut _4: i8 = 0;
    let mut _7: i8 = 0;
    let mut _8: i8 = 0;
    for s in a {
        let l = s.len();
        if l==2 { _1 = encode(s); }
        else if l==4 { _4 = encode(s); }
        else if l==3 { _7 = encode(s); }
        else if l==7 { _8 = encode(s); };
    }
    let x =  _7 & !_1 & _7;
    let c: char = decode(&x).chars().nth(0).unwrap(); 
    dict.insert(c, 'a');
    
    let _1_permute: Vec<char> = decode(&_1).chars().collect();
    dict.insert(_1_permute[0], 'c');
    dict.insert(_1_permute[1], 'f');
    if permute4(_1, _4, _7, _8, a, dict, &truth) {
        return;
    }
    else {
        dict.insert(_1_permute[1], 'c');
        dict.insert(_1_permute[0], 'f');
        if permute4(_1, _4, _7, _8, a, dict, &truth) == false {
            println!("Erorr cannot be solved!");
        }
    }
}

fn main() {
    let truth: HashMap<i8, i8>  = HashMap::from([
        (encode("cf"), 1), (encode("acdeg"), 2), (encode("acdfg"), 3), (encode("bcdf"), 4), 
        (encode("abdfg"), 5), (encode("abdefg"), 6), (encode("acf"), 7), (encode("abcdefg"), 8), 
        (encode("abcdfg"), 9), (encode("abcefg"), 0)
        ]);
        
    let dat = include_str!("input08.txt").lines()
    .map(|line| {
        let (a, b) = line.trim().split_once(" | ").unwrap();
        (a.trim().split(" ").collect::<Vec<_>>(), b.trim().split(" ").collect::<Vec<_>>())
    })
    .collect::<Vec<_>>();
    
    let mut part_a_count = 0;
    let mut part_b_count = 0;
    for (a, b) in &dat {
        // For solving part A
        for s in b {
            let l = s.len();
            if l==2 || l==3 || l==4 || l==7 { part_a_count += 1; }
        }
        
        // Its possible to try all permutations and verify but we can do quite a bit faster by
        // observation of certain patterns to eliminate permutations down to only 8. Computes 'instantly'
        let mut dict: HashMap<char, char> = HashMap::new();
        solve(a, &mut dict, &truth);

        let mut num: i64 = 0;
        for s in b {
            let s2 = s.chars().map(|c| dict.get(&c).unwrap()).collect::<String>();
            num = num*10 + truth[& encode(&s2)] as i64;
        }
        part_b_count += num;
    }
    println!("Part A: {}", part_a_count);
    println!("Part B: {}", part_b_count);
}