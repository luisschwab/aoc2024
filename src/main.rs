use aoc2024::{
    day01,
};

use std::collections::HashSet;
use std::env;

fn main() {
	let args = env::args().skip(1).collect::<HashSet<_>>();
    
    println!("\nAdvent of Code 2024\n");

  	if args.is_empty() || args.contains("1") {
		println!("Day 01, part 1: {}", day01::part01().unwrap());
		println!("Day 01, part 2: {}", day01::part02().unwrap());
		println!("");
	}
}

