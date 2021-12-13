use std::collections::HashSet;

fn do_fold(grid: &mut HashSet<(i32, i32)>, (axis, coord): (char, i32)) {
    if axis == 'x' {
        for (x,y) in grid.iter().filter(|&&(x, _)| x>coord ).map(|&c| c).collect::<Vec<(_,_)>>() {
            grid.insert((coord-(x-coord), y)); 
            grid.remove(&(x, y));
        }
    }
    else {
        for (x,y) in grid.iter().filter(|&&(_, y)| y>coord ).map(|&c| c).collect::<Vec<(_,_)>>() {
            grid.insert((x, coord-(y-coord)));
            grid.remove(&(x, y));
        }
    }
}

fn main() {
    let (dat, fold_str) = include_str!("input13.txt").trim().split_once("\n\n").unwrap();
    let mut grid: HashSet<(i32,i32)> = dat.lines().map(|d| {
            let (x_str, y_str) = d.split_once(",").unwrap();
            (x_str.parse::<i32>().unwrap(), y_str.parse::<i32>().unwrap())
        })
        .collect();
    let folds: Vec<(char, i32)> = fold_str.lines()
        .map(|d| {
            let (a,b) = d.split_once("=").unwrap();
            let axis =  a.chars().nth(a.len()-1).unwrap();
            let coord = b.parse::<i32>().unwrap();  
            (axis, coord)
        })
        .collect();

    do_fold(&mut grid, folds[0]);
    println!("Part A: {}", grid.len());

    for fo in folds.iter().skip(1) {
        do_fold(&mut grid, *fo);
    }
    let max_x = grid.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = grid.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    println!("Part B:");
    for y in 0..=max_y {
        for x in 0..=max_x  {
            if grid.contains(&(x,y)) { print!("#"); }
            else { print!("."); }
        }
        println!();
    }
}