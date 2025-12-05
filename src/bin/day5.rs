use std::fs::File;
use std::cmp::{self, Ordering};
use std::collections::HashSet;
use std::collections::HashMap;
use std::str::FromStr;
use std::io::{self, BufRead};
use std::path::Path;
use std::num;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Fresh {
   rangestart: u64,
   rangeend: u64,
}
//adapted from rust language forum
trait Contains {
    fn contains(&self, elt: u64) -> bool;
}
impl Contains for Fresh {
   fn contains(&self, elt: u64) -> bool { self.rangestart <= elt && elt <= self.rangeend }
}
impl Fresh {
   fn new(rangestart: u64, rangeend: u64) -> Self {
      Self { rangestart, rangeend }
      }
   fn len(&self) -> u64 {
      self.rangeend - self.rangestart + 1
      }
   fn overlaps_or_touches(&self, other: &Fresh) -> bool {
        // Because we will sort the list before calling this, we know 
        // 'other' will always start >= 'self'.
        // We use +1 because 3-5 and 6-10 are contiguous and should be merged.
        other.rangestart <= self.rangeend + 1
    }
    fn merge(&mut self, other: &Fresh) {
       self.rangeend = cmp::max(self.rangeend, other.rangeend);
       }
}
//AI assisted part two, needs to use sort and merge 
fn solve_part_two(mut ranges: Vec<Fresh>) -> u64 {
   ranges.sort_unstable_by_key(|r| r.rangestart);
   if ranges.is_empty() {
        return 0;
    }
    let mut merged_ranges = Vec::new();
    let mut current = ranges[0];
    for next in &ranges[1..] {
        if current.overlaps_or_touches(next) {
            // If they overlap or touch, extend the current range
            current.merge(next);
        } else {
            // No overlap: Push the finished range and switch to the new one
            merged_ranges.push(current);
            current = *next;
        }
    }
    merged_ranges.push(current);

    // 3. Sum the lengths
    merged_ranges.iter().map(|r| r.len()).sum()
}

//for part II, adapted using from day2 - too slow to use for the large input
fn provide_ids(first_id: u64, last_id: u64, hash: &mut HashSet<u64>) -> &mut HashSet<u64> {
    let mut id = first_id;
    hash.insert(first_id);
    while id != last_id {
        id = id + 1;
        hash.insert(id);
    }
    hash
}

fn count_ids(first_id: u64, last_id: u64) -> u64 {
    let id = (last_id - first_id)+1;
    id
}


//read lines function is from the Rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut ranges: Vec<Fresh> = Vec::new();
    let mut finalranges: Vec<Fresh> = Vec::new();
    //used for part I
    //let mut tests: Vec<u64> = Vec::new();
    //used for part I
    //let mut ingredients: HashSet<u64> = HashSet::new();
    //used for part I
    //let mut test: u64 = 0;
    //used for part I
    //let mut flag: i8 = 0;
    let mut ids = 0;
    let mut totalids = 0;
    //let mut all_ingredients: HashSet<u64> = HashSet::new();
    if let Ok(lines) = read_lines("query_input_day5.txt") {
        for line in lines.map_while(Result::ok) {
	     let inp = line.trim();
	     println!("input is {:?}", &inp);
	     if line.is_empty() {
	         //now we are in the test set
		 println!("in the empty line");
		 //used for partI
		 //flag+= 1;
		 break;
		 }
	     else {
	         //used for part I
	         //if flag == 0 
	             //we are in the rangeset 
		     let res: Vec<_> = inp.split('-').collect();
		     let start = res[0].parse::<u64>().unwrap();
		     let end = res[1].parse::<u64>().unwrap();
		     //used for part I
                     let fresh = Fresh { rangestart: start, rangeend: end };
		     println!("fresh is {:?} ranges {:?}", &fresh, &ranges);
		     ranges.push(fresh);
		  
		 }
	}
    }
    let answer = solve_part_two(ranges);
    println!("and the answer is {:?}", &answer);
}
//used for part I
//else {
//    test = inp.parse::<u64>().unwrap();
//    tests.push(test);
//    }
	         

    //used in part I
    //for t in tests {
    //    for rang in &ranges {
    //	    if rang.contains(t) {
    //                   println!("it does {:?} {:?}", &t, &rang);
   //		   ingredients.insert(t);
   //		   }
    //	    }
    //	}
   // println!("and the answer is {:?}", &all_ingredients.len());

