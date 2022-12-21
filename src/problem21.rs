use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    None,
}

#[derive(Clone)]
struct Monkey {
    name: String,
    value: i64,
    math_op: MathOp,
    dependencies: Vec<String>,
}

impl Monkey {
    fn parse(v: &str) -> Self {
        // wbrf: 17
        // ldhv: vpdn * lpth
        let parts = v.split_once(": ").unwrap();
        let id = parts.0;
        let opparts = parts.1.split(" ").collect::<Vec<_>>();
        if opparts.len() == 1 {
            return Monkey {
                name: String::from(id),
                value: opparts[0].parse::<i64>().unwrap(),
                math_op: MathOp::None,
                dependencies: vec![],
            };
        } else {
            return Monkey {
                name: String::from(id),
                value: -1,
                math_op: match opparts[1] { "+" => MathOp::Add, "-" => MathOp::Sub, "*" => MathOp::Mul, "/" => MathOp::Div, &_ => todo!() },
                dependencies: vec![String::from(opparts[0]), String::from(opparts[2])],
            };
        }
    }

    fn value(&self, monkey_lookup: &HashMap<String, Monkey>) -> i64 {
        if self.dependencies.len() == 0 {
            return self.value;
        }

        let vals = self.dependencies.iter().map(|dn| monkey_lookup.get(dn).unwrap().value(monkey_lookup)).collect::<Vec<_>>();
        return match self.math_op {
            MathOp::Add => vals[0] + vals[1],
            MathOp::Sub => vals[0] - vals[1],
            MathOp::Mul => vals[0] * vals[1],
            MathOp::Div => vals[0] / vals[1],
            MathOp::None => todo!(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem21.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let monkeys_raw = contents.split("\n").map(|r| Monkey::parse(r));

    let mut monkey_lookup = HashMap::new();
    for m in monkeys_raw {
        monkey_lookup.insert(m.name.clone(), m);
    }

    println!("Part 1: {}", monkey_lookup.get("root").unwrap().value(&monkey_lookup));

    Ok(())
}
