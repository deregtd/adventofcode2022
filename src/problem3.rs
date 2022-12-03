use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn conv_pri(c: char) -> i32 {
    if c >= 'A' && c <= 'Z' {
        return (c as i32) - ('A' as i32) + 27;
    } else if c >= 'a' && c <= 'z' {
        return (c as i32) - ('a' as i32) + 1;
    }
    println!("s: {:?}", c);
    return -1000;
}

fn conv_str(s: &str) -> Vec<i32> {
    return s.chars().map(|item| conv_pri(item)).collect();
}

fn to_set(l: &[i32]) -> HashSet<i32> {
    let mut ret = HashSet::new();
    for i in l {
        ret.insert(*i);
    }
    return ret;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem3.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let sacks = &contents
        .split("\n")
        .map(|r| conv_str(r))
        .collect::<Vec<Vec<i32>>>();

    let sacks1 = sacks.iter()
        .map(|sack| {
            let (s1, s2) = sack.split_at(sack.len()/2);
            let mut hs1 = to_set(s1);
            let hs2 = to_set(&s2);
            hs1.retain(|e| hs2.contains(e));
            return hs1;
        })
        .collect::<Vec<HashSet<i32>>>();

    let mut pt1score = 0;
    for sack in sacks1 {
        for i in sack {
            pt1score += i;
        }
    }

    println!("Part 1: {:?}", pt1score);

    // dedupe each stack for the next part
    let sacksdedup = sacks.iter().map(|s| {
        let mut sc = s.clone();
        sc.sort();
        sc.dedup();
        return sc;
    }).collect::<Vec<Vec<i32>>>();

    // split into groups of three and find the intersection across them
    let mut pt2score = 0;
    for i in (0..sacksdedup.len()).step_by(3) {
        let mut intoverall = sacksdedup[i].clone();
        intoverall.retain(|e| sacksdedup[i+1].contains(e) && sacksdedup[i+2].contains(e));
        pt2score += intoverall[0];
    }

    println!("Part 2: {:?}", pt2score);

    Ok(())
}
