use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem4.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let pairs = &contents
        .split("\n")
        .map(|r| r.split(",").map(|p| p.split("-").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pt1pairs = pairs.clone();
    pt1pairs.retain(|p| {
        if p[0][1] - p[0][0] <= p[1][1] - p[1][0] {
            // first is smaller or equal to second
            return p[0][0] >= p[1][0] && p[0][1] <= p[1][1];
        }
        return p[1][0] >= p[0][0] && p[1][1] <= p[0][1];
    });

    println!("Part 1: {:?}", pt1pairs.len());

    let mut pt2pairs = pairs.clone();
    pt2pairs.retain(|p| {
        let mut ps = p.clone();
        // sort them by smallest fist element to make the comparison easy
        if ps[0][0] > ps[1][0] {
            ps = vec![ps[1].clone(), ps[0].clone()];
        }
        return ps[1][0] <= ps[0][1];
    });

    println!("Part 2: {:?}", pt2pairs.len());

    Ok(())
}
