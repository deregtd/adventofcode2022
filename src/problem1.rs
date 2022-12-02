use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut elves = contents
        .split("\n\n")
        .map(|x|
            x.split("\n")
            .map(|x|
                x.parse::<i32>()
                .unwrap_or_default()
            )
            .reduce(|a, b| a + b)
            .unwrap()
        )
        .collect::<Vec<i32>>();
    elves.sort();
    println!("{:?}", elves.last().unwrap());
    println!("{:?}", elves[elves.len()-1] + elves[elves.len()-2] + elves[elves.len()-3]);
    Ok(())
}
