use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn solve_gaussian(num_rows: usize, buttons: &Vec<Vec<u32>>, target: &Vec<u32>) -> u64 {
    let nc = buttons.len();
    let mut mat = vec![vec![0.0; nc + 1]; num_rows];
    for (c, btn) in buttons.iter().enumerate() {
        for &r in btn { if (r as usize) < num_rows { mat[r as usize][c] = 1.0; } }
    }
    for r in 0..num_rows { mat[r][nc] = target[r] as f64; }

    let (mut pivs, mut cols, mut r) = (HashMap::new(), Vec::new(), 0);
    for c in 0..nc {
        if r >= num_rows { break; }
        if let Some(pr) = (r..num_rows).find(|&i| mat[i][c].abs() > 1e-4) {
            mat.swap(r, pr);
            let div = mat[r][c];
            for j in c..=nc { mat[r][j] /= div; }
            for i in 0..num_rows {
                if i != r && mat[i][c].abs() > 1e-4 {
                    let f = mat[i][c];
                    for j in c..=nc { mat[i][j] -= f * mat[r][j]; }
                }
            }
            pivs.insert(c, r);
            cols.push(c);
            r += 1;
        }
    }

    if (r..num_rows).any(|i| mat[i][nc].abs() > 1e-4) { return u64::MAX; }

    let free: Vec<_> = (0..nc).filter(|c| !pivs.contains_key(c)).collect();
    let bounds: HashMap<_, _> = free.iter().map(|&f| (f, (0..num_rows)
        .filter(|&r| buttons[f].contains(&(r as u32)))
        .map(|r| (target[r] as f64).floor() as u64).min().unwrap_or(0))).collect();

    let eqs: Vec<_> = cols.iter().map(|&p| {
        let row = pivs[&p];
        (mat[row][nc], free.iter().filter(|&&f| mat[row][f].abs() > 1e-4).map(|&f| (f, mat[row][f])).collect::<Vec<_>>())
    }).collect();

    let mut min = u64::MAX;
    fn dfs(i: usize, fr: &[usize], bds: &HashMap<usize, u64>, eqs: &[(f64, Vec<(usize, f64)>)], asn: &mut HashMap<usize, u64>, min: &mut u64) {
        let cur: u64 = asn.values().sum();
        if cur >= *min { return; }
        if i == fr.len() {
            let mut p_sum = 0;
            for (k, deps) in eqs {
                let v = deps.iter().fold(*k, |acc, (f, c)| acc - c * (*asn.get(f).unwrap() as f64));
                if v < -1e-4 || (v - v.round()).abs() > 1e-4 { return; }
                p_sum += v.round() as u64;
            }
            *min = (*min).min(cur + p_sum);
            return;
        }
        let f = fr[i];
        let w = 1.0 - eqs.iter().map(|(_, d)| d.iter().find(|(x, _)| *x == f).map_or(0.0, |(_, c)| *c)).sum::<f64>();
        let (start, end, step) = if w < 0.0 { (bds[&f] as i64, -1, -1) } else { (0, bds[&f] as i64 + 1, 1) };
        let mut v = start;
        while v != end {
            asn.insert(f, v as u64);
            dfs(i + 1, fr, bds, eqs, asn, min);
            v += step;
        }
        asn.remove(&f);
    }
    dfs(0, &free, &bounds, &eqs, &mut HashMap::new(), &mut min);
    min
}

