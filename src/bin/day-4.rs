use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/4.txt")
        .or_else(|_| fs::read_to_string("../../inputs/4.txt"))
        .expect("Could not find input file in either location");    
    
    let lines: Vec<&str> = content.lines().collect();

    // input is grid of 1s and 0s
    // make 2d vector of bools

    let mut grid: Vec<Vec<bool>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c == '@')
                .collect()
        })
        .collect();

    // print grid
    for row in &grid {
        for &cell in row {
            print!("{}", if cell { '1' } else { '0' });
        }
        println!();
    }

    // corners can have max 3 neighbors
    // edges can have max 5 
    
    // we need to increment a tally for every cell == 1 where sum of neighbours < 4
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    let mut tally = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] {
                let mut neighbor_count = 0;

                for (dx, dy) in &directions {
                    let ni = i as isize + dx;
                    let nj = j as isize + dy;

                    if ni >= 0 && ni < grid.len() as isize && nj >= 0 && nj < grid[i].len() as isize {
                        neighbor_count += grid[ni as usize][nj as usize] as usize;
                    }
                }

                if neighbor_count < 4 {
                    tally += 1;
                }
            }
        }
    }

    println!("Part 1 Result: {}", tally);

    
    let mut something_was_removed = true;
    let mut removal_tally = 0;

    while something_was_removed {
        something_was_removed = false;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] {
                    let mut neighbor_count = 0;

                    for (dx, dy) in &directions {
                        let ni = i as isize + dx;
                        let nj = j as isize + dy;

                        if ni >= 0 && ni < grid.len() as isize && nj >= 0 && nj < grid[i].len() as isize {
                            if grid[ni as usize][nj as usize] {
                                neighbor_count += 1;
                            }   
                        }
                    }

                    if neighbor_count < 4 {
                        grid[i][j] = false;
                        something_was_removed = true;
                        removal_tally += 1;
                    }
                }
            }
        }
    }

    println!("Part 2 Result: {}", removal_tally);

    
    Ok(())
}