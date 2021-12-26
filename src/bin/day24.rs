
#[derive(Debug, Clone)]
enum VarNum { Var(char), Num(i64) }

#[derive(Debug, Clone)]
enum Instr {
    Add(char, VarNum),
    Mul(char, VarNum),
    Div(char, VarNum),
    Modulo(char, VarNum),
    Eql(char, VarNum),
}

fn run_block(prog: &Vec<Instr>, inp: i64, z: i64 ) -> i64 {
    let mut regs: Vec<i64> = vec![inp, 0, 0, z];
    let idx = |c: &char| { *c as usize - 'w' as usize };    

    for instr in prog {
        match instr {
            Instr::Add(c, v) => {
                match v {
                    VarNum::Var(c2) => regs[idx(c)] += regs[idx(c2)],
                    VarNum::Num(n) => regs[idx(c)] += *n,
                }
            },
            Instr::Mul(c, v) => {
                match v {
                    VarNum::Var(c2) => regs[idx(c)] *= regs[idx(c2)],
                    VarNum::Num(n) => regs[idx(c)] *= *n,
                }
            },
            Instr::Div(c, v) => {
                match v {
                    VarNum::Var(c2) => if regs[idx(c2)]==0 { panic!(); } else { regs[idx(c)] /= regs[idx(c2)] },
                    VarNum::Num(n) => if *n==0 { panic!() }  else { regs[idx(c)] /= *n },
                }
            },
            Instr::Modulo(c, v) => {
                match v {
                    VarNum::Var(c2) => {
                        if regs[idx(c)]<0 || regs[idx(c2)]<0 { panic!(); }
                        regs[idx(c)] %= regs[idx(c2)]
                    },
                    VarNum::Num(n) => {
                        if regs[idx(c)]<0 || *n<0 { panic!(); }
                        regs[idx(c)] %= *n
                    },
                }
            },
            Instr::Eql(c, v) => {
                match v {
                    VarNum::Var(c2) => regs[idx(c)] = if regs[idx(c)]==regs[idx(c2)] {1} else {0},
                    VarNum::Num(n) => regs[idx(c)] = if regs[idx(c)]==*n {1} else {0},
                }
            },
        }
    }
    regs[3]
}

fn search(b: &[Vec<Instr>], bnum: usize, prev_z: i64, inp: &mut Vec<i64>, max_val: &mut i64, min_val: &mut i64)  {
    if bnum>=b.len()-1 {
        for i in 1..=9 {
            if run_block(&b[bnum], i, prev_z) == 0 {
                inp.push(i);
                let n = inp.iter().fold(0, |acc, x| acc*10 + *x);
                if n > *max_val {
                    *max_val = n;
                }
                if n < *min_val {
                    *min_val = n;
                }
                inp.pop();
            }
        }
    }
    else {
        let new_z = (1..=9).map(|i| run_block(&b[bnum], i, prev_z)).collect::<Vec<_>>();
        if new_z.iter().all(|x| *x > prev_z) {
            for (i, z) in new_z.iter().enumerate() {
                inp.push(i as i64+1);
                search(&b, bnum+1, *z, inp, max_val, min_val);
                inp.pop();
            }
        }
        else {
            for (i, z) in new_z.iter().enumerate() {
                if *z == prev_z/26 {
                    inp.push(i as i64+1);
                    search(&b, bnum+1, *z, inp, max_val, min_val);
                    inp.pop();
                }
            }
        }
    }
}

fn main() {
    let get_varnum = |t: &str| { 
        let c = t.chars().nth(0).unwrap();
        if c=='w' || c=='x' || c=='y' || c=='z' {
            VarNum::Var(c)
        } else {
            VarNum::Num(t.parse::<i64>().unwrap())
        }};

    let mut block: Vec<Vec<Instr>> = vec![];
    for line in include_str!("input24.txt").lines() {
        let tok = line.split_whitespace().collect::<Vec<_>>();
        let a = tok[1].chars().next().unwrap();
        match tok[0] {
            "inp" => block.push(vec![]),
            "add" => block.last_mut().unwrap().push( Instr::Add(a, get_varnum(tok[2])) ),
            "mul" => block.last_mut().unwrap().push( Instr::Mul(a, get_varnum(tok[2])) ),
            "div" => block.last_mut().unwrap().push( Instr::Div(a, get_varnum(tok[2])) ),
            "mod" => block.last_mut().unwrap().push( Instr::Modulo(a, get_varnum(tok[2])) ),
            "eql" => block.last_mut().unwrap().push( Instr::Eql(a, get_varnum(tok[2])) ),
            _ => panic!(),
        }
    }

    let mut max_val = -1;
    let mut min_val = i64::max_value();
    search(&block, 0, 0, &mut vec![], &mut max_val, &mut min_val);
    println!("Part A: {}", max_val);
    println!("Part B: {}", min_val);
}


 /*              
inp w             w
mul x 0           
add x z           x = z
mod x 26          x = z % 26
div z 1           
add x 10          x = z % 26 + 10
eql x w           x = 1 if x == w
eql x 0           x = 1 if x == 0 // x always become 1
mul y 0       
add y 25  
mul y x           y = 25 * x 
add y 1           y = (25*x) + 1  // y = 26
mul z y           z = z * ( 25x + 1 )   // z = 26 * z
mul y 0           
add y w         
add y 6           y = w + 6  
mul y x           y = (w+6)*x  // y = w+6 
add z y           z = z *  (25*x + 1) + (w + 6) * x // z = z * 26 + w + 6


inp w
mul x 0
add x z
mod x 26      x = z % 26
div z 26      z = z / 26
add x -11     x = z % 26 - 11
eql x w       x = 1 if x == w     
eql x 0       x = 1 if x == 0    // x = 0 or 1
mul y 0
add y 25
mul y x      
add y 1      // y = 1 or 26
mul z y      // z = floor(z/26) or 26*floor(z/26)
mul y 0  
add y w     
add y 11    y = w + 11 
mul y x     y = 0 or w+11
add z y     z = floor(z/26) or 26*floor(z/26) + w+ 11

Note: Can only downsize (x==0) only if  z % 26 - 11 == w
*/