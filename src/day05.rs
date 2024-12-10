use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::{HashMap, HashSet};

// we need to figure out which updates to the sleigh
// launch safety manual are according to spec
pub fn part01() -> io::Result<u32> {
    let path = "inputs/input05.txt";
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();
    
    // page ordering rules
    let mut rules: Vec<(u8,u8)> = Vec::new();
    for line in &mut lines {
        let line = line?;

        if line.is_empty() { break; }

        let rule: Vec<u8> = line.split('|').map(|n| n.parse().unwrap()).collect();

        rules.push((rule[0], rule[1]));
    }

    // updated pages
    let mut updates: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        let line = line?;

        let update: Vec<u8> = line.split(',').map(|n| n.parse().unwrap()).collect();

        updates.push(update);
    }

    // we have to check that each update is in accordance to the rules that apply
    // ie: for a rule to apply, `a` and `b` need to be in the update, otherwise it does not
    let mut valid_updates: Vec<Vec<u8>> = Vec::new();
    for update in updates {
        let mut rules_subset: Vec<(u8,u8)> = Vec::new();
        
        // for each update, find the subset of rules that apply
        for rule in &rules {
            if update.contains(&rule.0) && update.contains(&rule.1) {
                rules_subset.push(*rule);
            }
        }

        // check if this update is valid to all rules in `rules_subset`
        if is_update_valid(&update, &rules_subset) {
            valid_updates.push(update);
        }
    }
    //for valid_update in &valid_updates {
    //    println!("{:?}", valid_update);
    //}

    // find the middle page number of valid updates and sum them up
    let mut sum: u32 = 0;
    for valid_update in valid_updates {
        sum += valid_update[valid_update.len() / 2] as u32;
    }

    Ok(sum)
}

// now we have to take incorrect updates,
// and reorder them in such a way to make them correct
pub fn part02() -> io::Result<u32> {
    let path = "inputs/input05.txt";
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();
    
    // page ordering rules: given a rule (a, b),
    // we need to make sure that page `a` comes before page `b`
    // in the update, else it's invalid
    let mut rules: Vec<(u8,u8)> = Vec::new();
    for line in &mut lines {
        let line = line?;

        if line.is_empty() { break; }

        let rule: Vec<u8> = line.split('|').map(|n| n.parse().unwrap()).collect();

        rules.push((rule[0], rule[1]));
    }

    // updated pages
    let mut updates: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        let line = line?;

        let update: Vec<u8> = line.split(',').map(|n| n.parse().unwrap()).collect();

        updates.push(update);
    }
    
    // find all updates that are invalid and reorder them
    let mut reordered_updates: Vec<Vec<u8>> = Vec::new();
    for update in updates {
        let mut rules_subset: Vec<(u8,u8)> = Vec::new();
        
        // for each update, find the subset of rules that apply
        for rule in &rules {
            if update.contains(&rule.0) && update.contains(&rule.1) {
                rules_subset.push(*rule);
            }
        }

        // now we check if this update is invalid to any rule in `rules_subset`,
        // if so, reorder it using Kahn's
        if !is_update_valid(&update, &rules_subset) {
            let reordered = topological_sort(&update, &rules_subset);

            reordered_updates.push(reordered);
        }
    }
    
    
    let mut sum: u32 = 0;
    for reordered_update in reordered_updates {
        sum += reordered_update[reordered_update.len() / 2] as u32;
    }

    Ok(sum)
}

fn is_update_valid(update: &[u8], rules: &[(u8, u8)]) -> bool {
    rules.iter().all(|&(a, b)| {
        let a_pos = update.iter().position(|&x| x == a).unwrap();
        let b_pos = update.iter().position(|&x| x == b).unwrap();
        a_pos < b_pos
    })
}

// blindly applying the rules is not robust enough for the input,
// but it works on the example input. 
// since `rules_subset` is a DAG, we can do a topological sort 
// with Kahn's algorithm to create a valid order based on the rules
fn topological_sort(update: &[u8], rules: &[(u8, u8)]) -> Vec<u8> {
    // adjacency list
    let mut graph: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut in_degree: HashMap<u8, usize> = HashMap::new();

    for &page in update {
        graph.entry(page).or_default();
        in_degree.entry(page).or_insert(0);
    }

    for &(a, b) in rules {
        graph.entry(a).or_default().insert(b);
        *in_degree.entry(b).or_insert(0) += 1;
    }

    // topological sort using Kahn's algorithm
    let mut queue: Vec<u8> = update
        .iter()
        .filter(|&&page| in_degree[&page] == 0)
        .cloned()
        .collect();

    let mut result = Vec::new();
    while !queue.is_empty() {
        let page = queue.remove(0);
        result.push(page);

        if let Some(neighbors) = graph.get(&page) {
            for &neighbor in neighbors {
                *in_degree.get_mut(&neighbor).unwrap() -= 1;
                if in_degree[&neighbor] == 0 {
                    queue.push(neighbor);
                }
            }
        }
    }

    if result.len() != update.len() {
        return update.to_vec();
    }

    result
}
    
