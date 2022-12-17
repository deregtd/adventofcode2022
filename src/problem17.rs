use std::fs::File;
use std::io::prelude::*;

// NOTE: NONE OF THIS MAKES SENSE WITHOUT THIS EXPLANATION SINCE THIS CODE DOES NOT ACTUALLY SOLVE PART 2

// Okay, so this part two was kinda out there.  The base calculation was reasonably computationally impossible,
// so I assumed that there was some repetitive pattern, given that the blocks repeated with a vertical and
// then horizontal line.  I debug spewed the sample output and saw what looked like a repeating pattern, so I
// set about finding what the repetition was, to see if we could safely extrapolate out to the giant number
// of rocks they wanted to predict at.

// I started out spewing a simple debug log every time the rock_index went to 0 (every time it completed one
// set of all 5 rocks) outputting the jet_index and the rock pile height.  On the sample, a pattern quickly
// emerged -- after the first 10 rocks, the rocks settled into a pattern of resetting every 35 rocks, and the
// rock pile's height and the jet index were consistent and predictable immediately.  I repeated the
// experiment on the sample data set and found a similar pattern, just far more spread out since the jet
// pattern is so much longer.  I took the rock pile heights starting on the second full cycle of jets and
// put them into this array:

const OFFSETS: [i32; 348] = [2668,2674,2680,2687,2691,2699,2707,2714,2721,2728,2736,2745,2750,2756,2763,2772,2783,2793,2802,2809,2817,2823,2836,2844,2855,2861,2868,2876,2885,2891,2897,2905,2918,2928,2934,2942,2948,2956,2959,2967,2974,2984,2991,2998,3002,3010,3017,3024,3032,3035,3046,3054,3061,3073,3081,3089,3095,3105,3110,3118,3124,3128,3136,3139,3147,3154,3160,3166,3172,3182,3192,3200,3208,3219,3226,3234,3243,3249,3254,3263,3270,3278,3283,3290,3299,3309,3318,3328,3338,3348,3354,3359,3371,3378,3389,3394,3398,3408,3420,3427,3430,3439,3450,3458,3466,3475,3482,3490,3492,3500,3507,3513,3520,3525,3533,3539,3546,3550,3556,3561,3571,3579,3582,3589,3595,3603,3609,3619,3626,3634,3641,3649,3655,3667,3676,3681,3690,3695,3704,3713,3719,3727,3731,3737,3745,3751,3758,3768,3772,3779,3791,3802,3808,3816,3825,3831,3836,3845,3853,3860,3866,3875,3882,3888,3894,3905,3914,3925,3931,3939,3947,3953,3960,3966,3974,3982,3991,4000,4009,4015,4025,4032,4040,4047,4053,4062,4069,4077,4083,4094,4105,4109,4116,4125,4132,4142,4152,4160,4168,4175,4183,4194,4206,4214,4220,4229,4237,4246,4252,4259,4268,4272,4279,4288,4296,4304,4310,4321,4327,4333,4342,4353,4361,4367,4372,4376,4387,4393,4404,4411,4419,4429,4440,4445,4452,4458,4463,4470,4480,4489,4499,4510,4517,4527,4530,4539,4550,4558,4566,4574,4583,4596,4603,4612,4618,4625,4631,4643,4650,4655,4661,4670,4679,4684,4693,4701,4707,4716,4722,4731,4740,4749,4756,4764,4769,4780,4790,4798,4806,4816,4825,4833,4840,4845,4854,4860,4866,4875,4879,4887,4890,4897,4903,4912,4919,4926,4936,4944,4953,4958,4964,4972,4981,4988,4999,5009,5016,5026,5037,5047,5055,5059,5065,5073,5078,5088,5098,5107,5115,5125,5138,5146,5153,5159,5164,5175,5181,5189,5196,5203,5216,5225,5234,5241,5247,5253,5259,5272,5278,5285,5290,5300,5307,5313,5320,5329,5335,5339];

// and then calculated out the math (see below) and got the correct predicted rock height.

// Part 1
// const ROCK_LIMIT: u64 = 2022;
// Part 2 exploration was way out wide
const ROCK_LIMIT: u64 = 10000;

