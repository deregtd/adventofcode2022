use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
struct Pt {
    x: usize,
    y: usize,
    z: usize,
}

struct Grid {
    data: Vec<bool>,
    maxes: Vec<usize>,
}

impl Grid {
    fn new(maxes: Vec<usize>) -> Self {
        let g = Grid {
            data: vec![false; maxes[0] * maxes[1] * maxes[2]],
            maxes: maxes,
        };
        return g;
    }
    fn insert(&mut self, pt: &Pt) {
        self.data[pt.z * self.maxes[0] * self.maxes[1] + pt.y * self.maxes[0] + pt.x] = true;
    }
    // fn remove(&mut self, pt: &Pt) {
    //     self.data[pt.z * self.maxes[0] * self.maxes[1] + pt.y * self.maxes[0] + pt.x] = false;
    // }
    fn has(&self, pt: &Pt) -> bool {
        return self.data[pt.z * self.maxes[0] * self.maxes[1] + pt.y * self.maxes[0] + pt.x];
    }

    fn find_outer_air(&self) -> Pt {
        // be lazy for now and just check the plane at z=0
        for y in 0..=self.maxes[1] {
            for x in 0..=self.maxes[0] {
                let pt = Pt { x: x, y: y, z: 0};
                if !self.has(&pt) {
                    return pt;
                }
            }
        }
        println!("Crap");
        return Pt { x: 0, y: 0, z: 0};
    }

    fn surfaces(&self) -> usize {
        let mut faces = 0 as usize;

        for z in 0..=self.maxes[2] {
            for y in 0..=self.maxes[1] {
                for x in 0..=self.maxes[0] {
                    let at = x < self.maxes[0] && y < self.maxes[1] && z < self.maxes[2] && self.has(&Pt { x: x, y: y, z: z});
                    let prevx = x > 0 && y < self.maxes[1] && z < self.maxes[2] && self.has(&Pt { x: x - 1, y: y, z: z});
                    let prevy = y > 0 && x < self.maxes[0] && z < self.maxes[2] && self.has(&Pt { x: x, y: y - 1, z: z});
                    let prevz = z > 0 && x < self.maxes[0] && y < self.maxes[1] && self.has(&Pt { x: x, y: y, z: z - 1});
                    
                    if prevx ^ at {
                        faces += 1;
                    }
                    if prevy ^ at {
                        faces += 1;
                    }
                    if prevz ^ at {
                        faces += 1;
                    }
                }
            }
        }

        return faces;
    }

    fn calc_outer_air(&self) -> Self {
        let mut is_air = Grid::new(self.maxes.clone());
        let air_pt = self.find_outer_air();

        is_air.insert(&air_pt);
        let mut walks = vec![air_pt];
        while !walks.is_empty() {
            let mut next_walks = vec![];

            for pt in walks {
                let mut check_pts = vec![];
                if pt.x > 0 {
                    check_pts.push(Pt { x: pt.x - 1, y: pt.y, z: pt.z });
                }
                if pt.x < self.maxes[0] - 1 {
                    check_pts.push(Pt { x: pt.x + 1, y: pt.y, z: pt.z });
                }
                if pt.y > 0 {
                    check_pts.push(Pt { x: pt.x, y: pt.y - 1, z: pt.z });
                }
                if pt.y < self.maxes[1] - 1 {
                    check_pts.push(Pt { x: pt.x, y: pt.y + 1, z: pt.z });
                }
                if pt.z > 0 {
                    check_pts.push(Pt { x: pt.x, y: pt.y, z: pt.z - 1 });
                }
                if pt.z < self.maxes[2] - 1 {
                    check_pts.push(Pt { x: pt.x, y: pt.y, z: pt.z + 1 });
                }
                for pt in &check_pts {
                    if !self.has(pt) && !is_air.has(pt) {
                        is_air.insert(&pt);
                        next_walks.push(*pt);
                    }
                }
            }

            walks = next_walks;
        }

        return is_air;
    }

    fn remove_inners(&mut self) {
        let air = self.calc_outer_air();

        for x in 0..self.maxes[0] {
            for y in 0..self.maxes[1] {
                for z in 0..self.maxes[2] {
                    let pt = Pt { x: x, y: y, z: z};
                    let air_has = air.has(&pt);
                    let we_has = self.has(&pt);
                    if !we_has && !air_has {
                        self.insert(&pt);
                    }
                }
            }
        }
    }

}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem18.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let cubes = contents.split("\n")
        .map(|r| r.split(",").map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>())
        .map(|v| Pt { x: v[0], y: v[1], z: v[2] })
        .collect::<Vec<_>>();

    let mut maxes = [0 as usize; 3];
    for cube in &cubes {
        if cube.x >= maxes[0] {
            maxes[0] = cube.x + 1;
        }
        if cube.y >= maxes[1] {
            maxes[1] = cube.y + 1;
        }
        if cube.z >= maxes[2] {
            maxes[2] = cube.z + 1;
        }
    }

    println!("Maxes: {:?}", maxes);

    let mut grid = Grid::new(maxes.to_vec());

    for cube in &cubes {
        grid.insert(cube);
    }

    println!("Surface area: {:?}", grid.surfaces());

    grid.remove_inners();

    println!("Outer Surface area: {:?}", grid.surfaces());

    Ok(())
}
