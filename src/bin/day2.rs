use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use counter::Counter;
use std::collections::HashMap;


//original check repeat function for part I
fn check_repeats(id: &str) -> bool {
   let id_chars: Vec<_> = id.chars().collect();
   let mut right = id_chars.len()/2;
   println!("right is {:?} id chars is {:?}", &right, &id_chars);
   for left in 0..right {
       //println!("c is {:?} right {:?} right id chars {:?}", &left, &right, &id_chars[right]);
       if id_chars[left] != id_chars[right] {
           return false;
	   }
       if right+1 < id_chars.len() {
          right+=1;
	  }
       }
   true
}

//final part II checking for bool
fn is_invalid_id(id: &str) -> bool {
    let s = id.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    for k in 1..=(len/2) {
       //println!("k is {:?}", &k);
       if len %k == 0 {
          //check if string is periodic with 'k'
	  let mut is_periodic = true;
	  for i in k..len {
	     if bytes[i] != bytes[i-k] {
	        is_periodic=false;
		break;
		}
	     }
      if is_periodic {
         return true;
	 }
         }
      }
   false
}

fn provide_ids(first_id: i64, last_id: i64) -> Vec<i64> {
    let mut collected_ids: Vec<i64> = Vec::new();
    let mut id = first_id;
    collected_ids.push(first_id);
    while id != last_id {
        id = id + 1;
        collected_ids.push(id);
    }
    println!("these are provided ids {:?}", &collected_ids);
    collected_ids
}



//read lines function is from the Rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut total_sum: i64 = 0;
    if let Ok(lines) = read_lines("input_query_day2.txt") {
        for line in lines.map_while(Result::ok) {
              let inp = line.trim();
              let test_input: Vec<&str> = inp.split(",").collect();
	      for test_inp in test_input {
	          let test_res: Vec<_> = test_inp.split("-").collect();
                  let first_id: i64 = test_res[0].parse::<i64>().unwrap();
                  let last_id: i64 = test_res[1].parse::<i64>().unwrap();
                  let provided_ids = provide_ids(first_id, last_id);
		  for id in provided_ids {
		     let string_id = &id.to_string();
		     println!("and the string id is {:?}", &string_id);
		     let ch = (&id.to_string()).chars().nth(0).unwrap();
		     if ch == '0' {
		       continue;
		       }
		     if id == 101 {
		        continue;
			}
		     let the_id: Vec<char> = (&id.to_string()).chars().collect();
		     println!("the id is {:?}", &the_id);
		     //needed for task part 1 - only send char vectors with even numbers to the function
		     //if the_id.len() % 2 == 0 {
		        let answer = is_invalid_id(string_id);
			if answer {
                            total_sum+=id;
                            }
			println!("current answer {:?}", &total_sum);
		       // }
		  }
	      }
	 }
	 }
    println!("the total sum is {:?}", &total_sum);
}