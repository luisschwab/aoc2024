use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

// we are given a matrix of 'X', 'M', 'A', 'S',
// and have to find the number of occurrences of
// 'XMAS' in all directions
pub fn part01() -> io::Result<u32> {
    let path = "inputs/input04.txt";

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let target = vec!['X', 'M', 'A', 'S'];
    
    // 
    let orientations = vec![
        (0,1),
        (1,0),
        (1,1,),
        (-1,1),
        (0,-1),
        (-1,0),
        (-1,-1),
        (1,-1)
    ];

    let mut unique_occurrences = HashSet::new();

    for start_row in 0..n_rows {
        for start_col in 0..n_cols {
            for &(dy, dx) in &orientations {
                if let Some(occurrence) = search_xmas_from_point(&grid, &target, start_row, start_col, dy, dx) {
                    unique_occurrences.insert(occurrence);
                }
            }
        }
    }
    
    Ok(unique_occurrences.len() as u32)
}

// search for XMAS from a given 
// position and through a certain orientation
fn search_xmas_from_point(
    grid: &Vec<Vec<char>>, 
    target: &Vec<char>, 
    start_row: usize, 
    start_col: usize, 
    dy: i32, 
    dx: i32
) -> Option<Vec<(usize, usize)>> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut path = Vec::new();

    for i in 0..target.len() {
        let row = start_row as i32 + i as i32 * dy;
        let col = start_col as i32 + i as i32 * dx;

        if row < 0 || row >= rows || col < 0 || col >= cols {
            return None;
        }

        if grid[row as usize][col as usize] != target[i] {
            return None;
        }

        path.push((row as usize, col as usize));
    }

    Some(path)
}

// now we need to       M.S
// find two MAS's in    .A. 
// the shape of an X:   M.S
pub fn part02() -> io::Result<u32> {
    let path = "inputs/input04.txt";

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    const SAM: [char; 3] = ['S', 'A', 'M'];
    const MAS: [char; 3] = ['M', 'A', 'S'];

    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut xmas_count: u32 = 0;

    for row in 0..(n_rows-2) {
        for col in 0..(n_cols-2) {
            
            // check diagonals for SAM or MAS
            let cand1 = [
                grid[row][col],
                grid[row + 1][col + 1],
                grid[row + 2][col + 2],
            ];

            let cand2 = [
                grid[row][col + 2],
                grid[row + 1][col + 1],
                grid[row + 2][col],
            ];

            if (cand1 == MAS || cand1 == SAM) && (cand2 == SAM || cand2 == MAS) {
                xmas_count += 1;
            }
        }
    }

    Ok(xmas_count)
}

