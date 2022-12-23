use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq)]
enum Dir {
    None,
    Right,
    Down,
    Left,
    Up,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem23.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut elves = contents.split("\n").map(|r| r.chars().map(|c| c == '#' ).collect::<Vec<_>>()).collect::<Vec<_>>();

    // Pt 1
    // for round in 0..10 {
    // Pt 2
    for round in 0..1000 {
        // see if we need to expand -- max of expand 1 per round
        let edge = elves[0].iter().any(|e| *e) || elves[elves.len()-1].iter().any(|e| *e) || elves.iter().any(|r| r[0] || r[r.len()-1]);
        if edge {
            println!("Expanding in round {}", round);
            // add new cols to left/right
            elves = elves.iter().map(|r| {
                let mut newr = r.clone();
                newr.insert(0, false);
                newr.push(false);
                return newr;
            }).collect::<Vec<_>>();
            // top/bottom
            elves.insert(0, vec![false; elves[0].len()]);
            elves.push(vec![false; elves[0].len()]);
        }

        let round_mod = round % 4;

        let mut proposals = elves.iter().map(|r| vec![Dir::None; r.len()]).collect::<Vec<_>>();
        let mut proposal_targets = elves.iter().map(|r| vec![vec![]; r.len()]).collect::<Vec<_>>();
        for y in 0..elves.len() {
            for x in 0..elves[0].len() {
                if !elves[y][x] {
                    continue;
                }

                let above = y > 0 && ((x > 0 && elves[y-1][x-1]) || elves[y-1][x] || (x < elves[0].len() - 1 && elves[y-1][x+1]));
                let below = y < elves.len() - 1 && ((x > 0 && elves[y+1][x-1]) || elves[y+1][x] || (x < elves[0].len() - 1 && elves[y+1][x+1]));
                let left = x > 0 && ((y > 0 && elves[y-1][x-1]) || elves[y][x-1] || (y < elves.len() - 1 && elves[y+1][x-1]));
                let right = x < elves[0].len() - 1 && ((y > 0 && elves[y-1][x+1]) || elves[y][x+1] || (y < elves.len() - 1 && elves[y+1][x+1]));
                if !above && !below && !left && !right {
                    // don't move
                    continue;
                }

                let mut prop_list = vec![];
                prop_list.push(if !above { Dir::Up } else { Dir::None });
                prop_list.push(if !below { Dir::Down } else { Dir::None });
                prop_list.push(if !left { Dir::Left } else { Dir::None });
                prop_list.push(if !right { Dir::Right } else { Dir::None });
                for p in 0..=3 {
                    let index = (p + round_mod) % 4;
                    if prop_list[index] != Dir::None {
                        proposals[y][x] = prop_list[index];
                        match prop_list[index] {
                            Dir::Right => { proposal_targets[y][x+1].push((x, y)); },
                            Dir::Left => { proposal_targets[y][x-1].push((x, y)); },
                            Dir::Up => { proposal_targets[y-1][x].push((x, y)); },
                            Dir::Down => { proposal_targets[y+1][x].push((x, y)); },
                            _ => todo!(),
                        };
                        break;
                    }
                }
            }
        }

        println!("Proposals:");
        for r in &proposals {
            println!("{:?}", r);
        }

        // cull overlapping proposals
        for y in 0..elves.len() {
            for x in 0..elves[0].len() {
                if proposal_targets[y][x].len() < 2 {
                    continue;
                }
                println!("Culling {} {}", x, y);
                for target in &proposal_targets[y][x] {
                    proposals[target.1][target.0] = Dir::None;
                }
            }
        }

        // now move
        let mut any_moved = false;
        for y in 0..elves.len() {
            for x in 0..elves[0].len() {
                if proposals[y][x] == Dir::None {
                    continue;
                }

                any_moved = true;
                elves[y][x] = false;
                match proposals[y][x] {
                    Dir::Right => elves[y][x+1] = true,
                    Dir::Left => elves[y][x-1] = true,
                    Dir::Up => elves[y-1][x] = true,
                    Dir::Down => elves[y+1][x] = true,
                    _ => todo!(),
                };
            }
        }

        if !any_moved {
            println!("No changes at round {}", round + 1);
            break;
        }

        // contract?
        if !elves[0].iter().any(|e| *e) {
            elves.remove(0);
        }
        if !elves[elves.len()-1].iter().any(|e| *e) {
            elves.remove(elves.len()-1);
        }
        if !elves.iter().any(|r| r[0]) {
            elves = elves.iter().map(|r| {
                let mut newr = r.clone();
                newr.remove(0);
                return newr;
            }).collect::<Vec<_>>();
        }
        if !elves.iter().any(|r| r[r.len() - 1]) {
            elves = elves.iter().map(|r| {
                let mut newr = r.clone();
                newr.remove(r.len()-1);
                return newr;
            }).collect::<Vec<_>>();
        }

        println!("Elves:");
        for r in &elves {
            println!("{:?}", r.iter().map(|c| if *c { "#" } else { "." }).collect::<Vec<_>>().join(""));
        }

        let empty_count: usize = elves.iter().map(|r| r.iter().filter(|c| !**c).count()).sum();

        println!("Round {}: {} by {}: {} empty", round, elves[0].len(), elves.len(), empty_count);
    }

    Ok(())
}
