use std::fs;
use std::error::Error;
use memoise::memoise;
extern crate memoise;

#[memoise(start_index <= 600)]
fn find_num_paths_to_end(start_index: usize, end_index: usize, nodes: &Vec<Node>) -> u128 {
    // base case
    if start_index == end_index {
        return 1;
    }
    return nodes[start_index].edges.iter().map(|&edge_index| {
        find_num_paths_to_end(edge_index, end_index, &nodes)
    }).sum();
}

#[memoise(start_index <= 600, dac_visited <= 1, fft_visited <= 1)]
fn find_num_paths_to_end_part_2(start_index: usize, end_index: usize, nodes: &Vec<Node>, 
                                mut dac_visited: bool, mut fft_visited: bool, dac_index: usize, 
                                fft_index: usize) -> u128 {
    // base case
    if start_index == end_index {
      if dac_visited && fft_visited {
        return 1;
      } else {
        return 0;
      }
    } else if start_index == dac_index {
      dac_visited = true;
    } else if start_index == fft_index {
      fft_visited = true;
    }


    return nodes[start_index].edges.iter().map(|&edge_index| {
      find_num_paths_to_end_part_2(edge_index, end_index, &nodes, dac_visited, 
                                   fft_visited, dac_index, fft_index)
    }).sum();
}

struct Node {
  // index: usize,
  edges: Vec<usize>, // the indices of nodes it connects to
}


fn main() -> Result<(), Box<dyn Error>> {
  // to run, add "out: " to the end of input file

  let content = fs::read_to_string("inputs/11.txt")
  .or_else(|_| fs::read_to_string("../../inputs/11.txt"))
  .expect("Could not find input file in either location");    
  
  // directed graph
  // How many different paths lead from you to out?
  
  
  // parse input into graph
  let parts_vec: Vec<(&str, Vec<&str>)> = content.lines().collect::<Vec<&str>>().iter().map(|&l| { 
                  let halves: Vec<&str> = l.split(": ").collect();
                  (halves[0], if halves.len() > 1 {halves[1].split_whitespace().collect()} else { Vec::new() })
                                                                                  }).collect();
  // try printing
  // for parts in &parts_vec {
  //   println!("({}, {:?})", parts.0, parts.1);
  // }


  let mut nodes: Vec<Node> = Vec::new();
  for parts in &parts_vec {
    // parse this
    // (aaa, ["iii", "ccc"])
    // (ccc, ["iii"])
    // (iii, ["out"])
    // (out, [])

    // into this:
    // Node { index: 0, edges: [1, 2] }
    // Node { index: 1, edges: [2] }
    // etc

    let edges: Vec<usize> = parts.1.iter().map(|&edge_name| {
      // find index of edge_name in parts_vec
      parts_vec.iter().position(|&(name, _)| name == edge_name).unwrap()
    }).collect();
    nodes.push( Node { edges } );
  }

  // try printing nodes
  // for node in &nodes {
  //   println!("Node {}: edges to {:?}", node.index, node.edges);
  // }


  // dynamically set you_index to the index of "you" and the same for the others
  // part 1
  let you_index: usize = parts_vec.iter().position(|&(name, _)| name == "you").unwrap();
  let out_index: usize = parts_vec.iter().position(|&(name, _)| name == "out").unwrap();

  
    println!("Part 1 number of possible routes: {}", 
            find_num_paths_to_end(you_index, out_index, &nodes));

           
  let svr_index: usize = parts_vec.iter().position(|&(name, _)| name == "svr").unwrap();
  let dac_index: usize = parts_vec.iter().position(|&(name, _)| name == "dac").unwrap();
  let fft_index: usize = parts_vec.iter().position(|&(name, _)| name == "fft").unwrap();

  // possible routes can be found as
  // svr -> dac * dac -> fft * fft -> out
  // but order doesnt matter
  // so you need to add:
  // svr -> fft * fft -> dac * dac -> out
  // I dont think I need to worry about combinations like svr -> dac -> fft -> dac -> out  because they should be impossible
  
  // let part_2_num_possble_routes: u128 = 
  //     find_num_paths_to_end(svr_index, dac_index, &nodes) *
  //     find_num_paths_to_end(dac_index, fft_index, &nodes) *
  //     find_num_paths_to_end(fft_index, out_index, &nodes) +
  //     find_num_paths_to_end(svr_index, fft_index, &nodes) *
  //     find_num_paths_to_end(fft_index, dac_index, &nodes) *
  //     find_num_paths_to_end(dac_index, out_index, &nodes);

  // was way too high

  let part_2_num_possble_routes: u128 = 
      find_num_paths_to_end_part_2(svr_index, out_index, 
              &nodes, false, false, dac_index, fft_index);

  println!("Part 2 number of possible routes: {}", part_2_num_possble_routes);
  Ok(())
}