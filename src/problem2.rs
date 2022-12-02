use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem2.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let games = contents
        .split("\n")
        .map(|x|
            x.split(" ")
            .map(|y| y.as_bytes()[0] as i32)
            .collect()
        )
        .collect::<Vec<Vec<i32>>>();

    let mut pt1score = 0;
    let mut pt2score = 0;

    for mut game in games {
        game[0] -= 'A' as i32;
        game[1] -= 'X' as i32;

        // start with your object's score
        let mut pt1round = game[1] + 1;
        if game[0] == game[1] {
            // tie
            pt1round += 3;
        } else if (game[1] == 0 && game[0] == 2) || (!(game[1] == 2 && game[0] == 0) && (game[1] > game[0])){
            // we win
            pt1round += 6;
        } else {
            pt1round += 0;
        }

        pt1score += pt1round;

        // part 2 -- calculate what we need to do
        let mut pt2round: i32 = 0;
        if game[1] == 0 {
            // we need to lose
            let ourcard = if game[0] == 0 { 2 } else { game[0] - 1 };
            pt2round = ourcard + 1 + 0;
        } else if game[1] == 1 {
            // draw
            let ourcard = game[0];
            pt2round = ourcard + 1 + 3;
        } else {
            // we win
            let ourcard = if game[0] == 2 { 0 } else { game[0] + 1 };
            pt2round = ourcard + 1 + 6;
        }

        pt2score += pt2round;
    }

    println!("Part 1: {:?}", pt1score);
    println!("Part 2: {:?}", pt2score);
    Ok(())
}
