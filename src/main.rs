use aoc2024::{
    day01,
    day02,
    day03,
    day04,
    day05,
};

use std::collections::HashSet;
use std::env;

fn main() {
	let args = env::args().skip(1).collect::<HashSet<_>>();
    
    println!("\nAdvent of Code 2024\n");

  	if args.is_empty() || args.contains("1") {
		println!("Day 01");
		println!("  Part 01: {}", day01::part01().unwrap());
		println!("  Part 02: {}", day01::part02().unwrap());
        println!("");
	}
    
  	if args.is_empty() || args.contains("2") {
		println!("Day 02");
		println!("  Part 01: {}", day02::part01().unwrap());
        println!("  Part 02: {}", day02::part02().unwrap());
		println!("");
	}

  	if args.is_empty() || args.contains("3") {
		println!("Day 03");
		println!("  Part 01: {}", day03::part01().unwrap());
        println!("  Part 02: {}", day03::part02().unwrap());
		println!("");
	}

  	if args.is_empty() || args.contains("4") {
		println!("Day 04");
		println!("  Part 01: {}", day04::part01().unwrap());
        println!("  Part 02: {}", day04::part02().unwrap());
		println!("");
	}

  	if args.is_empty() || args.contains("5") {
		println!("Day 05");
		println!("  Part 01: {}", day05::part01().unwrap());
        println!("  Part 02: {}", day05::part02().unwrap());
		println!("");
	}

}

