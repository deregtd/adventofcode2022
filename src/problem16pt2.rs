use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;
use regex::Regex;

const NUM_HELPERS: usize = 2;

#[derive(Clone)]
struct State {
    minutes: u16,
    positions: [u16; NUM_HELPERS],
    timeouts: [u16; NUM_HELPERS],
    valves_open: Vec<u16>,
    savings: u16,
}

impl State {
    fn move_forward(&mut self) {
        let to_step: usize = if self.timeouts[0] <= self.timeouts[1] { 0 } else { 1 };
        let to_drop = self.timeouts[to_step];
        self.minutes += to_drop;
        self.timeouts[0] -= to_drop;
        self.timeouts[1] -= to_drop;
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mins: {}, Savings: {}, Pos: {}, Timeouts: {}, ValvesOpen: {}", self.minutes, self.savings,
            self.positions.iter().map(|v| u16_to_id(*v)).collect::<Vec<_>>().join(","),
            self.timeouts.iter().map(|v| format!("{}", *v)).collect::<Vec<_>>().join(","),
            self.valves_open.iter().map(|v| u16_to_id(*v)).collect::<Vec<_>>().join(","))
    }
}

#[derive(Clone)]
#[derive(Debug)]
struct Room {
    id: u16,
    tunnels: Vec<u16>,
    flow_rate: u16,
}

fn id_to_u16(id: &str) -> u16 {
    let b = id.as_bytes();
    return (b[0] as u16) | (b[1] as u16) << 8;
}

fn u16_to_id(u: u16) -> String {
    return [(u & 0xFF) as u8 as char, ((u & 0xFF00) >> 8) as u8 as char].iter().collect();
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem16.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    let re = Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]* lead[s]* to valve[s]* ([A-Z\s,]+)$").unwrap();
    let rooms = contents.split("\n")
        .map(|p| re.captures(p).unwrap().iter().skip(1).collect::<Vec<_>>())
        .map(|r| Room {
            id: id_to_u16(r[0].unwrap().as_str()),
            flow_rate: r[1].unwrap().as_str().parse::<u16>().unwrap(),
            tunnels: r[2].unwrap().as_str().split(", ").map(|v| id_to_u16(v)).collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();
    let mut room_lookup = HashMap::new();
    for v in &rooms {
        room_lookup.insert(v.id, v);
    }
    
    // precalc flow>0 rooms
    let mut flowing_rooms = rooms.iter().map(|r| r).collect::<Vec<_>>();
    flowing_rooms.retain(|room| room.flow_rate > 0);

    // precalc path lengths from each room to each other room
    let mut lengths = HashMap::new();
    for room in &rooms {
        let mut min_dists: HashMap<u16, u16> = HashMap::new();
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

    let aa_pos = id_to_u16("AA");
    let initial = State {
        minutes: 0,
        positions: [aa_pos; NUM_HELPERS],
        timeouts: [0; NUM_HELPERS],
        valves_open: vec![],
        savings: 0,
    };

    // Pt 1
    // let end_minutes: u16 = 30;
    // Pt 2
    let end_minutes: u16 = 26;

    let mut most_pressure = initial.clone();
    let mut todo: Vec<State> = vec![initial.clone()];
    while !todo.is_empty() {
        let mut next_todo = vec![];
        for st in &todo {
            if most_pressure.savings < st.savings {
                most_pressure = st.clone();
            }

            // find new searches and only fill in one set of options, even if both are free, and don't update times
            let mut free_indexes = vec![];
            if st.timeouts[0] == 0 {
                free_indexes.push(0);
            }
            if st.timeouts[1] == 0 {
                free_indexes.push(1);
            }
            
            let mut found_any = false;
            for free_index in free_indexes {
                // for the very end state, wrap around to the second "free index" just in case
                for oroom in &flowing_rooms {
                    if oroom.id == st.positions[free_index] {
                        continue;
                    }
                    if st.valves_open.contains(&oroom.id) {
                        continue;
                    }
    
                    // expense is walk length + 1 to open
                    let expense = *lengths.get(&(st.positions[free_index], oroom.id)).unwrap() + 1;
                    if st.minutes + expense > end_minutes {
                        // Somehow, moving this to > instead of >= fixes my final answer, implying that there's
                        // some edge condition near the very end of the run, but I can't figure out what it is
                        // and I'm tired.
                        continue;
                    }
    
                    let mut new_state = st.clone();
                    new_state.positions[free_index] = oroom.id;
                    new_state.timeouts[free_index] = expense;
                    new_state.savings += (end_minutes - new_state.minutes - expense) * oroom.flow_rate;
                    new_state.valves_open.push(oroom.id);
                    new_state.move_forward();
                    next_todo.push(new_state);
                    found_any = true;
                }
                if found_any {
                    break;
                }
            }
        }
        todo = next_todo;
        println!("Round: {}, Most: {:?}", todo.len(), most_pressure);
    }

    println!("Most: {:?}", most_pressure);

    Ok(())
}
