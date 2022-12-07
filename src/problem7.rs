use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Entry {
    name: String,
    dir: bool,
    size: i32,
    subs: HashMap<String, Entry>,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem7.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut dirs = vec!["/"];
    let root = &mut Entry {
        name: String::from("/"),
        dir: true,
        size: -1,
        subs: HashMap::new(),
    };

    let lines = contents.split("\n").collect::<Vec<_>>();
    let mut i=0;
    while i < lines.len() {
        let line = lines[i];
        let cmds = line.split(" ").collect::<Vec<_>>();
        if cmds[0] != "$" {
            println!("bad line: {:?} {:?}", i, line);
            break;
        }
        if cmds[1] == "cd" {
            if cmds[2] == "/" {
                dirs = vec!["/"];
            } else if cmds[2] == ".." {
                dirs.pop();
            } else {
                dirs.push(cmds[2]);
            }
            i += 1;
        } else if cmds[1] == "ls" {
            let mut dir: &mut Entry = root;
            for dn in dirs.clone() {
                if dn == "/" {
                    dir = root;
                } else {
                    dir = dir.subs.get_mut(dn).unwrap();
                }
            }
            i += 1;
            loop {
                if i >= lines.len() || lines[i].starts_with("$") {
                    break;
                }

                let es = lines[i].split_once(" ").unwrap();
                let e: Entry;
                if es.0 == "dir" {
                    e = Entry {
                        name: String::from(es.1),
                        dir: true,
                        size: -1,
                        subs: HashMap::new(),
                    };
                } else {
                    e = Entry {
                        name: String::from(es.1),
                        dir: false,
                        size: es.0.parse::<i32>().unwrap(),
                        subs: HashMap::new(),
                    };
                }
                dir.subs.insert(e.name.clone(), e);
                i += 1;
            }
        } else {
            println!("unknown cmd: {:?}", line);
            break;
        }
    }

    fn calc_dir(e: &mut Entry) {
        if e.size != -1 {
            return;
        }

        let mut size = 0;
        for es in e.subs.values_mut() {
            if es.dir {
                calc_dir(es);
            }
            size += es.size;
        }
        e.size = size;
    }
    calc_dir(root);

    println!("dirs: {:?}", root);

    fn flatten_dirs(e: &Entry) -> Vec<&Entry> {
        let mut r = vec![];
        for es in e.subs.values() {
            if es.dir {
                r.push(es);
                r.append(&mut flatten_dirs(es));
            }
        }
        return r;
    }

    let alldirs = flatten_dirs(root);
    
    let mut pt1dirs = alldirs.clone();
    pt1dirs.retain(|d| d.size < 100000);
    let total = pt1dirs.iter().map(|d| d.size).sum::<i32>();
    println!("Part 1: {:?}", total);

    let free_space = 70000000 - root.size;
    let target = 30000000 - free_space;
    let mut pt2dirs = alldirs.clone();
    pt2dirs.retain(|d| d.size > target);
    pt2dirs.sort_by_key(|d| d.size);
    println!("Part 2: {:?}", pt2dirs[0].size);

    Ok(())
}
