use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Reusing the read_lines helper
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    //Parse the input file
    if let Ok(lines) = read_lines("query_input_day11.txt") {
        for line in lines.map_while(Result::ok) {
            if let Some((node, neighbors)) = line.split_once(':') {
                let node_name = node.trim().to_string();
                let neighbor_list: Vec<String> = neighbors
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                graph.insert(node_name, neighbor_list);
            }
        }
    }

    //part I here
    //Memoization Cache: Key is the node name, Value is path count to "out"
    //let mut memo: HashMap<String, u64> = HashMap::new();
    //part II we need to track NodeName, Saw_DAC, Saw_FFT
    let mut memo: HashMap<(String, bool, bool), u64> = HashMap::new();

    //Start a DFS from "you" - part I
    //let total_paths = count_paths("you".to_string(), &graph, &mut memo);
    //Start a DFS from 'svr' instead of 'you'
    let total_paths = count_paths("svr".to_string(), false, false, &graph, &mut memo);

    println!("Total paths from 'svr' to 'out' whilst visiting dac and fft: {}", &total_paths);
}

// Optimized DFS with Memoization
//part I
//fn count_paths(current: String, graph: &HashMap<String, Vec<String>>,memo: &mut HashMap<String, u64>) -> u64 {
//part II
fn count_paths(current: String, mut saw_dac: bool, mut saw_fft: bool, graph: &HashMap<String, Vec<String>>, memo: &mut HashMap<(String,bool, bool), u64>) -> u64 {
    if current == "dac" { saw_dac = true; }
    if current == "fft" { saw_fft = true; }
    
    // Base Case: We hit the end
    if current == "out" {
        //part I
        //return 1;
	//part II
	return if saw_dac && saw_fft { 1 } else { 0 };
    }
    let state = (current.clone(), saw_dac, saw_fft);
    // Return cached result if we've been here before
    //part I
    //if let Some(&count) = memo.get(&current) {
    //part II
    if let Some(&count) = memo.get(&state) {
        return count;
    }

    let mut total_from_here = 0;

    // Explore neighbors
    if let Some(neighbors) = graph.get(&current) {
        for neighbor in neighbors {
	    //part I
            //total_from_here += count_paths(neighbor.clone(), graph, memo);
	    //part II
	    total_from_here += count_paths(neighbor.clone(), saw_dac, saw_fft, graph, memo);
        }
    }

    // Store in cache
    //part I
    //memo.insert(current, total_from_here);
    //part II
    memo.insert(state, total_from_here);
    total_from_here
}