use std::fs;
use std::error::Error;

fn is_point_in_polygon(polygon: Vec<(u32, u32)>, point: (f64, f64)) -> bool {
  let mut inside = false;
  let num_vertices = polygon.len();
  
  let (p1x, p1y) = polygon[0];
  let mut p1x = p1x as f64;
  let mut p1y = p1y as f64;
  let (x, y) = point;
  
  for i in 0..=num_vertices {
    let (p2x_u, p2y_u) = polygon[i % num_vertices];
    let p2x = p2x_u as f64;
    let p2y = p2y_u as f64;
    
    if p1y.min(p2y) < y && y < p1y.max(p2y) {
      let xinters = (y - p1y) * (p2x - p1x) / (p2y - p1y) + p1x;
      if p2x == p1x {
        if x < p1x {
          inside = !inside;
        }
      } else {
        if x < xinters {
          inside = !inside;
        }
      }
    }
    p1x = p2x;
    p1y = p2y;
  }
  inside
}

fn is_valid_green_rectangle(polygon: Vec<(u32, u32)>, coord1: (u32, u32), coord2: (u32, u32)) -> bool {
  let (x1, y1) = coord1;
  let (x2, y2) = coord2;
  
  let r_min_x = x1.min(x2); 
  let r_max_x = x1.max(x2);
  let r_min_y = y1.min(y2);
  let r_max_y = y1.max(y2);
  let num_vertices = polygon.len();
  
  // iterate through every EDGE of the polygon.
  for i in 0..num_vertices {
    let (ux, uy) = polygon[i];
    let (vx, vy) = polygon[(i + 1) % num_vertices]; // wrap around to the first point
    
    // check if u is inside the rectangle
    if r_min_x < ux && ux < r_max_x && r_min_y < uy && uy < r_max_y {
      return false; 
    }
    
    // check if edge intersects rectangle
    if ux == vx { // vertical edge
      if r_min_x < ux && ux < r_max_x {
        let overlap_min = uy.min(vy).max(r_min_y);
        let overlap_max = uy.max(vy).min(r_max_y);
        if overlap_max > overlap_min {
          return false;
        }
      }
    } else { // horizontal edge
      if r_min_y < uy && uy < r_max_y {
        let overlap_min = ux.min(vx).max(r_min_x);
        let overlap_max = ux.max(vx).min(r_max_x);
        if overlap_max > overlap_min {
          return false;
        }
      }
    }
  }
  // if no intersections, then rectangle is entirely outside polygon or entirely inside polygon,
  // we will check 1 point to determine which
  // nvm 
  if !is_point_in_polygon(polygon.clone(), (r_min_x as f64 + 0.5, r_min_y as f64 + 0.5)) {
    return false;
  }
  return true;
}

fn main() -> Result<(), Box<dyn Error>> {
  // remove last line of dots in the input file. replace the starting coord of S if it is different on code line 105
  let content = fs::read_to_string("inputs/9.txt")
  .or_else(|_| fs::read_to_string("../../inputs/9.txt"))
  .expect("Could not find input file in either location");    
  
  // content has form
  // 7,1
  // 11,1
  // 11,7
  let coords: Vec<(u32, u32)> = content
  .lines()
  .map(|l| {
    let mut parts = l.split(',');
    // println!("line: {:?}", l);
    let x: u32 = parts.next().unwrap().parse().unwrap();
    let y: u32 = parts.next().unwrap().parse().unwrap();
    (x, y)
  })
  .collect();
  
  let mut largest_area: u64 = 1;
  
  for coord1 in coords.clone() {
    for coord2 in coords.clone() {
      if coord1 == coord2 { continue; }
      let area: u64 = (((coord1.0 as i64 - coord2.0 as i64).abs() + 1) * ((coord1.1 as i64 - coord2.1 as i64).abs() + 1) ) as u64;
      if area > largest_area {
        largest_area = area;
      }
    }
  }
  
  println!("Part 1 largest area: {}", largest_area);
  
  largest_area = 1;
  
  for coord1 in coords.clone() {
    for coord2 in coords.clone() {
      if coord1 == coord2 { continue; }
      let area: u64 = (((coord1.0 as i64 - coord2.0 as i64).abs() + 1) * ((coord1.1 as i64 - coord2.1 as i64).abs() + 1) ) as u64;
      if area > largest_area {
        // check if it is within the bounds of the shape
        // we can do this by checking that the two corners (coord1 and coord2) are to the left of
        // every pair of consecutive coords in our coords vector.
        // so we will index through the coords vector
        
        // println!("coord1, coord2: {:?}, {:?}", coord1, coord2);
        
        // let mut within_bounds = true;
        // for i in 0..coords.len() {
        //   let a = coords[i];
        //   let b = coords[(i + 1) % coords.len()]; // wrap around to the first point
        
        //   if a.0 == b.0 {
        //     if a.1 < b.1 {
        //       if coord1.0 > a.0 { within_bounds = false; println!("1");}
        
        //     } else {
        //       if coord1.0 < a.0 { within_bounds = false; println!("2");}
        //     }
        //   } else {
        //     if a.0 < b.0 {
        //       if coord1.1 > a.1 { within_bounds = false; println!("3");}
        //     } else {
        //       if coord1.1 < a.1 { within_bounds = false; println!("4");}
        //     }
        //   }
        // }
        
        // screw it! this will never work when an edge of our desired rectangle is not within the shape 
        // lets check every vertex!
        // a grid would have approx 10 billion items, so thats a no go. lets try this 
        
        // I need to update is_point_in_shape to handle edges properly
        
        if is_valid_green_rectangle(coords.clone(), coord1, coord2) {
          largest_area = area;
        }
      }
    }
  }
  
  println!("Part 2 largest area: {}", largest_area);
  
  Ok(())
}
