use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("assets/challenge.txt")?;
    let reader = BufReader::new(file);
    let mut prim_heap: BinaryHeap<i64> = BinaryHeap::new();
    let mut sec_heap: BinaryHeap<i64> = BinaryHeap::new();
    let mut distances: Vec<i64> = Vec::new();
    let mut sec_map: HashMap<i64, i64> = HashMap::new();
    let mut score: i64 = 0;

    for line in reader.lines() {
        match line {
            Ok(s) => {
                let pairs: Vec<i64> = s.split("   ")
                    .filter_map(|nums| nums.parse::<i64>().ok())
                    .collect();
                prim_heap.push(pairs[0]);
                sec_heap.push(pairs[1]);

                match sec_map.get(&pairs[1]) {
                    Some(count ) => { sec_map.insert(pairs[1], count+1); }
                    None => { sec_map.insert(pairs[1], 1); }
                }
            }
            Err(e) => panic!("{}", e)
        }
    }

    while let (Some(prim), Some(sec)) = (prim_heap.pop(), sec_heap.pop()) {
        distances.push((prim - sec).abs());
        match sec_map.get(&prim) {
            Some(count) => { score += count * prim; }
            None => {}
        }
    }

    println!("{:?}", distances.iter().sum::<i64>());
    println!("Score: {}", score);

    Ok(())
}