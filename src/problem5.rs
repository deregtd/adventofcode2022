use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem5.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let halves = &contents
        .split("\n\n").map(|e| e.split("\n").collect::<Vec<_>>()).collect::<Vec<_>>();
    
    let num_stacks = (halves[0][0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![];
    for i in 0..num_stacks {
        stacks.push(vec![]);
        for h in (0..halves[0].len() - 1).rev() {
            let ch = halves[0][h].as_bytes()[i * 4 + 1] as char;
            if ch != ' ' {
                stacks[i].push(ch);
            }
        }
    }

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let moves = halves[1].iter().map(|o| re.captures(o).unwrap().iter().skip(1).map(|s| s.unwrap().as_str().parse::<usize>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut pt1stacks = stacks.clone();
    for m in moves.clone() {
        for _i in 0..m[0] {
            let pulled = pt1stacks[m[1]-1].pop().unwrap();
            pt1stacks[m[2]-1].push(pulled);
        }
    }

    let pt1word = pt1stacks.iter().map(|s| s[s.len()-1]).collect::<Vec<_>>();
    println!("Part 1: {:?}", pt1word);

    let mut pt2stacks = stacks.clone();
    for m in moves.clone() {
        let from_stack = &mut pt2stacks[m[1]-1];
        let mut pulled = from_stack.splice(from_stack.len()-m[0]..from_stack.len(), []).collect::<Vec<_>>();
        let to_stack = &mut pt2stacks[m[2]-1];
        to_stack.append(&mut pulled);
    }

    let pt2word = pt2stacks.iter().map(|s| s[s.len()-1]).collect::<Vec<_>>();
    println!("Part 2: {:?}", pt2word);

    Ok(())
}
