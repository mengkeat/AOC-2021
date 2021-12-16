
fn hex_to_binary(hex: &str) -> Vec<i16> {
    hex.chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap())
                .chars()
                .map(|c| if c=='1' {1} else {0}).collect::<Vec<i16>>()
        )
        .flatten()
        .collect::<Vec<_>>()
}

// Returns (version sum, num tokens consumed, expression value)
fn parse(inp: &[i16]) -> (i64, usize, i64) {
    if inp.len()==0 { // should not happen!?!?!
        panic!();
    }

    let ver = (inp[0]*4 + inp[1]*2 + inp[2]) as i64;
    let id = inp[3]*4 + inp[4]*2 + inp[5];
    if id==4 {  // literal
        let mut v: i64 = 0;
        let mut i = 6;
        loop {
            v = (v << 4) + (inp[i+1]*8 + inp[i+2]*4 + inp[i+3]*2 + inp[i+4]) as i64;
            i += 5;
            if inp[i-5]==0 { 
                break
            }
        }
        return (ver, i, v)
    }
    else { // operator
        let mut val_arr: Vec<i64> = Vec::new();
        let mut curr_idx: usize;
        let mut sum_ver: i64 = 0;

        if inp[6] == 0 { // next 15 bits are total length in bits of the subpackets
            let l = inp[7..7+15].iter().fold(0, |acc, &x| acc*2 + x) as usize;
            curr_idx = 7+15;
            while curr_idx < 7+15+l {
                let (v, i, val) = parse(&inp[curr_idx..]);
                sum_ver += v;
                curr_idx += i;
                val_arr.push(val);
            }
        }
        else {  // next 11 bits number of sub-packets
            let num_pkts = inp[7..7+11].iter().fold(0, |acc, &x| acc*2 + x);
            curr_idx = 7+11;
            for _ in 0..num_pkts {
                let (v, i, val) = parse(&inp[curr_idx..]);
                sum_ver += v;
                curr_idx += i;
                val_arr.push(val);
            }
        }

        let combined_val = match id {
            0 => val_arr.iter().sum(),
            1 => val_arr.iter().fold(1, |acc, &x| acc*x),
            2 => *val_arr.iter().min().unwrap(),
            3 => *val_arr.iter().max().unwrap(),
            5 => if val_arr[0] > val_arr[1] {1} else {0},
            6 => if val_arr[0] < val_arr[1] {1} else {0},
            7 => if val_arr[0] == val_arr[1] {1} else {0},
            _ => panic!("Unknown type ID!"),
        };
        return (ver + sum_ver, curr_idx, combined_val)
    }
}

fn main() {
    let dat = hex_to_binary(include_str!("input16.txt").trim());
    println!("Parts A & B : {:?}", parse(&dat));
}