fn main() -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string("inputs/10.txt")
  .or_else(|_| fs::read_to_string("../../inputs/10.txt"))
  .expect("Could not find input file in either location");    
  
  let indicator_light_diagrams: Vec<&str> = content.lines().collect();
  
  // print indicator light diagrams
  // for diagram in indicator_light_diagrams {
  //   println!("{:?}", diagram);
  // }
  
  // try all combinations of button presses to find the one that minimises total presses
  let mut press_tally: u64 = 0;
  let mut part_2_press_tally: u64 = 0;
  
  for diagram in indicator_light_diagrams {
    if diagram.trim().is_empty() { continue; } // safe skip empty lines

    let mut parts = diagram.split(' ');
    let required_state_temp = parts.next().unwrap();
    let required_state: Vec<u8> = required_state_temp[1..required_state_temp.len()-1]
                                               .as_bytes()
                                               .into_iter()
                                               .map(|x| match x {
                                                 b'.' => 0,
                                                 b'#' => 1,
                                                 _ => panic!("Invalid character in required state"),
                                               })
                                               .collect();
    let mut button_lists: Vec<Vec<u32>> = vec![]; 
    for _ in 0..(parts.clone().collect::<Vec<&str>>().len() - 1) {
      let s = parts
                        .next()
                        .unwrap();
      button_lists.push(s[1..s.len()-1] // remove first and last char (brackets)
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                       );
    }
    let joltage_required_state_temp = parts.next().unwrap();
    let joltage_required_state = joltage_required_state_temp[1..joltage_required_state_temp.len()-1]
                                        .split(',')
                                        .map(|x| x.parse::<u32>().unwrap())
                                        .collect::<Vec<u32>>();

    // try printing
    // println!("Required state: {:?}", required_state);
    // println!("Button lists: {:?}", button_lists);
    // println!("Joltage requirement: {}", joltage_required_state);

    // logic to determine minimum presses
    let mut min_presses: u64 = 9999;
    // try every combination
    // the state modulused by 2 must equal the required state. this is because of the toggle nature
    let num_buttons = button_lists.len();
    for i in 0..(1 << num_buttons) {
      let mut current_state: Vec<u8> = vec![0; required_state.len()];
      let mut presses: u64 = 0;
      for j in 0..num_buttons {
        if (i & (1 << j)) != 0 {
          // press button j
          presses += 1;
          for &index in &button_lists[j] {
            let idx = index as usize;
            if idx < current_state.len() { // safety check
                current_state[idx] ^= 1; // toggle state using xor with 1
            }
          }
        }
      }
      if current_state == required_state {
        if presses < min_presses {
          min_presses = presses;
        }
      }
    }

    // println!("Min presses for this diagram to turn on: {}\n", min_presses);
    if min_presses == 9999 { min_presses = 0; } // handle unreachable part 1
    press_tally += min_presses;



    ////// PART 2
    // try every combination.
    // this time, you can press a single button multiple times.
    // each button press incrememnts a the corresponding joltage counter
    // we have to keep pressing until the joltage_state == joltage_required_state

    // we can write this as a matrix to solve, but there could be multiple solutions and we need to find the
    // one that minimises button presses, so we should just brute force in a smart way

    // so I will try to test button combinations in this order:
    // 000
    // 001
    // 010
    // ...
    
    // basically we interpret the sequence of buttons as binary, then test every combination
    // then we interpret as ternary and test every remaining combination, then quaternary, etc
    // up to a reasonable limit

    // reasonable limit for brute force depth
    // let max_base = 9; 

    // // interpret the sequence of buttons as binary (base 2), then ternary (base 3), etc
    // for base in 2..=max_base {
    //     // dynamic counter, length equals number of buttons
    //     let mut press_counts = vec![0u32; num_buttons];

    //     loop {
    //         // optimisation: if we are in base 3 (0,1,2), we want to skip anything that is purely 0s and 1s
    //         // because we already checked those in base 2.
    //         // so if all digits are strictly less than current base-1, we skip.
    //         let already_checked = base > 2 && press_counts.iter().all(|&x| x < base - 1);

    //         if !already_checked {
    //             let mut current_joltage = vec![0u32; joltage_required_state.len()];
    //             let mut total_presses: u64 = 0;

    //             // calculate effect of current combination
    //             for (btn_idx, &count) in press_counts.iter().enumerate() {
    //                 if count > 0 {
    //                     total_presses += count as u64;
    //                     for &target_idx in &button_lists[btn_idx] {
    //                         if (target_idx as usize) < current_joltage.len() {
    //                             current_joltage[target_idx as usize] += count;
    //                         }
    //                     }
    //                 }
    //             }

    //             // check if we hit the target
    //             if current_joltage == joltage_required_state {
    //                 if total_presses < part_2_min_presses {
    //                     part_2_min_presses = total_presses;
    //                     // print press_counts for debugging
    //                     println!("new part 2 counts {:?}", press_counts);
    //                 }
    //             }
    //         }

    //         // increment our counter
    //         let mut i = 0;
    //         while i < num_buttons {
    //             press_counts[i] += 1;
    //             if press_counts[i] < base {
    //                 break;
    //             } else {
    //                 press_counts[i] = 0;
    //                 i += 1;
    //             }
    //         }

    //         // if we overflowed the last digit, we've finished this base
    //         if i == num_buttons {
    //             break;
    //         }
    //     }
    // }

    // if part_2_min_presses == u64::MAX {
    //     // just in case we didn't find one within the limit
    //     part_2_min_presses = 0; 
    //     println!("No solution found within limit for this diagram.");
    // }

    // that was far too slow
    // we'll have to use matrices (probably)

    // use gaussian elimination
    let part_2_min_presses = solve_gaussian(joltage_required_state.len(), &button_lists, &joltage_required_state);

    if part_2_min_presses == u64::MAX {
        // Warning only, don't crash
        println!("Warning: Could not solve diagram {:?}", diagram);
    }
    part_2_press_tally += part_2_min_presses;
  }
  
  println!("Part 1 min presses: {}", press_tally);
  println!("Part 2 min presses: {}", part_2_press_tally);
  
  Ok(())
}