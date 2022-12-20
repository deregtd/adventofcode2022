use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashSet;

const RESOURCE_ITER: [usize;4] = [0,1,2,3];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Resource {
    // Ore = 0,
    // Clay = 1,
    // Obsidian = 2,
    Geode = 3,
}

#[derive(Debug, Clone)]
struct Blueprint {
    robot_costs: [[usize;4];4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FactoryState {
    have: [usize; 4],
    robots: [usize; 4],
}

// Part 1
// const MAX_MINS: usize = 24;
// const USE_BPS: usize = 30;
// Part 2
const MAX_MINS: usize = 32;
const USE_BPS: usize = 3;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem19.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"^Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
    let mut blueprints = contents.split("\n").map(|r| re.captures(r).unwrap().iter().skip(1).map(|i| i.unwrap().as_str().parse::<usize>().unwrap()).collect::<Vec<_>>())
        .map(|c| Blueprint {
                robot_costs: [
                    [c[0],0,0,0],
                    [c[1],0,0,0],
                    [c[2],c[3],0,0],
                    [c[4],0,c[5],0],
                ],
            }).collect::<Vec<_>>();

    if USE_BPS < blueprints.len() {
        blueprints.drain(USE_BPS..blueprints.len());
    }

    let geodes = blueprints.iter().map(|b| find_max_geodes(b)).collect::<Vec<_>>();

    let mut quality = 0;
    for i in 0..blueprints.len() {
        quality += (i + 1) * geodes[i];
    }

    println!("Part 1: {:?}", quality);

    let mut total = 1;
    for i in 0..blueprints.len() {
        total *= geodes[i];
    }

    println!("Part 2: {:?}", total);

    Ok(())
}

// max geodes in MAX_MINS
fn find_max_geodes(bp: &Blueprint) -> usize {
    println!("BP: {:?}", bp);

    let initial_state = FactoryState {
        have: [0, 0, 0, 0],
        robots: [1, 0, 0, 0],
    };

    let mut most: [usize; 4] = [0,0,0,0];
    let mut todo = vec![initial_state];
    for minute in 0..MAX_MINS {
        let mut next_round = HashSet::new();

        for state in todo {
            // calc the end state for ores made this round up front
            let mut end_have = state.have.clone();
            state.robots.iter().enumerate().for_each(|(i, r)| { end_have[i] += r; });

            end_have.iter().enumerate().for_each(|(i, r)| { if most[i] < *r { most[i] = *r; } });

            // optimization
            if minute != MAX_MINS - 1 {
                let can_afford = RESOURCE_ITER.map(|rt| {
                    for r in 0..4 {
                        if state.have[r] < bp.robot_costs[rt][r] {
                            return false;
                        }
                    }
                    return true;
                });

                let wants = RESOURCE_ITER.map(|rt| {
                    if rt == Resource::Geode as usize {
                        return true;
                    }
                    for r in 0..4 {
                        if state.robots[rt] < bp.robot_costs[r][rt] {
                            return true;
                        }
                    }
                    return false;
                });

                // options to build robots
                for r in 0..4 {
                    if !wants[r] {
                        continue;
                    }
                    if !can_afford[r] {
                        continue;
                    }
                    if can_afford[Resource::Geode as usize] && r != Resource::Geode as usize {
                        continue;
                    }

                    let mut new_state = state.clone();
                    new_state.have = end_have.clone();
                    for rt in 0..4 {
                        new_state.have[rt] -= bp.robot_costs[r][rt];
                    }
                    new_state.robots[r] += 1;
                    next_round.insert(new_state);
                }
            }

            // don't build anything and see what happens
            let mut new_state = state.clone();
            new_state.have = end_have.clone();
            next_round.insert(new_state);
        }

        todo = next_round.into_iter().collect();

        // pull 5000 best answers each round, sorted by geode then obsidian robots
        if todo.len() > 5000 {
            todo.sort_by_key(|t| [t.robots[3], t.robots[2]]);
            todo.drain(0..(todo.len() - 5000));
        }

        println!("Minute {}: {:?}, Most: {:?}", minute + 1, todo.len(), most);
    }

    let mut most_geodes = 0;
    for st in &todo {
        if most_geodes < st.have[Resource::Geode as usize] {
            most_geodes = st.have[Resource::Geode as usize];
        }
    }

    println!("Results: {:?}", most_geodes);

    return most_geodes;
}