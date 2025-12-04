use std::ops::{Add, Sub};
use std::fs::File;
use std::cmp::{self, Ordering};
use std::collections::HashMap;
use std::str::FromStr;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;
use std::num;

#[derive(Debug, Clone, Copy)]
struct Positions {
    north: i32,
    east: i32,
    west: i32,
    south: i32,
    north_west: i32,
    north_east: i32,
    south_west: i32,
    south_east: i32,
}
impl Positions {
   fn new() -> Self {
      Self {
         north: 0,
	 east: 0,
	 west: 0,
	 south: 0,
	 north_west: 0,
	 north_east: 0,
	 south_west: 0,
	 south_east: 0,
	 }
      }
}
impl Positions {
   fn add(mut self) -> bool {
      let access = self.north + self.east + self.west + self.south + self.north_west + self.north_east + self.south_west + self.south_east;
      if access < 4 {
          return true;
	  }
      false
      }
}
    

//read lines function is from the Rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut total_sum: u64 = 0;
    let mut dim_x = 0;
    let mut dim_y = 0;
    let mut total_access = 0;
    let mut topvec: Vec<Vec<_>> = Vec::new();
    if let Ok(lines) = read_lines("query_input_day4.txt") {
        for line in lines.map_while(Result::ok) {
              let inp: &str = line.trim();
	      dim_x = inp.len();
	      let mut vecy: Vec<_> = vec![0; dim_x];
	      for (i,el) in line.chars().enumerate() {
	         if el == '@' {
		     vecy[i] = 1
		     }
		 }
	      topvec.push(vecy);
	      dim_y+=1;
	      }
	      
	}
   for j in 0..dim_y {
      let mut n = 0;
      let mut s = 0;
      let mut w = 0;
      let mut e = 0;
      let mut nw = 0;
      let mut ne = 0;
      let mut se = 0;
      let mut sw = 0;
      for k in 0..dim_x {
      if topvec[j][k] == 1 {
      //for (k, element) in item.iter().enumerate() {
          //println!("j {:?} k {:?} item {:?} element {:?}", &j, &k, &topvec[j], &topvec[j][k]);
	  let positions = Positions::new();
	  //println!("k is {:?} dim is {:?} j is {:?}", &k, &dim_x, &j);
	  let w = if k < dim_x-1 { topvec[j][k+1] } else { 0 };
	  let ne = if k < dim_x-1 && j.checked_sub(1).is_some() { topvec[j-1][k+1] } else { 0 };
	  if k.checked_sub(1).is_some() {
	      e = topvec[j][k.saturating_sub(1)];}
	  if j.checked_sub(1).is_some() {
	     n = topvec[j.saturating_sub(1)][k];}
	  let s = if j < dim_y-1 { topvec[j+1][k] } else { 0 };
	  let sw = if j < dim_y-1 && k.checked_sub(1).is_some() { topvec[j+1][k.saturating_sub(1)] } else { 0 };
	  if j.checked_sub(1).is_some() && k.checked_sub(1).is_some() {
	     nw = topvec[j.saturating_sub(1)][k.saturating_sub(1)];}
	  let se = if j < dim_y-1  && k < dim_x-1 { topvec[j.saturating_add(1)][k + 1] } else { 0 };
	  let test_positions = Positions { north: n, west: w, east: e, south: s, north_west: nw, north_east: ne, south_west: sw, south_east: se};
	  let result = test_positions.add();
	  //println!("test positions is {:?}", &test_positions);
	  if result {
	      //println!("can access");
	      total_access+=1;
              }
          }
      }
      }
   println!("rolls is {:?}", &total_access);
}
             