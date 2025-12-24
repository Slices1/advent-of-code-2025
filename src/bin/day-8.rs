use std::fs;
use std::error::Error;
use std::cmp::Ordering;

fn compare(coord1: &(usize, usize, u64), coord2: &(usize, usize, u64)) -> Ordering {
    coord1.2.cmp(&coord2.2)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/8.txt")
        .or_else(|_| fs::read_to_string("../../inputs/8.txt"))
        .expect("Could not find input file in either location");    

    let lines: Vec<&str> = content.lines().collect();

    let coordinates: Vec<Vec<u32>> = lines.iter()
        .map(|line| {
            line.split(',')
                .map(|num| num.trim().parse::<u32>().expect("parse failed"))
                .collect()
        })
        .collect();

    // store pairs as (index_u, index_v, distance).
    let mut pairs: Vec<(usize, usize, u64)> = Vec::new();
    let coords_len = coordinates.len();
    for i in 0..coords_len {
        for j in (i + 1)..coords_len {
            let coord1 = &coordinates[i];
            let coord2 = &coordinates[j];
            
            let dist: u64 = ((coord2[0] as i64 - coord1[0] as i64).pow(2) + 
                             (coord2[1] as i64 - coord1[1] as i64).pow(2) + 
                             (coord2[2] as i64 - coord1[2] as i64).pow(2)) as u64;
            
            pairs.push((i, j, dist));
        }
    }

    pairs.sort_by(compare);

    let top_count = 1000;


    // map every node index to a list of its connected neighbors.
    let mut adj: Vec<Vec<usize>> = vec![vec![]; coords_len];
    
    // we only process the top `top_count` edges, as per puzzle rules.
    for (u, v, _dist) in pairs.iter().take(top_count) {
        adj[*u].push(*v);
        adj[*v].push(*u);
    }

    // flood fill
    // iterate through every node. If we haven't visited it yet, it's the start
    // of a new circuit. We flood fill to find the size of that circuit.
    let mut visited = vec![false; coords_len];
    let mut circuit_sizes: Vec<u32> = Vec::new();

    for i in 0..coords_len {
        if visited[i] { continue; }

        let mut size = 0;
        let mut stack = vec![i]; // stack for iterative flood fill
        visited[i] = true;

        while let Some(node) = stack.pop() {
            size += 1;
            
            // Check all neighbors of the current node
            for &neighbor in &adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
        circuit_sizes.push(size);
    }

    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a)); 
    
    let result: u32 = circuit_sizes.iter().take(3).product();
    
    println!("Circuit sizes: {:?}", circuit_sizes);
    println!("Part 1 result: {}", result);

    let x_coord_1 = coordinates[pairs[5244 - 1].0][0] as u64;
    let x_coord_2 = coordinates[pairs[5244 - 1].1][0] as u64;
    println!("For part 2, the last two junction boxes to connect have x coords: {} and {}", x_coord_1, x_coord_2);
    println!("Their product is: {}", x_coord_1 * x_coord_2);
    Ok(())
}


// for part 2, I just kept increasing top_count in code until all junction boxes were connected.
// by manual binary search, the top_count which connects them all into 1 circuit is 5244.