use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem8.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let grid = contents.split("\n").map(|s| s.chars().map(|v| String::from(v).parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    // println!("grid: {:?}", grid);
    
    let mut pt1vis = grid.iter().map(|g| vec![false; g.len()]).collect::<Vec<_>>();

    // Start on the y and move along the two x sides for each row
    for y in 0..grid.len() {
        // left to right
        let mut max_r = -1;
        for x in 0..grid[y].len() {
            if grid[y][x] > max_r {
                pt1vis[y][x] = true;
                max_r = grid[y][x];
            }
        }

        // right to left
        let mut max_l = -1;
        for x in (0..grid[y].len()).rev() {
            if grid[y][x] > max_l {
                pt1vis[y][x] = true;
                max_l = grid[y][x];
            }
        }
    }

    for x in 0..grid[0].len() {
        // top to bottom
        let mut max_d = -1;
        for y in 0..grid.len() {
            if grid[y][x] > max_d {
                pt1vis[y][x] = true;
                max_d = grid[y][x];
            }
        }

        // bottom to top
        let mut max_u = -1;
        for y in (0..grid.len()).rev() {
            if grid[y][x] > max_u {
                pt1vis[y][x] = true;
                max_u = grid[y][x];
            }
        }
    }

    // println!("Part 1 vis: {:?}", pt1vis);

    let mut pt1cnt = 0;
    for row in pt1vis {
        for col in row {
            if col {
                pt1cnt+=1;
            }
        }
    }

    println!("Part 1: {:?}", pt1cnt);

    let mut pt2score = grid.iter().map(|g| vec![-1; g.len()]).collect::<Vec<_>>();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut up = 0;
            for yi in (0..y).rev() {
                up += 1;
                if grid[yi][x] >= grid[y][x] {
                    break;
                }
            }

            let mut down = 0;
            for yi in (y+1)..grid.len() {
                down += 1;
                if grid[yi][x] >= grid[y][x] {
                    break;
                }
            }

            let mut left = 0;
            for xi in (0..x).rev() {
                left += 1;
                if grid[y][xi] >= grid[y][x] {
                    break;
                }
            }

            let mut right = 0;
            for xi in (x+1)..grid[0].len() {
                right += 1;
                if grid[y][xi] >= grid[y][x] {
                    break;
                }
            }

            // println!("{:?},{:?}: {:?} {:?} {:?} {:?}", x, y, up, down, left, right);

            pt2score[y][x] = up * down * left * right;
        }
    }

    // println!("Part 2: {:?}", pt2score);

    let mut pt2max = 0;
    for row in pt2score {
        for col in row {
            if col > pt2max {
                pt2max = col
            }
        }
    }

    println!("Part 2: {:?}", pt2max);

    Ok(())
}
