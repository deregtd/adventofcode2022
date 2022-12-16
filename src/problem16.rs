use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt;
use regex::Regex;

#[derive(Clone)]
struct State {
    minutes: u32,
    position: u16,
    valves_open: HashSet<u16>,
    savings: u32,
    path: Vec<u16>,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mins: {}, Savings: {}, Pos: {}, ValvesOpen: {}, Path: {}", self.minutes, self.savings, u16_to_id(self.position),
            self.valves_open.iter().map(|v| u16_to_id(*v)).collect::<Vec<_>>().join(","),
            self.path.iter().map(|v| u16_to_id(*v)).collect::<Vec<_>>().join(","))
    }
}

#[derive(Debug)]
struct Room {
    id: u16,
    tunnels: Vec<u16>,
    flow_rate: u32,
}

fn id_to_u16(id: &str) -> u16 {
    let b = id.as_bytes();
    return (b[0] as u16) | (b[1] as u16) << 8;
}

fn u16_to_id(u: u16) -> String {
    return [(u & 0xFF) as u8 as char, ((u & 0xFF00) >> 8) as u8 as char].iter().collect();
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem16s.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    let re = Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]* lead[s]* to valve[s]* ([A-Z\s,]+)$").unwrap();
    let rooms = contents.split("\n")
        .map(|p| re.captures(p).unwrap().iter().skip(1).collect::<Vec<_>>())
        .map(|r| Room {
            id: id_to_u16(r[0].unwrap().as_str()),
            flow_rate: r[1].unwrap().as_str().parse::<u32>().unwrap(),
            tunnels: r[2].unwrap().as_str().split(", ").map(|v| id_to_u16(v)).collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();
    let mut room_lookup = HashMap::new();
    for v in &rooms {
        room_lookup.insert(v.id, v);
    }
    
    // precalc path lengths from each room to each other room
    let mut lengths = HashMap::new();
    for room in &rooms {
        let mut min_dists: HashMap<u16, u32> = HashMap::new();
        min_dists.insert(room.id, 0);
        let mut todo: Vec<u16> = vec![room.id];
        while !todo.is_empty() {
            let mut next_todo = vec![];
            for trm in todo {
                let rm2 = room_lookup.get(&trm).unwrap();
                let next_dist = min_dists.get(&rm2.id).unwrap() + 1;
                for tun in &rm2.tunnels {
                    if !min_dists.contains_key(&tun) || *min_dists.get(&tun).unwrap() > next_dist {
                        min_dists.insert(*tun, next_dist);
                        next_todo.push(*tun);
                    }
                }
            }
            todo = next_todo;
        }
        for dist in &min_dists {
            if room.id != *dist.0 {
                lengths.insert((room.id, *dist.0), *dist.1);
            }
        }
    }

    // println!("Shortest routes: {:?}", lengths);

    let initial = State {
        minutes: 0,
        position: id_to_u16("AA"),
        valves_open: HashSet::new(),
        savings: 0,
        path: vec![id_to_u16("AA")],
    };

    // Pt 1
    // let end_minutes: u32 = 30;
    // Pt 2
    let end_minutes: u32 = 26;

    let mut most_pressure = initial.clone();
    let mut todo: Vec<State> = vec![initial.clone()];
    while !todo.is_empty() {
        let mut next_todo = vec![];
        for st in &todo {
            if most_pressure.savings < st.savings {
                most_pressure = st.clone();
            }

            // find new searches
            for oroom in &rooms {
                if oroom.flow_rate == 0 {
                    continue;
                }
                if oroom.id == st.position {
                    continue;
                }
                if st.valves_open.contains(&oroom.id) {
                    continue;
                }

                // expense is walk length + 1 to open
                let expense = *lengths.get(&(st.position, oroom.id)).unwrap() + 1;
                if st.minutes + expense >= end_minutes {
                    // don't even bother with = since that doesn't get us anything
                    continue;
                }

                let mut new_valves = st.valves_open.clone();
                new_valves.insert(oroom.id);

                let mut new_path = st.path.clone();
                new_path.append(&mut vec![oroom.id]);

                next_todo.push(State {
                    minutes: st.minutes + expense,
                    position: oroom.id,
                    savings: st.savings + (end_minutes - (st.minutes + expense)) * oroom.flow_rate,
                    valves_open: new_valves,
                    path: new_path,
                });
            }
        }
        todo = next_todo;
    }

    println!("Part 1: {:?}", most_pressure);

    Ok(())
}
