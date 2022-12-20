use std::fs::File;
use std::io::prelude::*;

struct WrappingList {
    list: Vec<i64>,
    index_list: Vec<usize>,
}

impl WrappingList {
    fn new(list: Vec<i64>) -> Self {
        let mut index_list = vec![0; list.len()];
        for i in 0..list.len() {
            index_list[i] = i;
        }
        return WrappingList {
            list,
            index_list,
        };
    }

    fn mix(&mut self) {
        // println!("List: {:?}", self.list);
        for pre_index in 0..self.index_list.len() {
            let index = self.index_of_index(pre_index);
            // println!("Index: {:?}", index);
            let entry = self.list.remove(index);
            // println!("Entry: {:?}", entry);
            let index_entry = self.index_list.remove(index);
            let new_index = self.adjust_index(index, entry);
            // println!("New Index: {:?}", new_index);
            self.list.insert(new_index as usize, entry);
            self.index_list.insert(new_index as usize, index_entry);
            // println!("List: {:?}", self.list);
        }
    }

    fn index_of_index(&self, index: usize) -> usize {
        return self.index_list.iter().position(|fi| *fi == index).unwrap();
    }

    fn index_of(&self, elem: i64) -> usize {
        return self.list.iter().position(|fi| *fi == elem).unwrap();
    }

    fn adjust_index(&self, index: usize, adjust: i64) -> usize {
        if adjust == 0 {
            return index;
        }
        if adjust < 0 {
            let mut new_index = index as i64 + adjust;
            if new_index < 0 {
                new_index += (1 - (new_index / self.list.len() as i64)) * (self.list.len() as i64);
            }
            if new_index == 0 {
                new_index = self.list.len() as i64;
            }
            new_index as usize
        } else {
            let mut new_index = (index as i64 + adjust) % self.list.len() as i64;
            if new_index == self.list.len() as i64 {
                new_index = 0;
            }
            new_index as usize
        }
    }

    fn grove_coords(&self) -> i64 {
        let zero_index = self.index_of(0);
        let v_1000 = self.list[(zero_index + 1000) % self.list.len()];
        let v_2000 = self.list[(zero_index + 2000) % self.list.len()];
        let v_3000 = self.list[(zero_index + 3000) % self.list.len()];
        return v_1000 + v_2000 + v_3000;
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem20.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let entries = contents.split("\n").map(|i| i.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut list = WrappingList::new(entries.clone());
    list.mix();
    println!("Part 1: {}", list.grove_coords());

    let mut list2 = WrappingList::new(entries.iter().map(|i| i * 811589153).collect::<Vec<_>>());
    for _i in 0..10 {
        list2.mix();
    }
    println!("Part 2: {}", list2.grove_coords());

    Ok(())
}
