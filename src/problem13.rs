use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone)]
struct Elem {
    num: i32,
    arr: Vec<Elem>,
}

impl Elem {
    fn compare_to_right(&self, right: &Elem) -> Ordering {
        if self.num != -1 && right.num != -1 {
            if self.num < right.num {
                return Ordering::Less;
            } else if self.num == right.num {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        }
        let mut llist = self.clone();
        if llist.num != -1 {
            llist.arr = vec![Elem { num: llist.num, arr: vec![] }];
            llist.num = -1;
        }
        let mut rlist = right.clone();
        if rlist.num != -1 {
            rlist.arr = vec![Elem { num: rlist.num, arr: vec![] }];
            rlist.num = -1;
        }
        for i in 0..llist.arr.len() {
            if rlist.arr.len() <= i {
                return Ordering::Greater;
            }
            let ret = llist.arr[i].compare_to_right(&rlist.arr[i]);
            if ret != Ordering::Equal {
                return ret;
            }
        }
        if llist.arr.len() < rlist.arr.len() {
            return Ordering::Less;
        }
        // same len
        return Ordering::Equal;
    }

    fn parse(a: String) -> Self {
        let mut rec = 0;
        let mut maxrec = 0;
        let mut gathered = String::from("");
        let mut root = Elem {
            num: -1,
            arr: vec![],
        };
        for c in a.chars() {
            if c == '[' {
                rec += 1;
                if rec > maxrec {
                    maxrec = rec;
                }
                if rec > 1 {
                    gathered.push(c);
                }
            } else if c == ']' {
                rec -= 1;
                if rec == 0 {
                    if gathered != "" {
                        root.arr.push(Elem::parse(gathered));
                        gathered = String::from("");
                    }
                } else if rec >= 1 {
                    gathered.push(c);
                }
            } else if rec == 1 {
                if c == ',' {
                    root.arr.push(Elem::parse(gathered));
                    gathered = String::from("");
                } else {
                    gathered.push(c);
                }
            } else {
                gathered.push(c);
            }
        }
        if maxrec == 0 {
            root.num = gathered.parse::<i32>().unwrap();
        }
        return root;
    }
}

impl fmt::Debug for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.num != -1 {
            write!(f, "{}", self.num)
        } else {
            let mut out = String::from("[");
            for i in 0..self.arr.len() {
                if i != 0 {
                    out.push_str(",");
                }
                out.push_str(format!("{:?}", self.arr[i]).as_str());
            }
            out.push_str("]");
            write!(f, "{}", out.as_str())
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem13.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let pairs = contents.split("\n\n").map(|p| p.split("\n").map(|r| Elem::parse(String::from(r))).collect::<Vec<_>>()).collect::<Vec<_>>();

    let pt1_rets = pairs.iter().map(|p| p[0].compare_to_right(&p[1])).collect::<Vec<_>>();
    // for p in pairs {
    //     println!("Pt 1 Pair: {:?}", p);
    // }
    // println!("Pt 1 Rets: {:?}", pt1_rets);

    let mut pt1sum = 0;
    for i in 0..pt1_rets.len() {
        if pt1_rets[i] == Ordering::Less {
            pt1sum += i + 1;
        }
    }
    println!("Pt 1: {:?}", pt1sum);

    let mut pt2base = vec![];
    for pair in pairs {
        pt2base.append(&mut pair.clone());
    }
    let dec1 = Elem { num: -1, arr: vec![Elem { num: -1, arr: vec![Elem { num: 2, arr: vec![] }]}]};
    let dec2 = Elem { num: -1, arr: vec![Elem { num: -1, arr: vec![Elem { num: 6, arr: vec![] }]}]};
    pt2base.push(dec1.clone());
    pt2base.push(dec2.clone());

    pt2base.sort_by(|f1, f2| f1.compare_to_right(f2));

    // for p in pt2base {
    //     println!("Pt 2 Elem: {:?}", p);
    // }

    let mut dec1i = 0;
    let mut dec2i = 0;
    for i in 0..pt2base.len() {
        if pt2base[i].compare_to_right(&dec1) == Ordering::Equal {
            dec1i = i + 1;
        } else if pt2base[i].compare_to_right(&dec2) == Ordering::Equal {
            dec2i = i + 1;
        }
    }

    println!("Pt 2: {:?}", dec1i * dec2i);

    Ok(())
}
