use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

#[derive(Clone, Debug, Copy, PartialEq)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Clone, Debug)]
enum WalkIns {
    Forward(usize),
    Right,
    Left,
}

#[derive(Clone, Debug, PartialEq)]
enum MapSpace {
    None,
    Wall,
    Air,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem22.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let iparts = contents.split_once("\n\n").unwrap();
    
    // parse walk instructions
    let mut walk_list = vec![];
    let mut buf = String::new();
    for c in iparts.1.chars() {
        if c == 'L' || c == 'R' {
            if !buf.is_empty() {
                walk_list.push(WalkIns::Forward(buf.parse::<usize>().unwrap()));
                buf = String::new();
            }
            if c == 'L' {
                walk_list.push(WalkIns::Left);
            } else if c == 'R' {
                walk_list.push(WalkIns::Right);
            }
        } else {
            buf.push(c);
        }
    }
    if !buf.is_empty() {
        walk_list.push(WalkIns::Forward(buf.parse::<usize>().unwrap()));
    }

    // println!("{:?}", walk_list);

    let board = iparts.0.split("\n").map(|r| {
        return r.chars().map(|c| match c { ' ' => MapSpace::None, '.' => MapSpace::Air, '#' => MapSpace::Wall, _ => todo!() }).collect::<Vec<_>>();
    }).collect::<Vec<_>>();
    // println!("{:?}", board);

    let mut width: usize = 0;
    board.iter().for_each(|r| { if r.len() > width { width = r.len(); } });
    let height = board.len();

    // Precalc wraparounds
    let left_calc = board.iter().map(|r| r.iter().position(|fi| *fi != MapSpace::None).unwrap()).collect::<Vec<_>>();
    let right_calc = (0..height).into_iter().map(|y| {
        for x in (0..width).rev() {
            if x < board[y].len() && board[y][x] != MapSpace::None {
                return x;
            }
        }
        todo!()
    }).collect::<Vec<_>>();
    let top_calc = (0..width).into_iter().map(|x| {
        for y in 0..height {
            if board[y].len() > x && board[y][x] != MapSpace::None {
                return y;
            }
        }
        todo!()
    }).collect::<Vec<_>>();
    let bottom_calc = (0..width).into_iter().map(|x| {
        for y in (0..height).rev() {
            if board[y].len() > x && board[y][x] != MapSpace::None {
                return y;
            }
        }
        todo!()
    }).collect::<Vec<_>>();

    // Part 1
    // let off_top = (0..width).into_iter().map(|x| {
    //     return (Pt { x: x as i32, y: bottom_calc[x] as i32 }, Facing::Up);
    // }).collect::<Vec<_>>();
    // let off_bottom = (0..width).into_iter().map(|x| {
    //     return (Pt { x: x as i32, y: top_calc[x] as i32 }, Facing::Down);
    // }).collect::<Vec<_>>();
    // let off_left = (0..height).into_iter().map(|y| {
    //     return (Pt { x: right_calc[y] as i32, y: y as i32 }, Facing::Left);
    // }).collect::<Vec<_>>();
    // let off_right = (0..height).into_iter().map(|y| {
    //     return (Pt { x: left_calc[y] as i32, y: y as i32 }, Facing::Right);
    // }).collect::<Vec<_>>();
    // Part 2
    let off_top = (0..width).into_iter().map(|x| {
        if x < 50 {
            return (Pt { x: 50, y: 50 + x as i32 }, Facing::Right);
        } else if x < 100 {
            return (Pt { x: 0, y: 150 + ((x as i32) - 50) }, Facing::Right);
        } else {
            return (Pt { x: x as i32 - 100, y: 199 }, Facing::Up);
        }
    }).collect::<Vec<_>>();
    let off_bottom = (0..width).into_iter().map(|x| {
        if x < 50 {
            return (Pt { x: 100 + x as i32, y: 0 }, Facing::Down);
        } else if x < 100 {
            return (Pt { x: 49, y: 150 + ((x as i32) - 50) }, Facing::Left);
        } else {
            return (Pt { x: 99, y: 50 + ((x as i32) - 100) }, Facing::Left);
        }
    }).collect::<Vec<_>>();
    let off_left = (0..height).into_iter().map(|y| {
        if y < 50 {
            return (Pt { x: 0, y: 149 - (y as i32) }, Facing::Right);
        } else if y < 100 {
            return (Pt { x: (y as i32) - 50, y: 100 }, Facing::Down);
        } else if y < 150 {
            return (Pt { x: 50, y: 49 - ((y as i32) - 100) }, Facing::Right);
        } else {
            return (Pt { x: 50 + ((y as i32) - 150), y: 0 }, Facing::Down);
        }
    }).collect::<Vec<_>>();
    let off_right = (0..height).into_iter().map(|y| {
        if y < 50 {
            return (Pt { x: 99, y: 149 - (y as i32) }, Facing::Left);
        } else if y < 100 {
            return (Pt { x: 100 + ((y as i32) - 50), y: 49 }, Facing::Up);
        } else if y < 150 {
            return (Pt { x: 149, y: 49 - ((y as i32) - 100) }, Facing::Left);
        } else {
            return (Pt { x: 50 + ((y as i32) - 150), y: 149 }, Facing::Up);
        }
    }).collect::<Vec<_>>();

    let mut pos = Pt { x: board[0].iter().position(|fi| *fi == MapSpace::Air).unwrap() as i32, y: 0 };
    let mut head = Facing::Right;

    for ins in walk_list {
        if let WalkIns::Forward(walk) = ins {
            for _i in 0..walk {
                let pt_delta = match head {
                    Facing::Right => Pt { x: 1, y: 0 },
                    Facing::Down => Pt { x: 0, y: 1 },
                    Facing::Left => Pt { x: -1, y: 0 },
                    Facing::Up => Pt { x: 0, y: -1 },
                };
                let mut next_pt = pos + pt_delta;
                let mut next_head = head;
                if next_pt.y < 0 || next_pt.x < 0 || next_pt.x >= width as i32 || next_pt.y >= height as i32 || 
                    next_pt.x < left_calc[next_pt.y as usize] as i32 ||
                    next_pt.x > right_calc[next_pt.y as usize] as i32 ||
                    next_pt.y < top_calc[next_pt.x as usize] as i32 ||
                    next_pt.y > bottom_calc[next_pt.x as usize] as i32 ||
                    board[next_pt.y as usize][next_pt.x as usize] == MapSpace::None {
                    let next_info = match head {
                        Facing::Right => off_right[next_pt.y as usize],
                        Facing::Left => off_left[next_pt.y as usize],
                        Facing::Up => off_top[next_pt.x as usize],
                        Facing::Down => off_bottom[next_pt.x as usize],
                    };
                    next_pt = next_info.0;
                    next_head = next_info.1;
                }

                if board[next_pt.y as usize][next_pt.x as usize] == MapSpace::Wall {
                    break;
                }
                pos = next_pt;
                head = next_head;
            }
        } else if let WalkIns::Right = ins {
            head = match head {
                Facing::Right => Facing::Down,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
                Facing::Up => Facing::Right,
            };
        } else if let WalkIns::Left = ins {
            head = match head {
                Facing::Right => Facing::Up,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Down,
                Facing::Up => Facing::Left,
            };
        }
    }

    println!("Answer: {:?} {:?} -> {}", pos, head, (pos.y + 1) * 1000 + (4 * (pos.x + 1)) + (head as i32));

    Ok(())
}
