use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
struct Pt {
    x: usize,
    y: usize,
}


fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem12.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let grid_raw = contents.split("\n").collect::<Vec<_>>();

    let mut s_pt = Pt { x: 0, y: 0 };
    let mut e_pt = Pt { x: 0, y: 0 };
    let mut grid = vec![];
    for y in 0..grid_raw.len() {
        let mut row = vec![];
        let chars = grid_raw[y].chars().collect::<Vec<_>>();
        for x in 0..chars.len() {
            let c = chars[x];
            if c == 'S' {
                s_pt = Pt { x: x, y: y};
                row.push(0);
            } else if c == 'E' {
                e_pt = Pt { x: x, y: y};
                row.push(25);
            } else {
                row.push((c as u8) - ('a' as u8));
            }
        }
        grid.push(row);
    }
    
    // fill minsteps with -1
    let mut minsteps = grid.iter().map(|r| r.iter().map(|_c| -1).collect::<Vec<_>>()).collect::<Vec<_>>();
    minsteps[e_pt.y][e_pt.x] = 0;
    let mut todo = vec![e_pt];
    while todo.len() > 0 {
        let mut next_todo = vec![];
        for pt in todo {
            let mut next_pts = vec![];
            if pt.x > 0 {
                next_pts.push(Pt { x: pt.x - 1, y: pt.y });
            }
            if pt.x < grid[0].len() - 1 {
                next_pts.push(Pt { x: pt.x + 1, y: pt.y });
            }
            if pt.y > 0 {
                next_pts.push(Pt { x: pt.x, y: pt.y - 1 });
            }
            if pt.y < grid.len() - 1 {
                next_pts.push(Pt { x: pt.x, y: pt.y + 1 });
            }
            let nsteps = minsteps[pt.y][pt.x] + 1;
            for npt in next_pts {
                if grid[pt.y][pt.x] > 0 && grid[npt.y][npt.x] < grid[pt.y][pt.x] - 1 {
                    continue;
                }
                if minsteps[npt.y][npt.x] == -1 || minsteps[npt.y][npt.x] > nsteps {
                    minsteps[npt.y][npt.x] = nsteps;
                    next_todo.push(npt);
                }
            }
        }
        todo = next_todo;
    }

    // for m in minsteps {
    //     println!("{:?}", m);
    // }

    println!("Pt 1: {:?}", minsteps[s_pt.y][s_pt.x]);

    let mut minmin = -1;
    for y in 0..minsteps.len() {
        for x in 0..minsteps[0].len() {
            if grid[y][x] == 0 && minsteps[y][x] != -1 && (minmin == -1 || minsteps[y][x] < minmin) {
                minmin = minsteps[y][x];
            }
        }
    }

    println!("Pt 2: {:?}", minmin);

    Ok(())
}
