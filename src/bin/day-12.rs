use std::fs;
use std::error::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BitGrid {
    // max region size = 50x50
    // 50x50 = 2500 bits. 
    // 40 * 64 = 2560 bits capacity.
    chunks: [u64; 40], 
}
impl BitGrid {
    pub fn new() -> Self {
        Self { chunks: [0; 40] }
    }

    // (board & mask) logic
    #[inline(always)]
    pub fn overlaps(&self, other: &BitGrid) -> bool {
        for i in 0..40 {
            if (self.chunks[i] & other.chunks[i]) != 0 {
                return true;
            }
        }
        false
    }

    // (board || mask) logic
    #[inline(always)]
    pub fn merge(&self, other: &BitGrid) -> Self {
        let mut new_grid = *self; // valid because we derived Copy
        for i in 0..40 {
            new_grid.chunks[i] |= other.chunks[i];
        }
        new_grid
    }

    pub fn set_bit(&mut self, row: usize, col: usize, width: usize) {
        let bit_index = row * width + col;
        
        let chunk_index = bit_index / 64;
        let bit_offset = bit_index % 64;
        
        if chunk_index < 40 {
            self.chunks[chunk_index] |= 1 << bit_offset;
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrecomputedPlacement {
    pub mask: BitGrid,
    // might store original (row, col) here to print the solution later
}

pub struct SolverContext {
    // Index = Shape ID. 
    // Value = All valid BitGrid masks for that shape on this specific board size.
    pub placements_by_shape: Vec<Vec<PrecomputedPlacement>>,
    
    // The specific list of pieces we need to fit for this query.
    // e.g. if we need two of shape 0 and one of shape 5, this looks like: [0, 0, 5]
    // sorted by difficulty (largest area first) for speed.
    pub pieces_to_solve: Vec<usize>,
}

fn can_presents_fit_in_region(present_amounts: Vec<usize>, region_size: (usize, usize), binary_presents: &Vec<Vec<Vec<u8>>>) -> bool {
  // all presents have convex hull of 3x3
  // therefore we can immediately accept any situation where all presents fit side by side
  // presents can't be stacked, so we only care about the x
  if present_amounts.clone().into_iter().sum::<usize>()*3 <= region_size.0 {
    return true;
  } 
   
  // we can also perform a check to see if the total area of presents exceeds the area available
  let num_presents = present_amounts.clone().iter().sum::<usize>();
  let total_volume_of_presents = num_presents*7; // all presents have 7 units of area and 2 units of air
  if total_volume_of_presents > region_size.0 * region_size.1 {
    return false;
  }

  // now we check every combination of them fitting together
  // the present shapes are hardcoded so we can use them in code

  // ###
  // ##.
  // ##.

  // ###
  // ##.
  // .##

  // .##
  // ###
  // ##.

  // ##.
  // ###
  // ##.

  // ###
  // #..
  // ###

  // ###
  // .#.
  // ###

  // altho they are already stored in binary_presents

  // allowed transformations: rotation, flipping, translation

  // we can check for overlaps by brute force placing them in every possible position
  // adding their binary arrays together and checking for any values > 1 and breaking early if it happens

  // initialise region
  let region: Vec<Vec<u8>> = vec![vec![0; region_size.0]; region_size.1];
  

  // for present_num in 0.. {
  //   for num_quarter_rotations in 0..=3 { // all possible quarter rotations
  //     for 0..num_quarter_rotations { rotate_90_degrees(&present); }
  
  //     for x in 0..=(region_size.0 - 3) { // all x translation possibilities
  //       for y in 0..=(region_size.1 - 3) { // all y translation possibilities
  //         // place it here

  //         // check it here

  //         // continue if invalid here
        
  //       }
  //     }
  //   }
  // }
  

  // for 2 presents that can be in 2 spots
  // for present2 in both spots
  //  // place
  //  // test
  //  // clear
  
  // how do we do this with a dynamic num of presents

  // lets see if that could work
  // operations for a place and test: 2*50*30 + 50*30*2
  // combinations: (50*50*4 )^(num presents) 
  // num presents: 300
  // max est possible operations = (2*50*30 + 50*30*2) * (50*50*4)^(300) = 6 × 10^1203
  // yep not happening. I didnt even factor in reflections
  
  // we'll need some optimisations
  // based on the estimation I just did, minimising the total x and y coords that presents can be in
  // might be enough to make it run in my lifetime.
  // basically if 1 present is at x=0, then others at the same y level can only be at x>=3
  // others at a y level 1 or 2 different can only be at x >= 2

  // other facts we can use:
  // - each present has exactly 2 units of air and 7 units filled -> we can minimise present overlapping even more using this
  // - each present has its top and bottom edge always filled

  // If I store PrecomputedPlacements of every piece for every combination beforehand, that would be how much memory:
  // num pieces * (num permutations) * (bytes ber PrecomputedPlacements) = 
  // num unique pieces = 5
  // num permutations per piece =  48*48*4*2 // x, y, rotation, mirror
  // bits ber PrecomputedPlacements = 2560/8
  // therefore the memory = 5 * 48*48*4*2 * 2560/8 = 29491200 bytes = 29 Megabytes
  // which is easilt doable


  false
}

fn main() -> Result<(), Box<dyn Error>> {
  // To run, add another newline before the last section of the input file.
  // also remove the numbers above the shapes.
  // e.g.
  // ###
  // ##.
  // ##.
  //
  // ###
  // ##.
  // .##
  //
  // ...
  //
  // ###
  // .#.
  // ###
  //
  //
  // 4x4: 0 0 0 0 2 0
  // ...

  let content = fs::read_to_string("inputs/12.txt")
  .or_else(|_| fs::read_to_string("../../inputs/12.txt"))
  .expect("Could not find input file in either location");    

  let sections: Vec<&str> = content.split("\n\n\n").collect();
  let present_shapes: Vec<&str> = sections[0].split("\n\n").collect();
  let binary_presents: Vec<Vec<Vec<u8>>> = present_shapes.into_iter().map(|shape| {
    shape.lines().map(|line| {
      line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>()
  }).collect::<Vec<Vec<Vec<u8>>>>();
  // this will be indexed as: [which present][row][col]

  let regions_section = sections[1].lines().collect::<Vec<&str>>();
  let mut region_sizes: Vec<(usize, usize)> = Vec::new();
  let mut region_presents: Vec<Vec<usize>> = Vec::new();
  for region in regions_section {
    let parts: Vec<&str> = region.split(":").collect();
    let size_part = parts[0].trim();
    let presents_part = parts[1].trim();
    let sizes = size_part.split("x").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let presents = presents_part.split(" ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    region_sizes.push((sizes[0], sizes[1]));
    region_presents.push(presents);
  }
  // try printing
  // for shape in binary_presents {
  //   for row in shape {
  //     for &cell in &row {
  //       print!("{}", if cell == 1 { '#' } else { '.' });
  //     }
  //     println!();
  //   }
  //   println!();
  // }

  // println!("Region sizes: {:?}", region_sizes);
  // println!("Region presents: {:?}", region_presents);


  // now we need to make a function which checks if a number of presents can fit in a region

  // now loop over all regions and check if the presents can fit
  let mut tally = 0;
  for (i, region_size) in region_sizes.iter().enumerate() {
    let present_amount = &region_presents[i];
    let can_fit = can_presents_fit_in_region(present_amount.clone(), *region_size, &binary_presents);
    if can_fit { tally += 1; }
    println!("Region {}: Can fit presents: {}", i + 1, can_fit);
  }

  let total_regions = region_sizes.len();
  println!("Total regions that can fit presents: {}/{}", tally, total_regions);

  Ok(())
}