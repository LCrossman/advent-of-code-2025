use std::ops::{Add, Sub};
use std::fs::File;
use std::cmp::{self, Ordering};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::str::FromStr;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;
use std::num;
use std::collections::HashSet;


//from the std Ord docs with impl for PartialEq, Eq PartialOrd and Ord 
#[derive(Debug, Clone)] 
struct Distance {
    index1: u64,
    index2: u64,
    distance: f64,
}
impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.distance.total_cmp(&other.distance) == Ordering::Equal
    }
}
impl Eq for Distance {}
impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

#[derive(Debug, Clone)]
struct UnionFind {
   parent: Vec<usize>,
   num_components: usize,
}
impl UnionFind {
   fn new(size: usize) -> Self {
      UnionFind {
         parent: (0..size).collect(),
	 num_components: size,
	 }
      }
   fn find(&mut self, node: usize) -> usize {
      if self.parent[node] == node {
         node
      } else {
         let leader = self.parent[node];
	 self.parent[node] = self.find(leader);
	 self.parent[node]
	 }
      }
    fn union(&mut self, node1: usize, node2: usize) -> bool {
        let root1 = self.find(node1);
	let root2 = self.find(node2);
	if root1 != root2 {
	   self.parent[root1] = root2; // group merging
	   self.num_components -= 1; //one less isolated group
	   true }
	else {
	   false }
	}
}



//read lines function is from the Rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Find Connected Components
fn find_circuit_sizes(all_node_ids: Vec<u64>, graph: &HashMap<u64, Vec<u64>>) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut circuit_sizes = Vec::new();

    // Iterate over every single node we know about
    for node_id in all_node_ids {
        if visited.contains(&node_id) {
            continue;
        }

        // We found a new unvisited node! This starts a new "Circuit"
        let mut current_circuit_size = 0;
        let mut stack = vec![node_id];
        visited.insert(node_id);

        // Run DFS to find all friends in this circuit
        while let Some(current) = stack.pop() {
            current_circuit_size += 1;

            // Check neighbors
            if let Some(neighbors) = graph.get(&current) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        // If we haven't visited this neighbor, add to stack
                        stack.push(neighbor);
                    }
                }
            }
        }
        circuit_sizes.push(current_circuit_size);
    }
    circuit_sizes
}

fn build_simple_graph(distances: &[Distance]) -> HashMap<u64, Vec<(u64, f64)>> {
    let mut graph: HashMap<u64, Vec<(u64, f64)>> = HashMap::new();

    for d in distances {
        //skipping self matches of 0 distance
        if d.index1 == d.index2 { continue; }

        //add the edge
	if d.index1 < d.index2 {
            graph.entry(d.index1)
                .or_default()
                .push((d.index2, d.distance));
            //add reverse for undirected graph If your graph is UNDIRECTED (A->B implies B->A), add the reverse:
            graph.entry(d.index2)
                .or_default()
                .push((d.index1, d.distance));
		}
    }
    graph
}

// We strictly use the indices here so we are not using the distances here
fn build_connectivity_graph(edges: &[Distance]) -> HashMap<u64, Vec<u64>> {
    let mut graph: HashMap<u64, Vec<u64>> = HashMap::new();
    for d in edges {
        graph.entry(d.index1).or_default().push(d.index2);
        graph.entry(d.index2).or_default().push(d.index1);
    }
    graph
}

fn main() {
    let mut grid: BTreeMap<u64, (i64, i64, i64)> = BTreeMap::new();
    let mut i = 0;
    println!("here in main");
    //read in file
    if let Ok(lines) = read_lines("query_input_day8.txt") {
        println!("ok line");
        for line in lines.map_while(Result::ok) {
            let elements: Vec<i64> = line.trim().split(',')
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect();
            // Storing as i64 helps with subtraction logic later
            grid.insert(i, (elements[0], elements[1], elements[2]));
	    println!("inserting {:?}", &elements[0]);
            i += 1;
        }
    }

    let mut distances: Vec<Distance> = Vec::new();
    //converting to a Vec so we can iterate easieily
    let node_ids: Vec<u64> = grid.keys().cloned().collect();
    
    for (idx, &id1) in node_ids.iter().enumerate() {
        for &id2 in &node_ids[idx + 1..] { // Start from idx + 1 - avoiding duplicates
            let p1 = grid[&id1];
            let p2 = grid[&id2];

            // Distance Formula is Euclidean from wikipedia
            let dist_sq = (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2);
            let dist = (dist_sq as f64).sqrt(); //actual distance

            distances.push(Distance { 
                index1: id1, 
                index2: id2, 
                distance: dist 
            });
        }
    }
    //println!("distances {:?}", &distances);
    // Sort by Distance - needs my impl Ord
    distances.sort();

    // Part I - Take the top N edges
    //let limit = 1000;
    //let edges_to_connect: Vec<Distance> = distances.iter().take(limit).cloned().collect();
    // Build Graph from those edges
    //let graph = build_connectivity_graph(&edges_to_connect);
    //println!("graph {:?}", &graph);
    //Find Circuit Sizes
    //let mut sizes = find_circuit_sizes(node_ids, &graph);
    //println!("sizes {:?}", &sizes);
    //sizes.sort_by(|a, b| b.cmp(a)); // Sort descending (Largest first)
    //println!("Circuit sizes found: {:?}", sizes);

    let num_nodes = grid.len();
    let mut uf = UnionFind::new(num_nodes);
    println!("starting with {:?} isolated circuits", &num_nodes);
    for edge in distances {
       let u = edge.index1 as usize;
       let v = edge.index2 as usize;
       if uf.union(u, v) {
         //then connection has been successful is true
	 if uf.num_components == 1 {
	    println!("ALL CONNECTED!");
	    println!("final connection was {:?} and {:?}", &u, &v);
	    let box1 = grid[&(u as u64)];
	    let box2 = grid[&(v as u64)];
	    println!("box 1 x {:?}", &box1.0);
	    println!("box 2 x {:?}", &box2.0);
	    let answer = (box1.0 as u128) * (box2.0 as u128);
	    println!("answer is {:?}", &answer);
            break;
	    }
	 }
       }
}
//part I needs the product of the top 3
//    if sizes.len() >= 3 {
//        let answer = sizes[0] * sizes[1] * sizes[2];
//        println!("Product of top 3: {}", answer);
//    } else {
//        println!("Not enough circuits found to multiply top 3!");
//    }
//}