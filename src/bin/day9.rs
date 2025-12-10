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


//what we are going to need to do is to create the grid
//and we need to put the tiles in it
//then the area of the rectangle is the width * height so we need to know those items
//we therefore need to know the length distances between the tiles
//so the coordinates such as (7,1) and (11,7) provide the width as 7-1+1 and 11-7+1 is 5 by 7
//and we don't even need to check if there are any tiles in the middle

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Grid {
    x: i64,
    y: i64,
}
impl Grid {
    fn new() -> Self {
      Self {
         x: 0,
	 y: 0,
	 }
      }
}
impl Grid {
    fn sub(mut self, other: &Self) -> (i64, i64) {
       let x_val = (self.x - other.x) + 1 ;
       let y_val = (self.y - other.y) + 1 ;
       (x_val, y_val)
       }
}

//read lines function is from the Rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
     let mut grid = Grid::new();
     let mut max_area = 0;
     let mut grid_vec: Vec<Grid> = Vec::new();
     if let Ok(lines) = read_lines("query_input_day9.txt") {
     for line in lines.map_while(Result::ok) {
           let inp = line.trim();
	   let vectors: Vec<_> = inp.split(",").collect();
	   grid = Grid { x: vectors[0].parse::<i64>().unwrap(), y: vectors[1].parse::<i64>().unwrap() };
           grid_vec.push(grid);
	   }
     }
     grid_vec.sort_by(|a, b| b.cmp(a));
     for g_vec in &grid_vec {
	for g2_vec in &grid_vec {
	    let result = g_vec.sub(&g2_vec);
	    let area = result.0 * result.1;
	    println!("g_vec is {:?} g2_vec is {:?} result is {:?} area is {:?}", &g_vec, &g2_vec, &result, &area);
	    if area > max_area {
	       max_area = area;
	       }
	}
     }
     println!("the max area is {:?}", &max_area);
}