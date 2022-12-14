use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Pt {
    x: usize,
    y: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Val {
    Air,
    Sand,
    Rock,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem14.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let linesets = contents.split("\n").map(|p| p.split(" -> ")
        .map(|pt| pt.split_once(",").map(|(x,y)| Pt { x: x.parse::<usize>().unwrap(), y: y.parse::<usize>().unwrap() }).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut min = Pt { x: 9999, y: 9999 };
    let mut max = Pt { x: 0, y: 0 };

    for lineset in &linesets {
        for pt in lineset {
            if pt.x < min.x {
                min.x = pt.x;
            }
            if pt.x > max.x {
                max.x = pt.x;
            }
            if pt.y < min.y {
                min.y = pt.y;
            }
            if pt.y > max.y {
                max.y = pt.y;
            }
        }
    }
    // force min to 0 to get the 500,0 pt in
    min.y = 0;

    // add extra
    min.x = 0;
    max.x += 500;

    // pt 2
    max.y += 2;

    // println!("{:?} {:?}", min, max);

    let mut grid = vec![0; max.y - min.y + 1].iter().map(|_r| vec![Val::Air; max.x - min.x + 1]).collect::<Vec<_>>();

    for lineset in linesets {
        for i in 1..lineset.len() {
            if lineset[i-1].x == lineset[i].x {
                let mut ys = [lineset[i-1].y, lineset[i].y];
                ys.sort();
                for y in ys[0]..=ys[1] {
                    grid[y - min.y][lineset[i].x - min.x] = Val::Rock;
                }
            } else if lineset[i-1].y == lineset[i].y {
                let mut xs = [lineset[i-1].x, lineset[i].x];
                xs.sort();
                for x in xs[0]..=xs[1] {
                    grid[lineset[i].y - min.y][x - min.x] = Val::Rock;
                }
            } else {
                println!("wtf");
                return Ok(());
            }
        }
    }

    // pt 2
    for x in min.x..=max.x {
        grid[max.y - min.y][x-min.x] = Val::Rock;
    }

    // for row in &grid {
    //     println!("{:?}", row);
    // }

    let mut off_bottom = false;
    loop {
        if grid[0 - min.y][500 - min.x] == Val::Sand {
            // have sand in start spot
            break;
        }
        if off_bottom {
            break;
        }
    
        // seed
        let mut sandpt = Pt { x: 500, y: 0 };

        loop {
            // can it go down
            // println!("{:?}", sandpt);
            if sandpt.y == max.y {
                // falls off bottom
                off_bottom = true;
                break;
            }
            if grid[sandpt.y-min.y+1][sandpt.x-min.x] == Val::Air {
                sandpt.y += 1;
            } else if grid[sandpt.y-min.y+1][sandpt.x-min.x-1] == Val::Air {
                sandpt.x -= 1;
                sandpt.y += 1;
            } else if grid[sandpt.y-min.y+1][sandpt.x-min.x+1] == Val::Air {
                sandpt.x += 1;
                sandpt.y += 1;
            } else {
                grid[sandpt.y-min.y][sandpt.x-min.x] = Val::Sand;
                break;
            }
        }
    }

    let mut cnt = 0;
    for row in grid {
        cnt += row.iter().filter(|c| **c == Val::Sand).count();
    }

    println!("Count: {:?}", cnt);

    Ok(())
}
