use std::fs::File;
use std::io::prelude::*;


// #[derive(Copy, Clone)]
// #[derive(PartialEq)]
// struct Pt {
//     x: usize,
//     y: usize,
// }

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem10.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let cmds = contents.split("\n").collect::<Vec<_>>();

    {
        let mut cycles = 0;
        let mut x = 1;
        let cyclechecks = vec![20, 60, 100, 140, 180, 220];
        let mut xvals: Vec<i32> = vec![];

        for cmd in &cmds {
            let parts = cmd.split(" ").collect::<Vec<_>>();
            let op = parts[0];
            let pre_x = x;
            if op == "noop" {
                cycles += 1;
            } else if op == "addx" {
                cycles += 2;

                x += parts[1].parse::<i32>().unwrap();
            }

            if xvals.len() < cyclechecks.len() && cycles >= cyclechecks[xvals.len()] {
                xvals.push(pre_x);
            }
        }

        // println!("Counts: {:?}", xvals);
        let mut pt1sum = 0;
        for i in 0..xvals.len() {
            pt1sum += cyclechecks[i] * xvals[i];
        }
        println!("Part 1: {:?}", pt1sum);
    }

    {
        let mut cycles = 0;
        let mut x = 1;
        let mut pc = 0;
        let mut pixels = vec![String::from(""); 6];

        let mut mid_add = false;
        while pc < cmds.len() {
            // println!("{:?} {:?} {:?}", cycles, pc, x);
            let drawing_row = cycles / 40;
            let drawing_px = cycles % 40;
            let npx: &str;
            if x - 1 <= drawing_px && x + 1 >= drawing_px {
                npx = "#";
            } else {
                npx = ".";
            }
            pixels[drawing_row as usize].push_str(npx);

            let parts = cmds[pc].split(" ").collect::<Vec<_>>();
            let op = parts[0];
            if op == "noop" {
                pc+=1;
            } else if op == "addx" {
                if !mid_add {
                    mid_add = true;
                } else {
                    mid_add = false;
                    x += parts[1].parse::<i32>().unwrap();
                    pc+=1;
                }
            }

            cycles+=1;
        }

        for r in pixels {
            println!("{:?}", r);
        }
    }

    Ok(())
}
