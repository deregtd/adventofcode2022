use std::fs::File;
use std::io::prelude::*;
use core::ops::Add;
use std::collections::HashSet;

#[derive(Clone, Debug, Copy, PartialEq)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
struct Pt {
    x: i32,
    y: i32,
}

impl Add for Pt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Blizzard {
    pos: Pt,
    dir: Facing,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem24.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let rows = contents.split("\n").collect::<Vec<_>>();
    let map = rows.iter().map(|r| r.chars().map(|c| c == '#' ).collect::<Vec<_>>()).collect::<Vec<_>>();
    let start = Pt { x: map[0].iter().position(|c| !*c).unwrap() as i32, y: 0 };
    let end = Pt { x: map[map.len()-1].iter().position(|c| !*c).unwrap() as i32, y: (map.len() as i32) - 1 };

    let mut blizzards = vec![];
    for y in 0..rows.len() {
        let row = rows[y];
        let rowb = row.as_bytes();
        for x in 0..row.len() {
            let c = rowb[x] as char;
            if c == '^' { blizzards.push(Blizzard { pos: Pt { x: x as i32, y: y as i32 }, dir: Facing::Up }); }
            if c == '>' { blizzards.push(Blizzard { pos: Pt { x: x as i32, y: y as i32 }, dir: Facing::Right }); }
            if c == 'v' { blizzards.push(Blizzard { pos: Pt { x: x as i32, y: y as i32 }, dir: Facing::Down }); }
            if c == '<' { blizzards.push(Blizzard { pos: Pt { x: x as i32, y: y as i32 }, dir: Facing::Left }); }
        }
    }

    let walkdirs = [
        Pt { x: 0, y: 0 },
        Pt { x: -1, y: 0 },
        Pt { x: 1, y: 0 },
        Pt { x: 0, y: -1 },
        Pt { x: 0, y: 1 },
    ];

    let mut todo = vec![(start, 0)];
    for round in 1..1000 {
        // calc blizzards this round
        for blizzard in &mut blizzards {
            match blizzard.dir {
                Facing::Up => {
                    blizzard.pos.y -= 1;
                    if blizzard.pos.y == 0 {
                        blizzard.pos.y = rows.len() as i32 - 2;
                    }
                }
                Facing::Down => {
                    blizzard.pos.y += 1;
                    if blizzard.pos.y == rows.len() as i32 - 1 {
                        blizzard.pos.y = 1;
                    }
                }
                Facing::Left => {
                    blizzard.pos.x -= 1;
                    if blizzard.pos.x == 0 {
                        blizzard.pos.x = map[0].len() as i32 - 2;
                    }
                }
                Facing::Right => {
                    blizzard.pos.x += 1;
                    if blizzard.pos.x == map[0].len() as i32 - 1 {
                        blizzard.pos.x = 1;
                    }
                }
            }
        }

        // let mut strs = rows.iter().map(|r| String::from(*r)).collect::<Vec<_>>();
        // for y in 1..strs.len() - 1 {
        //     for x in 1..strs[0].len() - 1 {
        //         unsafe {
        //             strs[y].as_bytes_mut()[x as usize] = '.' as u8;
        //         }
        //     }
        // }

        let mut blizzard_lookup = HashSet::new();
        for blizzard in &blizzards {
            blizzard_lookup.insert(blizzard.pos);
            // unsafe {
            //     strs[blizzard.pos.y as usize].as_bytes_mut()[blizzard.pos.x as usize] = 'x' as u8;
            // }
        }
        // println!("Round {}, Blizzards: {:?}", round, blizzard_lookup);
        // for str in strs {
        //     println!("{}", str);
        // }

        let mut finished = false;
        let mut next_todo: HashSet<(Pt, usize)> = HashSet::new();
        for (pt, stage) in todo {
            let mut next_stage = stage;
            if stage == 0 && pt == end {
                next_stage = 1;
            }
            if stage == 1 && pt == start {
                next_stage = 2;
            }
            if stage == 2 && pt == end {
                finished = true;
                println!("Finish: Round {}", round - 1);
                break;
            }

            // println!("Checking pt: {:?}", pt);
            for dir in &walkdirs {
                let newpt = pt + *dir;
                if newpt.y < 0 || newpt.x < 0 || newpt.y >= map.len() as i32 || map[newpt.y as usize][newpt.x as usize] || blizzard_lookup.contains(&newpt) {
                    // println!("Newpt culled: {:?}", newpt);
                    continue;
                }
                // println!("Newpt: {:?}", newpt);
                next_todo.insert((newpt, next_stage));
            }
        }
        if finished {
            break;
        }
        todo = next_todo.into_iter().collect::<Vec<_>>();
        // println!("Round {}, Todo: {:?}", round, todo);
    }

    Ok(())
}
