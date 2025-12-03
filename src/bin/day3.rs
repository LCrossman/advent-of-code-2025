use std::fs::File;
use std::cmp::{self, Ordering};
use std::collections::HashMap;
use std::str::FromStr;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;


fn choose_results<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}


#[derive(Debug, Clone)]
struct Joltage {
    jolts: u64,
}
impl Joltage {
    fn new() -> Self {
       Joltage {
          jolts: 0,
	  }
       }
}

//the impl for part I
//impl Joltage {
//    fn choose(jolts: Vec<u64>) -> u64 {
//       let vec1 = jolts;
//       let mut maxnum = 0;
//       let all = vec1.into_iter().combinations(2);
//       let mut jolt1 = 0;
//       let mut jolt2 = 0;
//       for a in all {
//          println!("this is a {:?}", &a);
//          jolt1 = a[0];
//	  jolt2 = a[1];
//        let num = (jolt1 * 10) + jolt2;
// 	  if num > maxnum {
//	     maxnum = num;
//	     }
//	  }
  //     maxnum
    //   }
//}

//my working function for part II, works with example but is TOO SLOW for the query which is much larger
//impl Joltage {
//    fn choose(jolts: Vec<u64>) -> u64 {
//       let vec1 = jolts;
//       let mut maxnum = 0;
//       let all = vec1.into_iter().combinations(12);
//       for a in all {
//	     let joined = Itertools::join(&mut a.iter(), "");
//	     let upto12 = &joined[0..12];
//	     let num = joined.parse::<u64>().unwrap();
//	     if num > maxnum {
//	         maxnum = num;
//	         }
//	     }
  //     maxnum
    //   }
//}

//AI assisted function
impl Joltage {
    fn choose(jolts: Vec<u64>) -> u64 {
        // We want a result of exactly 12 digits
        let target_len = 12;
        
        // Calculate how many digits we are allowed to 'skip' or 'remove'
        // to maximize the remaining ones.
        let mut attempts_remaining = jolts.len().saturating_sub(target_len);
        
        let mut stack: Vec<u64> = Vec::with_capacity(target_len);

        for &digit in &jolts {
            // While the stack is not empty, 
            // AND the current digit is bigger than the last kept digit,
            // AND we can afford to remove more digits:
            while let Some(&last) = stack.last() {
                if digit > last && attempts_remaining > 0 {
                    stack.pop();
                    attempts_remaining -= 1;
                } else {
                    break;
                }
            }
            stack.push(digit);
        }

        // If the digits were all descending (e.g. 98765...), we might not have 
        // removed enough. We must truncate to exactly 12 digits.
        stack.truncate(target_len);

        // Convert the vector of digits back into a single u64
        // (Assumes the result fits in u64, otherwise return String)
        let result_str = stack.iter()
            .map(|d| d.to_string())
            .collect::<String>();
            
        result_str.parse::<u64>().unwrap()
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
    let joltage = Joltage::new();
    if let Ok(lines) = read_lines("query_input_day3.txt") {
        for line in lines.map_while(Result::ok) {
              let inp = line.trim();
	      println!("jolts are {:?}", &inp);
	      let jolts: Vec<u64> = inp.chars().map(|c| c.to_digit(10).map(u64::from)).collect::<Option<Vec<u64>>>().ok_or("Invalid").expect("problem");
	      println!("jolts are {:?}", &jolts);
	      let this_total = Joltage::choose(jolts);
	      println!("chosen jolts are {:?}", &this_total);
	      total_sum+=this_total;
	      }
	}
   println!("total sum is {:?}", &total_sum);
}
             