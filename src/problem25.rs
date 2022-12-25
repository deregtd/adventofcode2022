use std::fs::File;
use std::io::prelude::*;

fn snafu_to_num(s: &str) -> i64 {
    let mut total = 0;
    let chars = s.chars().collect::<Vec<_>>();
    let mut mult = 1;
    for i in (0..chars.len()).rev() {
        let char = match chars[i] {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => todo!(),
        };

        total += char * mult;
        mult *= 5;
    }
    return total;
}

fn num_to_snafu(n: i64) -> String {
    let max_digit: usize = (n as f64).log(5.0).ceil() as usize + 2;
    let mut vals = vec![0; max_digit as usize];

    let mut total = n;
    let mut str = String::from("");

    for i in 0..max_digit {
        let count = total % 5;
        vals[i as usize] = count;
        total /= 5;
    }

    for i in 0..max_digit {
        let mut found = false;
        for i2 in i..max_digit {
            if vals[i2] != 0 {
                found = true;
                break;
            }
        }
        if !found {
            break;
        }

        if vals[i] > 2 {
            vals[i + 1] += 1;
            vals[i] -= 5;
        }

        let char = match vals[i] {
            2 => "2",
            1 => "1",
            0 => "0",
            -1 => "-",
            -2 => "=",
            _ => todo!(),
        };

        str = String::from(char) + str.as_str();
    }

    // println!("{} {:?}", n, vals);

    return str;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem25.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let rows = contents.split("\n").collect::<Vec<_>>();

    let numbers = rows.iter().map(|r| snafu_to_num(r)).reduce(|v, r| v + r).unwrap();

    println!("Nums: {:?}", numbers);

    let snafu = num_to_snafu(numbers);
    println!("SNAFU: {}", snafu);
    
    println!("SNAFU back: {}", snafu_to_num(snafu.as_str()));

    Ok(())
}
