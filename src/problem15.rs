use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Pt {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct RangeSet {
    ranges: Vec<(i32, i32)>,
}

impl RangeSet {
    fn new() -> Self {
        return RangeSet { ranges: vec![] }
    }

    fn add(&mut self, range: (i32, i32)) {
        // println!("adding: {:?}", range);
        let mut found = false;
        for i in 0..self.ranges.len() {
            // blindly sort it into the range
            if range.0 < self.ranges[i].0 {
                self.ranges.insert(i, range);
                found = true;
                break;
            }
        }
        if !found {
            self.ranges.push(range);
        }

        // println!("before merge: {:?}", self.ranges);

        let mut i = 0;
        while i < self.ranges.len() - 1 {
            // check if each element needs to be merged into the one after
            if self.ranges[i].1 >= (self.ranges[i+1].0 - 1) {
                self.ranges[i].1 = std::cmp::max(self.ranges[i].1, self.ranges[i+1].1);
                self.ranges.remove(i+1);
                continue;
            }
            i += 1;
        }

        // println!("after merge: {:?}", self.ranges);
    }

    fn clip(&mut self, min: i32, max: i32) {
        self.ranges.retain(|r| r.1 >= min && r.0 <= max);
        if self.ranges[0].0 < min {
            self.ranges[0].0 = min;
        }
        if self.ranges.last().unwrap().1 > max {
            self.ranges.last_mut().unwrap().1 = max;
        }
    }
}

fn man_dist(pt1: Pt, pt2: Pt) -> i32 {
    return (pt2.x - pt1.x).abs() + (pt2.y - pt1.y).abs();
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem15.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let re = Regex::new(r"^Sensor at x=(\-*\d+), y=(\-*\d+): closest beacon is at x=(\-*\d+), y=(\-*\d+)$").unwrap();
    let sensors = contents.split("\n")
        .map(|p| re.captures(p).unwrap().iter().skip(1).map(|v| v.unwrap().as_str().parse::<i32>().unwrap()).collect::<Vec<_>>())
        .map(|v| (Pt { x: v[0], y: v[1] }, Pt {x: v[2], y: v[3] }))
        .collect::<Vec<_>>();

    // Part 1 Sample
    // let y_check = 10;
    // Part 1 real
    let y_check = 2000000;

    let mut rs = RangeSet::new();
    for sensor in &sensors {
        let range = man_dist(sensor.0, sensor.1);
        let row_range = range - (sensor.0.y - y_check).abs();
        if row_range > 0 {
            rs.add((sensor.0.x - row_range, sensor.0.x + row_range));
        }
    }

    // println!("RS: {:?}", rs);

    let mut cnt: usize = 0;
    for range in rs.ranges {
        cnt += (range.1 - range.0) as usize + 1;
    }

    let mut beacons_on_row = sensors.iter().map(|s| s.1).filter(|b| b.y == y_check).collect::<Vec<_>>();
    beacons_on_row.dedup();

    println!("Part 1: {:?}", cnt - beacons_on_row.len());

    // sample
    // let outer_bound = 20;
    // real
    let outer_bound = 4000000;

    for y in 0..=outer_bound {
        let mut rs = RangeSet::new();
        for sensor in &sensors {
            let range = man_dist(sensor.0, sensor.1);
            let row_range = range - (sensor.0.y - y).abs();
            if row_range > 0 {
                rs.add((sensor.0.x - row_range, sensor.0.x + row_range));
            }
        }

        rs.clip(0, outer_bound);

        if rs.ranges.len() == 2 {
            let x = rs.ranges[0].1 + 1;
            println!("Pt 2: ({:?}, {:?}) -> {:?}", x, y, (x as usize)*4000000 + y as usize);
        }
    }

    Ok(())
}
