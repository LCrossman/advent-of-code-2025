use std::ops::{Add, Sub};
use std::fs::File;
use std::cmp::{self, Ordering};
use std::collections::HashMap;
use std::str::FromStr;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;
use std::num;


//read lines function is from the Rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut dim_x = 0;
    let mut dim_y = 0;
    let mut total_access = 0;
    let mut new_topvec: Vec<Vec<_>> = Vec::new();
    //let mut topvec_new: Vec<Vec<_>> = Vec::new();
    if let Ok(lines) = read_lines("query_input_day7.txt") {
        for line in lines.map_while(Result::ok) {
              let inp: &str = line.trim();
	      dim_x = inp.len();
	      let mut vecy: Vec<_> = vec![0; dim_x];
	      for (i,el) in line.chars().enumerate() {
	         //println!("still on the line {:?} {:?}", &line, &el);
	         match el {
	             'S' => { vecy[i] = 1; },
		     '^' => { vecy[i] = 2; },
		     '.' => { vecy[i] = 0; },
		     _ => { println!("unexpected symbol in the bagging area"); },
		     }
		 }
	      new_topvec.push(vecy.clone());
	      dim_y+=1;
	      }
	    }
   for li in &new_topvec {
      println!("new topvec is now {:?}", &li);
      }
   let mut prev_topvec: Vec<Vec<_>> = Vec::new();
   let mut beam_splitter = 0;
   while prev_topvec != new_topvec {
       println!("going round");
       for j in 0..dim_y {
	   prev_topvec = new_topvec.clone();
           for k in 0..dim_x {
	       match new_topvec[j][k] {
	           0 => { if j > 0 && j < dim_y-1 {
		             if new_topvec[j-1][k] == 3 {
			         new_topvec[j][k] = 3
				 }
			     }
			},
	           1 => { new_topvec[j][k] = 1;
                          new_topvec[j+1][k] = 3;
                         },
		   2 => { new_topvec[j][k] = 2;
		          if k > 0 {
		                 new_topvec[j][k-1] = 3;
				 }
			  if k < dim_x {	 
			         new_topvec[j][k+1] = 3;
				 }
		          if j > 0 && j < dim_y-1 {
			       if new_topvec[j-1][k] == 3 {
			           beam_splitter+=1;
				   }
			       }
			  },
		   3 => { new_topvec[j][k] = 3;
		          //if j < dim_y-1 {
		           // new_topvec[j+1][k] = 3;
			   // }
			   },
		    _ => { println!("unexpected character in the bagging area, {:?}", &new_topvec[j][k]); },
		    }
		
		    }
	        }
	      for li in &new_topvec {
                   println!("new topvec is now {:?}", &li);
                   }
           }
    println!("and verrily the beam_splitter is {:?}", &beam_splitter);
 }
             