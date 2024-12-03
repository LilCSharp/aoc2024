use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("assets/challenge.txt")?;
    let reader = BufReader::new(file);
    let mut num: i64 = 0;
    let re = Regex::new(r"(mul\(([\d]{1,3}),([\d]{1,3})\))|don\'t|do").unwrap();
    let mut allowed = true;

    for line in reader.lines() {
        match line {
            Ok(s) => {
                for cap in re.captures_iter(&s) {
                    if let Some(state_match) = cap.get(0) {
                        match state_match.as_str() {
                            "do" => {allowed = true},
                            "don't" => {allowed = false},
                            _ => {
                                if allowed {
                                    if let (Some(num1), Some(num2)) = (cap.get(2), cap.get(3)) {
                                        let num1: i64 = num1.as_str().parse().unwrap();
                                        let num2: i64 = num2.as_str().parse().unwrap();
                                        num += num1*num2;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => panic!("{}", e)
        }
    }

    println!("Num: {}", num);

    Ok(())
}

