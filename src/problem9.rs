use std::fs::File;
use std::io::prelude::*;


#[derive(Copy, Clone)]
#[derive(PartialEq)]
struct Pt {
    x: usize,
    y: usize,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem9.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let cmds = contents.split("\n").map(|r| r.split_once(" ").unwrap()).collect::<Vec<_>>();

    // find the extent
    let mut x: i32=0;
    let mut y: i32=0;
    let mut x_min = x;
    let mut x_max = x;
    let mut y_min = y;
    let mut y_max = y;
    for cmd in cmds.clone() {
        let dir = cmd.0;
        let num = cmd.1.parse::<usize>().unwrap();
        for _step in 0..num {
            if dir == "U" {
                y -= 1;
                if y < y_min {
                    y_min = y;
                }
            } else if dir == "D" {
                y += 1;
                if y > y_max {
                    y_max = y;
                }
            } else if dir == "L" {
                x -= 1;
                if x < x_min {
                    x_min = x;
                }
            } else if dir == "R" {
                x += 1;
                if x > x_max {
                    x_max = x;
                }
            } else {
                println!("Error: {:?}", dir);
                break;
            }
        }
    }

    // set up visited grid and initial spot to stay within the visited grid
    let width: usize = (x_max-x_min+1).try_into().unwrap();
    let height: usize = (y_max-y_min+1).try_into().unwrap();
    let num_tails = 9;
    let mut visited = vec![false; num_tails].iter().map(|_g| vec![false; height].iter().map(|_r| vec![false; width]).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut head = Pt {
        x: (-1 * x_min).try_into().unwrap(),
        y: (-1 * y_min).try_into().unwrap(),
    };
    let mut tails = vec![false; num_tails].iter().map(|_s| head).collect::<Vec<_>>();

    fn catch_up(head: Pt, tail: Pt) -> Pt {
        let xd = head.x as i32 - tail.x as i32;
        let yd = head.y as i32 - tail.y as i32;
        if xd.abs() <= 1 && yd.abs() <= 1 {
            return tail;
        }
        return Pt {
            x: if xd > 1 { head.x - 1 } else if xd < -1 { head.x + 1 } else { head.x },
            y: if yd > 1 { head.y - 1 } else if yd < -1 { head.y + 1 } else { head.y },
        };
    }

    for cmd in cmds {
        let dir = cmd.0;
        let num = cmd.1.parse::<usize>().unwrap();
        for _step in 0..num {
            if dir == "U" {
                head.y -= 1;
            } else if dir == "D" {
                head.y += 1;
            } else if dir == "L" {
                head.x -= 1;
            } else if dir == "R" {
                head.x += 1;
            }
            for ti in 0..num_tails {
                let prev = if ti == 0 { head } else { tails[ti - 1] };
                tails[ti] = catch_up(prev, tails[ti]);
                visited[ti][tails[ti].y][tails[ti].x] = true;
            }
        }
    }

    // let mut grid = visited[0].iter().map(|r| r.iter().map(|_c| ".").collect::<Vec<_>>().join("")).collect::<Vec<_>>();
    // for y in 0..height {
    //     for x in 0..width {
    //         let p = Pt { x: x, y: y };
    //         unsafe {
    //             if p == head {
    //                 grid[y].as_bytes_mut()[x] = 'H' as u8;
    //             }
    //             for ti in 0..num_tails {
    //                 if p == tails[ti] {
    //                     grid[y].as_bytes_mut()[x] = ('1' as u8 + ti as u8) as u8;
    //                 }
    //             }
    //         }
    //     }
    // }

    // for r in grid {
    //     println!("{:?}", r);
    // }

    // let dout = visited[num_tails-1].iter().map(|r| r.iter().map(|c| if *c { "#" } else { "." }).collect::<Vec<_>>().join("")).collect::<Vec<_>>();
    // for r in dout {
    //     println!("{:?}", r);
    // }

    let counts = visited.iter().map(|v| {
        let mut vcnt = 0;
        for row in v {
            for c in row {
                if *c {
                    vcnt+=1;
                }
            }
        }
        return vcnt;
    }).collect::<Vec<_>>();
    
    println!("Counts: {:?}", counts);

    Ok(())
}
