use std::collections::{HashMap, HashSet};

fn explore<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    curr_node: &'a str, 
    curr_path: &mut Vec<&'a str>,
    twice: bool ) -> u32 
{
    let all_caps = |s:&str| s.chars().fold(true, |acc, c| acc && c.is_uppercase());
    let mut count = 0;
    if curr_node=="end" { return 1  }

    let visited: HashSet<&str> = curr_path.iter().map(|s| *s).collect();
    for &next_node in graph.get(curr_node).unwrap() {
        let mut to_explore = true;
        let mut new_twice = twice;

        if !all_caps(next_node) {
            if !visited.contains(next_node) { 
                to_explore = true; 
            }
            else if visited.contains(next_node) && new_twice==false && next_node != "start" { // small caps and not twice
                to_explore = true;
                new_twice = true;
            } 
            else { 
                to_explore = false; 
            }
        }
        if to_explore==true {
            curr_path.push(next_node);
            count += explore(graph, next_node, curr_path, new_twice);
            curr_path.pop();
        }
    }
    count
}   

fn main() {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in  include_str!("input12.txt").lines() {
        if let Some((from, to)) = line.split_once("-") {
            graph.entry(from).or_default().push(to);
            graph.entry(to).or_default().push(from);
        }
    }
    let mut curr_path = vec!["start"];
    curr_path.reserve(graph.len());
    let part_a= explore(&graph, "start", &mut curr_path, true);
    println!("Part A: {}", part_a);

    curr_path = vec!["start"];
    let part_b = explore(&graph, "start", &mut curr_path, false);
    println!("Part B: {}", part_b);
}