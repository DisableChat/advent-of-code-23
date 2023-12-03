use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "./data/challenge01/input.txt";

#[allow(dead_code)]
pub fn part_one() {
    let mut total_sum: u32 = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(INPUT) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calibration) = line {
                //Part 1 Regex
                let re = Regex::new(r"\d").unwrap();
                let matches: Vec<&str> = re.find_iter(&calibration).map(|s| s.as_str()).collect();

                let fmatch = matches.first().unwrap().to_owned().to_string();
                let rmatch = matches.last().unwrap().to_owned();

                let sum: u32 = (fmatch + rmatch).parse().unwrap();
                total_sum = total_sum + sum;
            }
        }
    }
    println!("total sum: {}", total_sum)
}

#[allow(dead_code)]
pub fn part_two() {
    let mut total_sum: u32 = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(INPUT) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calibration) = line {
                //Part 2 Regex
                let rg = regex::Regex::new(r"(twone|oneight|threeight|fiveight|sevenine|nineight|eighthree|eightwo|one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
                let matches: Vec<&str> = rg.find_iter(&calibration).map(|s| s.as_str()).collect();

                //Front match
                let fmatch = reg_to_num(matches.first().unwrap().to_owned()).to_string();
                //Rear match
                let rmatch = rreg_to_num(matches.last().unwrap().to_owned());

                let sum: u32 = (fmatch.clone() + rmatch).parse().unwrap();
                total_sum = total_sum + sum;
            }
        }
        println!("total sum: {}", total_sum);
    }
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

fn reg_to_num(val: &str) -> &str {
    match val {
        "twone" => "2",
        "oneight" => "1",
        "threeight" => "3",
        "fiveight" => "5",
        "sevenine" => "7",
        "nineight" => "9",
        "eighthree" => "8",
        "eightwo" => "8",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => val,
    }
}

fn rreg_to_num(val: &str) -> &str {
    match val {
        "twone" => "1",
        "oneight" => "8",
        "threeight" => "8",
        "fiveight" => "8",
        "sevenine" => "9",
        "nineight" => "8",
        "eighthree" => "3",
        "eightwo" => "2",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => val,
    }
}
