use std::path::Path;
use std::io::{self, BufRead};
//use std::ops::{Add, Mul};
use std::fs::File;
use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Maths {
    value1: u64,
    value2: u64,
    value3: u64,
//add for query input
    value4: u64,
    sign: char,
    
}
impl Maths {
   fn add(self) -> u64 {
      let result = self.value1 + self.value2 + self.value3 + self.value4;
      result
      }
   fn mul(self) -> u64 {
      let result = self.value1 * self.value2 * self.value3 * self.value4;
      result
      }
}


//read lines function is from the Rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut full_input: Vec<Vec<_>> = Vec::new();
    let mut full_maths: Vec<Maths> = Vec::new();
    let mut dim_x = 0;
    let mut total_answer = 0;
    if let Ok(lines) = read_lines("query_input_day6.txt") {
        for line in lines.map_while(Result::ok) {
	     let input_vec: Vec<_> = line.trim().split_whitespace().map(|x| x.to_string()).collect();
	     dim_x = input_vec.len();
	     full_input.push(input_vec);
	     }
	}
    for i in 0..1  {
        println!("is full input i {:?}", &full_input[i]);
        for j in 0..dim_x {
	    let part1 = full_input[i][j].parse::<u64>().unwrap();
	    println!("part1 is {:?}", &part1);
	    let part2 = full_input[i+1][j].parse::<u64>().unwrap();
	    println!("part2 is {:?}", &part2);
	    let part3 = full_input[i+2][j].parse::<u64>().unwrap();
	    println!("part3 is {:?}", &part3);
	    let part4 = full_input[i+3][j].parse::<u64>().unwrap();
	    println!("part4 is {:?}", &part4);
	    let part5: Vec<_> = full_input[i+4][j].chars().collect();
	    println!("part5 is {:?}", &part5);
            let maths_set = Maths { value1: part1, value2: part2, value3: part3, value4: part4, sign: part5[0] };
	    println!("this is a maths set {:?}", &maths_set);
	    full_maths.push(maths_set);
	    }
	}
    for mat in full_maths {
        let answer = match mat.sign {
	                  '+' => { mat.add() },
			  '*' => { mat.mul() },
			  _ => { 0 },
			  };
	total_answer+=answer;
	}
    println!("so total answer is {:?}", &total_answer);
 }