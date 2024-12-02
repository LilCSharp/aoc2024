use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub enum Monotonicity {
    INC,
    DEC,
}

fn is_softvalid(vals: &Vec<i32>) -> bool {
    let len = vals.len();
    for i in 0..len {
        let mut modvec = vals.clone();
        modvec.remove(i);
        if is_valid(&modvec) {
            return true;
        }
    }

    return false;
}

fn is_valid(vals: &Vec<i32>) -> bool {
    let state: Monotonicity;
    let mut num: i32;
    let mut nums = vals.iter().peekable();
    match nums.next() {
        Some(start) => {
            num = *start;
            match nums.peek() {
                Some(next) => {
                    if start < *next {
                        state = Monotonicity::INC;
                    } else if start > *next {
                        state = Monotonicity::DEC;
                    } else {
                        return false;
                    }
                }
                None => { return true }
            }
        }
        None => { return false }
    }

    while let Some(next) = nums.next() {
        match state {
            Monotonicity::INC => {
                if num >= *next {
                    return false;
                } else if *next > num + 3 {
                    return false;
                }
                num = *next;
            }
            Monotonicity::DEC => {
                if num <= *next {
                    return false;
                } else if *next < num - 3 {
                    return false;
                }

                num = *next;
            }
        }
    }

    return true;
}

fn main() -> io::Result<()> {
    let file = File::open("assets/challenge.txt")?;
    let reader = BufReader::new(file);
    let mut count: i64 = 0;
    let mut softcount: i64 = 0;

    for line in reader.lines() {
        match line {
            Ok(s) => {
                let nums: Vec<i32> = s.split(" ")
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect();

                if is_valid(&nums) { 
                    count += 1;
                }

                if is_softvalid(&nums) {
                    softcount += 1;
                }
            }
            Err(e) => panic!("{}", e)
        }
    }

    println!("Original Count: {}", count);
    println!("Adjustable Count: {}", softcount);

    Ok(())
}
