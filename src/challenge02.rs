use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../data/challenge02/test_input.txt");
const INPUT: &str = include_str!("../data/challenge02/input.txt");

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

#[allow(dead_code)]
fn read_input(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

#[allow(dead_code)]
pub fn part_one() {
    //let lines = read_lines(REAL_INPUT);
    let lines = read_input(INPUT);
    let total: u32 = lines
        .iter()
        .map(|x| {
            let game_data = x.split(": ").collect_vec();
            let sets = game_data.last().unwrap().split("; ").collect_vec();
            println!("sets : {:?}", sets);

            let valid_sets = sets
                .iter()
                .map(|x| {
                    let set = x.split(", ").collect_vec();

                    println!("set {:?}", set);
                    let tmp = set
                        .iter()
                        .map(|x| {
                            let (num, color) = x.split_whitespace().collect_tuple().unwrap();
                            let num = num.parse::<u32>().unwrap();
                            match color {
                                "red" => {
                                    if num > RED_MAX {
                                        false
                                    } else {
                                        true
                                    }
                                }
                                "green" => {
                                    if num > GREEN_MAX {
                                        false
                                    } else {
                                        true
                                    }
                                }
                                "blue" => {
                                    if num > BLUE_MAX {
                                        false
                                    } else {
                                        true
                                    }
                                }
                                _ => unreachable!("not a valid color"),
                            }
                        })
                        .collect_vec();
                    println!("tmp val {:?} ", tmp);
                    let pass_fail = tmp.iter().all(|&x| x == true);
                    println!(
                        "pass fail {:?} game# : {} ",
                        pass_fail,
                        game_data.first().unwrap()
                    );
                    pass_fail
                })
                .collect_vec();
            let game_num = game_data
                .first()
                .unwrap()
                .split(" ")
                .collect_vec()
                .last()
                .unwrap()
                .to_owned()
                .to_string();
            let possible_game = valid_sets.iter().all(|&x| x == true);
            if possible_game {
                game_num.parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .collect_vec()
        .iter()
        .sum();

    println!("total: {:?}", total);
}

#[allow(dead_code)]
pub fn part_two() {
    let power: u32 = INPUT
        .trim()
        .lines()
        .map(|x| {
            let game_data = x.split(":").collect_vec();

            let (mut red, mut green, mut blue) = (0, 0, 0);

            let sets = game_data[1].split(";").collect_vec();

            sets.iter()
                .map(|set| {
                    let colors = set.split(",").collect_vec();
                    colors
                        .iter()
                        .map(|x| {
                            let (num, color) = x.split_whitespace().collect_tuple().unwrap();
                            let num = num.parse::<u32>().unwrap();

                            match color {
                                "red" => {
                                    if num > red {
                                        red = num
                                    }
                                }
                                "green" => {
                                    if num > green {
                                        green = num
                                    }
                                }
                                "blue" => {
                                    if num > blue {
                                        blue = num
                                    }
                                }
                                _ => unreachable!("not a valid color"),
                            }
                        })
                        .collect_vec();
                })
                .collect_vec();
            red * green * blue
        })
        .sum();
    println!("result {:?}", power);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
