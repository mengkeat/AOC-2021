
fn main() {
    let mut nums = include_str!("input07.txt").trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let m = if nums.len()%2==0 { nums.len()/2 } else { nums.len()/2+1 };
        
    // Median is optimal for L1 distance
    nums.sort();
    let mut c = nums.iter().fold(0, |acc, x| acc + (*x-nums[m]).abs());
    println!("Part A: {}", c);

    // Part B is convex. Gradient descent
    let cost2 = |i:i64| nums.iter().fold(0, |acc,x| { 
        let d = (x-i).abs();
        acc + d*(d+1)/2
    } );
    let mut guess = nums[m];
    loop {
        c = cost2(guess);
        let left = cost2(guess-1);
        let right = cost2(guess+1);
        if left<c { 
            guess -= 1;
        }
        else if right<c {
            guess += 1;
        }
        else {
            break;
        }
    }
    println!("Part B: {}", c);
}