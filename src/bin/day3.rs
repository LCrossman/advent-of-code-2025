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
impl Joltage {
    fn choose(jolts: Vec<u64>) -> u64 {
       let vec1 = jolts;
       let mut maxnum = 0;
       let all = vec1.into_iter().combinations(2);
       let mut jolt1 = 0;
       let mut jolt2 = 0;
       for a in all {
          jolt1 = a[0];
	  jolt2 = a[1];
	  let num = (jolt1 * 10) + jolt2;
	  if num > maxnum {
	     maxnum = num;
	     }
	  }
       maxnum
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
             