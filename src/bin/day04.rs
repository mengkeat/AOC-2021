use std::collections::{HashMap};

type BoardMap = HashMap<i32, usize>;
type BoardState = [bool; 25];

fn is_winner(state: BoardState) -> bool{
    let mut found = false;
    for i in 0..5 {
        let mut is_row = true;
        let mut is_col = true;
        for j in 0..5 {
            is_row &= state[i*5+j];
            is_col &= state[i+j*5];
        }
        found |= is_row;
        found |= is_col;
    }
    found
}

fn sum_score(m: &BoardMap, s: &BoardState) -> i32 {
    m.iter()
    .filter(|&(_, &pos)| !s[pos])
    .map(|(&n, _)| n)
    .sum::<i32>()
}

fn main() {
    let (num_str, board_str) = include_str!("input04.txt").split_once("\n\n").unwrap();

    let mut boards: Vec< (BoardMap, BoardState) > = board_str.split("\n\n")
        .map(|board| 
            (board.split_whitespace()
                .enumerate()
                .map(|(i, n)| { 
                    // println!("{}, {}", n.parse::<i32>().unwrap(), i);
                    (n.parse().unwrap(), i)  
                })  // map number -> position
                .collect(),
             [false; 25] )
        ).collect();

    let nums = num_str.split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let mut has_winner = false;
    let mut last_score = 0;
    for n in nums {
        for (bmap, state) in &mut boards {
            match bmap.get(&n) {
                Some(pos) => state[*pos] = true,
                None => {}
            }
        }
        for (m, state) in &boards {
            if !has_winner && is_winner(*state){
                has_winner = true;
                let s = sum_score(&m, &state);
                println!("Part A: {}", s*n);
            }

            if has_winner && is_winner(*state) {
                let s = sum_score(&m, &state);
                last_score = s * n;
            }
        }

        boards = boards.into_iter().filter(|(_, s)| !is_winner(*s)).collect();
    }

    println!("Part B: {}", last_score);
}