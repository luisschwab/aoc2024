use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

// we have to parse out instances of `mul(a,b)` from the input
// a and b can be 1 to 3 digits long, then multiply them and sum everything
pub fn part01() -> io::Result<u32> {
    let path = "inputs/input03.txt";

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    // we can use a regex to find `mul()` ops
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    let mut line_content;
    let mut sum: u32 = 0;
    for line in reader.lines() {
        line_content = line?;
        for (_, [a, b]) in re.captures_iter(&line_content).map(|c| c.extract()) {
            let a: u32 = a.parse().unwrap();
            let b: u32 = b.parse().unwrap();
            
            sum += a * b;
        }
    }
    //println!("{:?}", sum);

    Ok(sum)
}

pub fn part02() -> io::Result<u32> {
    let path = "inputs/input03.txt";

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    // we can use a regex to find `mul()`, `do()` and `don't()` ops
    let re = Regex::new(r"(?P<mul>mul\((?P<a>\d{1,3}),(?P<b>\d{1,3})\))|(?P<do>do\(\))|(?P<dont>don't\(\))").unwrap();
    
    let mut sum: u32 = 0;
    let mut compute: bool = true;
    let mut line_content;

    for line in reader.lines() {
        line_content = line?;
        for capture in re.captures_iter(&line_content) {
            if let Some(_) = capture.name("mul") {
                let a: u32 = capture.name("a").unwrap().as_str().parse().unwrap();
                let b: u32 = capture.name("b").unwrap().as_str().parse().unwrap();

                //println!("a: {}, b: {}", a, b);
                
                // only do muls if the last statement was nil, `mul()` or `do()`
                if compute {
                    sum += a * b;
                }

            } else if capture.name("do").is_some() {
                compute = true;
            } else if capture.name("dont").is_some() {
                compute = false;
            }
        }
    }
    //println!("{}", sum);

    Ok(sum)
}

