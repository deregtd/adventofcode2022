use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem6.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let cb = contents.as_bytes();
    let sizes = [4, 14];
    for nc in sizes {
        for i in (nc-1)..contents.len() {
            let mut c: HashSet<char> = HashSet::new();

            let mut f = false;
            for h in (i-(nc-1))..(i+1) {
                let ch = cb[h] as char;
                if c.contains(&ch) {
                    f = true;
                    break;
                }
                c.insert(ch);
            }
            if !f {
                println!("Size {:?}: {:?}", nc, i+1);
                break;
            }
        }
    }
    
    Ok(())
}
