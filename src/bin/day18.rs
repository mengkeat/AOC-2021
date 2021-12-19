
#[derive(Debug, Clone)]
enum Node {
    Val(i64),
    Open,
    Close,
}

fn make_tree(inp: &str) -> Vec<Node> {
    let mut tok: Vec<Node> = Vec::new();
    for c in inp.chars() {
        match c {
            '[' => tok.push(Node::Open),
            ']' => tok.push(Node::Close),
            ',' => (),
            _ => tok.push(Node::Val(c.to_digit(10).unwrap() as i64)),
        }
    }
    tok
}

fn explode(tok: &Vec<Node>) -> (bool, Vec<Node>) {
    let mut stack: Vec<Node> = Vec::new();    
    let mut carry : i64 = 0;
    let mut depth: u16 = 0;
    let mut modif = false;
    for n in tok {
        match n {
            Node::Open => {
                stack.push(Node::Open);
                depth += 1;
            },
            Node::Val(x) => {
                stack.push(Node::Val(x+carry));
                carry = 0;
            },
            Node::Close => { // Assume that max depth 4 
                if depth > 4 {
                    carry = if let Node::Val(x) = stack.pop().unwrap() { x } else { panic!() };
                    let back_carry = if let Node::Val(x) = stack.pop().unwrap() {x} else {panic!()};
                    stack.pop(); // Remove last bracket
                    let mut i = stack.len();
                    while i > 0 {
                        i -= 1;
                        if let Node::Val(x) = stack[i] {
                            stack[i] = Node::Val(x + back_carry);
                            break;
                        }
                    }
                    stack.push(Node::Val(0));
                    modif = true;
                }
                else {
                    stack.push(Node::Close);
                }
                depth -= 1;
            },
        }
    }
    (modif, stack)
}

fn split(tok: &Vec<Node>) -> (bool, Vec<Node>) {
    let mut stack: Vec<Node> = Vec::new();
    let mut modif = false;
    for n in tok {
        match n {
            Node::Val(x) => {
                if *x > 9 && !modif {
                    stack.push(Node::Open);
                    stack.push(Node::Val(*x/2));
                    stack.push(Node::Val(*x-*x/2));
                    stack.push(Node::Close);
                    modif = true;
                }
                else {
                    stack.push(Node::Val(*x));
                }
            },
            a => stack.push(a.clone()),
        }
    }
    (modif, stack)
}

fn reduce(tok: &Vec<Node>) -> Vec<Node> {
    let mut new_v: Vec<Node> = tok.clone();
    loop {
        let (m1, v1) = explode(&new_v);
        let (m2, v2) = split(&v1);
        new_v = v2.clone();
        if !m1 && !m2 { break; }
        // println!("{} {}", m1, m2);
        // show(&new_v);
    }
    new_v
}

fn add(t1: &Vec<Node>, t2: &Vec<Node>) -> Vec<Node> {
    let mut t = vec![Node::Open];
    t.append(&mut t1.clone());
    t.append(&mut t2.clone());
    t.push(Node::Close);
    reduce(&t)
}

fn magnitude(t: &Vec<Node>) -> i64 {
    let mut stack: Vec<i64> = Vec::new();
    for n in t {
        match n {
            Node::Val(x) => stack.push(*x),
            Node::Close => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a*2+b*3);
            },
            _ => (),
        }
    }
    stack[0]
}

fn show(t: &Vec<Node>) {
    for x in t {
        match x {
            Node::Val(x) => print!("{},", x),
            Node::Open => print!("["),
            Node::Close => print!("]"),
        }
    }
    println!();
}

fn main() {
    let nums = include_str!("input18_test.txt").lines()
    // let nums = include_str!("input18.txt").lines()
            .map(|l| make_tree(l) )
            .collect::<Vec<_>>();

    let mut v = nums[0].clone();
    for v2 in nums.iter().skip(1) {
        v = add(&v, v2);
    }
    println!("Part A: {}", magnitude(&v));

    let mut m: i64 = 0;
    for i in 0..nums.len()-1 {
        for j in i+1.. nums.len() {
            let v1  = add(&nums[i], &nums[j]);
            m = m.max(magnitude(&v1));
            let v2 = add(&nums[j], &nums[i]);
            m = m.max(magnitude(&v2));
        }
    }
    println!("Part B: {}", m);
}