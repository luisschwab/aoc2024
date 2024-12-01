use std::fs::File;
use std::vec::Vec;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

// we are given two lists of numbers. we need to
// pair the smallest from the 1st with the smallest from the 2nd
// until all elements are consumed
pub fn part01() -> io::Result<u32> {
    let path = "inputs/input01.txt";

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();

        let num1: u32 = parts.next().unwrap().parse().unwrap();
        let num2: u32 = parts.next().unwrap().parse().unwrap();
        
        list1.push(num1);
        list2.push(num2);
    }
    
    // sort both lists so we can just pop elements instead of 
    // finding the smallest on every iteration
    list1.sort();
    list2.sort();

    // take elems pairwise and compute diff
    for _ in 0..list1.len() {
        let num1 = list1.pop().unwrap() as i32;
        let num2 = list2.pop().unwrap() as i32;
        sum += (num1 - num2).abs();
    }
    //println!("sum: {}", sum);

    Ok(sum as u32)
}

// now, we need to compute a similarity score between both lists
// by seeing how many equal elements from the 1st are on the 2nd
pub fn part02() -> io::Result<u32> {    
    let path = "inputs/input01.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();

        let num1: u32 = parts.next().unwrap().parse().unwrap();
        let num2: u32 = parts.next().unwrap().parse().unwrap();
        
        list1.push(num1);
        list2.push(num2);
    }

    // condense the numbers from list2 into a hashmap
    // key is the number, value is number of ocurrences
    let mut map: HashMap<u32, u32> = HashMap::new();
    
    for num in list2 {
        if map.contains_key(&num) {
            *map.entry(num).or_insert(0) += 1;
        } else {
            map.insert(num, 1);
        }
    }
    
    // now, iterate over list1 and multiply each element to its
    // corresponding value in the map
    let mut similarity_score: u32 = 0;

    for num in list1 {
        if map.contains_key(&num) {
            similarity_score += num * map.get(&num).copied().unwrap_or(0);
            //println!("{}", similarity_score);
        }
    }

    Ok(similarity_score)
}

