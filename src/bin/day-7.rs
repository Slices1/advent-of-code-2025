use std::fs;
use std::error::Error;
extern crate memoise;
use memoise::memoise;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Beam, // 'S'
    Empty, // '.'
    Splitter, // '^'
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
        for row in grid.iter() {
        for cell in row.iter() {
            // print!("{}", sym);
            print!("{}", match cell {
                Cell::Beam => 'S',
                Cell::Empty => '.',
                Cell::Splitter => '^',
            });
        }
        println!("");
    }
}

#[memoise(row <= 143, col <= 143)]
fn timelines(grid: &Vec<Vec<Cell>>, row: usize, col: usize) -> u64 {
    // println!("Current depth: {}/71", row);
    
    // base case
    if row >= grid.len() - 1 {
        return 1;
    }

    if grid[row+1][col] == Cell::Beam {
        return timelines(&grid, row + 1, col);
    }

    if grid[row+1][col] == Cell::Splitter {
        return timelines(&grid, row + 1, col+1) + timelines(&grid, row + 1, col-1);
    }


    println!("This should never print");
    return 0;
}

fn main() -> Result<(), Box<dyn Error>> {
    // remove last line of dots in the input file. replace the starting coord of S if it is different on code line 105
    let content = fs::read_to_string("inputs/7.txt")
        .or_else(|_| fs::read_to_string("../../inputs/7.txt"))
        .expect("Could not find input file in either location");    

    let mut grid: Vec<Vec<Cell>> = content
        .split("\n.............................................................................................................................................\n")
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '^' => Cell::Splitter,
                    'S' => Cell::Beam,
                    _ => Cell::Empty, // Default to empty for '.' or others
                })
                .collect()
        })
        .collect();
    // AHHHHAAAAAAAAAAAAAAAAAAA GAD DAMN THESE RUST STRINGS WHY ARENT THEY JUST BYTES
    // muuuch better, still annoying to print these enums tho. cant use "as i32"


    // print
    // print_grid(&grid);

    let mut tally = 0;

    for i in 0..(grid.len()-1) {
        let current_row = grid[i].clone();
        let row_below = &mut grid[i + 1];

        for j in 0..current_row.len() {
            if current_row[j] != Cell::Beam { continue;}
            // cell above is beam
            // or splitter is to left and beam is above it
            // same for right
            if row_below[j] == Cell::Empty{
                row_below[j] = Cell::Beam;
                continue;
            }
            if row_below[j] == Cell::Splitter {
                row_below[j+1] = Cell::Beam;
                row_below[j-1] = Cell::Beam;
                tally += 1;
            }
        }
        
        // print
        // print_grid(&grid);
        // println!("Number of splits (part 1): {}", tally);
    }


    // tally
    println!("\nNumber of splits (part 1): {}", tally);
    
    println!("Number of timelines: {}", timelines(&grid, 0, 70));
        
    
    Ok(())
}