fn can_move(xdelta: i32, ydelta: i32, rock_x: usize, rock_y: usize, rock: &Vec<Vec<bool>>, rows: &Vec<[bool;7]>) -> bool {
    if rock_x as i32 + rock[0].len() as i32 + xdelta > 7 {
        return false;
    }
    if rock_x as i32 + xdelta < 0 {
        return false;
    }
    if rock_y as i32 - rock.len() as i32 + 1 + ydelta < 0 {
        return false;
    }
    for yr in 0..rock.len() {
        let y = (rock_y - yr) as i32 + ydelta;
        for xr in 0..rock[0].len() {
            if !rock[yr][xr] {
                continue;
            }
            let x = (rock_x + xr) as i32 + xdelta;
            if rows.len() > (y as usize) && rows[y as usize][x as usize] {
                return false;
            }
        }
    }
    return true;
}

// fn print_rows(rows: &Vec<[bool;7]>) {
//     for i in (0..rows.len()).rev() {
//         println!("{}", rows[i].map(|v| if v { '#' } else { '.' }).iter().collect::<String>());
//     }
// }

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem17.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let rocks: Vec<Vec<Vec<bool>>> = vec![
        vec![
            vec![true, true, true, true],
        ],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true, true, true],
        ],
        vec![
            vec![true],
            vec![true],
            vec![true],
            vec![true],
        ],
        vec![
            vec![true, true],
            vec![true, true],
        ],
    ];

    let winds = contents.chars().map(|b| b == '>').collect::<Vec<_>>();
    
    let mut rows = vec![[false; 7]; 0];

    let mut rock_index = 0;
    let mut jet_index = 0;
    
    for rock_num in 0..ROCK_LIMIT {
        let mut rock_y = rows.len() + 3 + rocks[rock_index].len() - 1;
        let mut rock_x: usize = 2;
        loop {
            if winds[jet_index] {
                // go right
                if can_move(1, 0, rock_x, rock_y, &rocks[rock_index], &rows) {
                    // println!("right");
                    rock_x += 1;
                } else {
                    // println!("right blocked");
                }
            } else {
                // go left
                if can_move(-1, 0, rock_x, rock_y, &rocks[rock_index], &rows) {
                    // println!("left");
                    rock_x -= 1;
                } else {
                    // println!("left blocked");
                }
            }

            jet_index += 1;
            if jet_index == winds.len() {
                jet_index = 0;
            }

            if can_move(0, -1, rock_x, rock_y, &rocks[rock_index], &rows) {
                rock_y -= 1;
                // println!("down {:?}", rock_y);
            } else {
                // manifest the rock
                // println!("manifest {:?} {:?}", rock_y, rows.len());
                while rock_y >= rows.len() {
                    rows.push([false; 7]);
                }
                for yr in 0..rocks[rock_index].len() {
                    let y = rock_y - yr;
                    for xr in 0..rocks[rock_index][0].len() {
                        if !rocks[rock_index][yr][xr] {
                            continue;
                        }
                        let x = xr + rock_x;
                        rows[y][x] = true;
                    }
                }
                break;
            }
        }
        rock_index = (rock_index + 1) % rocks.len();

        if rock_index == 0 {
            println!("Rock {}, Height {}, JI {}", rock_num, rows.len(), jet_index);

            if rock_num >= 1729 {
                let rep_index = (rock_num - 1729) / 5;
                let row_mult = rep_index / 348;
                let row_step = rep_index % 348;
                
                let pred_base = row_mult as i32 * 2681 + OFFSETS[row_step as usize];
                println!("RepI {}, Pred {}, Act {}", rep_index, pred_base, rows.len());
                // rock_num = 1000000000000
                // rep_index = 199999999654
                // row_mult = 574712642
                // row_step = 238
                // height = 574712642 * 2681 + offsets[238] (=4480)
            }
            // Sample set, where I worked on the algorithm
            // if rock_num > 10 {
            //     let rep_index = (rock_num + 1 - 15) / 5;
            //     let row_mult = rep_index / 7;
            //     let row_step = rep_index % 7;
                
            //     let pred_base = 25 + row_mult * 53 +
            //         match row_step {
            //             0 => 0,
            //             1 => 11,
            //             2 => 18,
            //             3 => 26,
            //             4 => 35,
            //             5 => 41,
            //             6 => 47,
            //             _ => 0,
            //         };
            //     // println!("RepI {}, Pred {}, Act {}", rep_index, pred_base, rows.len());
            //     // rock_num = 1000000000000
            //     // rep_index = 199999999997
            //     // row_mult = 28571428571
            //     // row_step = 1
            //     // 25 + (28571428571 * 53) + 11
            // }
        }
    }

    // print_rows(&rows);

    println!("Part 1: {:?}", rows.len());

    Ok(())
}
