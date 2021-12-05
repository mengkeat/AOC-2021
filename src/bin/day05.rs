use std::collections::HashMap;

fn main() {
    let dat_str: Vec<&str> = include_str!("input05.txt").lines().collect();
    let str_to_coord = |s:&str| {
        let (s0, s1) = s.split_once(",").unwrap();
        (s0.parse::<i32>().unwrap(), s1.parse::<i32>().unwrap())
    };

    let mut a: HashMap< (i32,i32), i32> = HashMap::new(); 
    let mut b: HashMap< (i32,i32), i32> = HashMap::new(); 
    for lines in dat_str {
        let (start, end) = lines.split_once(" -> ").unwrap();
        let (x0, y0) = str_to_coord(start);
        let (x1, y1) = str_to_coord(end);
        
        if x0==x1 {
            for y in y0.min(y1)..=y0.max(y1) {
                *a.entry((x0,y)).or_default() += 1;
                *b.entry((x0,y)).or_default() += 1;
            }
        }
        else if y0==y1{
            for x in x0.min(x1)..=x0.max(x1) {
                *a.entry((x,y0)).or_default() += 1;
                *b.entry((x,y0)).or_default() += 1;
            }
        }
        else {
            let mut x = x0;
            let mut y = y0;
            while (x,y) != (x1,y1) {
                *b.entry((x,y)).or_default()  += 1;
               if x1>x0 { x+=1; } else { x-=1; };
               if y1>y0 { y+=1; } else { y-=1; };
            }
            *b.entry((x,y)).or_default()  += 1;
        }   
    }
    println!("Part A: {}", a.values().filter(|&x| *x>=2).count());
    println!("Part A: {}", b.values().filter(|&x| *x>=2).count());
}