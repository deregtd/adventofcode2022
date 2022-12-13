use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone)]
enum Elem {
    Num(i32),
    Arr(Vec<Elem>),
}

impl Elem {
    fn compare_to_right(&self, right: &Elem) -> Ordering {
        if let (Elem::Num(ln), Elem::Num(rn)) = (self, right) {
            if ln < rn {
                return Ordering::Less;
            } else if ln == rn {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        }
        let ll = match self {
            Elem::Num(ln) => vec![Elem::Num(*ln)],
            Elem::Arr(la) => la.clone(),
        };
        let rl = match right {
            Elem::Num(rn) => vec![Elem::Num(*rn)],
            Elem::Arr(ra) => ra.clone(),
        };
        for i in 0..ll.len() {
            if rl.len() <= i {
                return Ordering::Greater;
            }
            let ret = ll[i].compare_to_right(&rl[i]);
            if ret != Ordering::Equal {
                return ret;
            }
        }
        if ll.len() < rl.len() {
            return Ordering::Less;
        }
        // same len
        return Ordering::Equal;
    }

    fn parse(a: String) -> Self {
        let mut rec = 0;
        let mut maxrec = 0;
        let mut gathered = String::from("");
        let mut children = vec![];
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
                        children.push(Elem::parse(gathered));
                        gathered = String::from("");
                    }
                } else if rec >= 1 {
                    gathered.push(c);
                }
            } else if rec == 1 {
                if c == ',' {
                    children.push(Elem::parse(gathered));
                    gathered = String::from("");
                } else {
                    gathered.push(c);
                }
            } else {
                gathered.push(c);
            }
        }
        if maxrec == 0 {
            return Elem::Num(gathered.parse::<i32>().unwrap());
        }
        return Elem::Arr(children);
    }
}

impl fmt::Debug for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Elem::Num(n) = self {
            write!(f, "{}", n)
        } else if let Elem::Arr(arr) = self {
            let mut out = String::from("[");
            for i in 0..arr.len() {
                if i != 0 {
                    out.push_str(",");
                }
                out.push_str(format!("{:?}", arr[i]).as_str());
            }
            out.push_str("]");
            write!(f, "{}", out.as_str())
        } else {
            write!(f, "Err")
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
    let dec1 = Elem::Arr(vec![Elem::Arr(vec![Elem::Num(2)])]);
    let dec2 = Elem::Arr(vec![Elem::Arr(vec![Elem::Num(6)])]);
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
