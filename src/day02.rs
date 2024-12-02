use std::fs::File;
use std::vec::Vec;
use std::io::{self, prelude::*, BufReader};

// in order for a report to be safe, it must be monotonic,
// and adjacent numbers need to have a delta of value between 1 and 3
pub fn part01() -> io::Result<u32> {
    let path = "inputs/input02.txt";

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut safe_reports: u32 = 0;
    
    for line in reader.lines() {
        let elements: Vec<i32> = line?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        // check if elements increase monotonically + delta constraints
        let increasing = elements.windows(2).all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);

        // check if elements decrease monotonically + delta constraints
        let decreasing = elements.windows(2).all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);

        if (increasing || decreasing) {
            safe_reports += 1;
        }
    }

    Ok(safe_reports)
}

// now, an invalid report will be valid 
// iff removing a single sample makes it valid
pub fn part02() -> io::Result<u32> {
    let path = "inputs/input02.txt";

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut safe_reports: u32 = 0;
    
    for line in reader.lines() {
        let elements: Vec<i32> = line?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        // create subsets of `elements` and break on the first valid subset
        for i in 0..elements.len() {
            let subset: Vec<i32> = elements
                .iter()
                .enumerate()
                .filter(|&(index,_)| index != i)
                .map(|(_, &val)| val)
                .collect();

            // check if elements increase monotonically + delta constraints
            let increasing = subset.windows(2).all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);

            // check if elements decrease monotonically + delta constraints
            let decreasing = subset.windows(2).all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);

            if (increasing || decreasing) {
                safe_reports += 1;
                break;
            }
        }
    }

    Ok(safe_reports)
}

