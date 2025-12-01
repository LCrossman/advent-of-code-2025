use std::ops::{Add, Sub};
use std::fs::File;
use std::str::FromStr;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone)]
struct Dial {
    value: i32,
    click: i32,
}
impl Dial {
   fn new() -> Self {
      Self {
        value: 50,
	click: 0,
        }
      }
}
impl Sub<i32> for Dial {
   type Output = Self;
   fn sub(mut self, left_instruction: i32) -> Self {
      let mut result = self.value - left_instruction;
      while result < 0 {
         result = result + 100;
	 if result == 0 {
	    self.click +=1;
	    }
	 }
      //println!("this is result {:?}", &result);
      if result == 0 {
         //println!("its a click left");
         return Self {
	   value: 0,
	   click: self.click + 1,
	   };
	 }
      else if result > 99 {
         println!("issue with left self");
	 }
      else {
        return Self {
	   value: result,
	   click: self.click,
	   };
      }
      self
      }
}
impl Add<i32> for Dial {
   type Output = Self;
   fn add(mut self, right_instruction: i32) -> Self {
      let mut result = self.value + right_instruction;
      while result > 100 {
         if result == 100 {
	     result = 0;
	     self.click += 1;
	     }
	 result = result - 100
	 }
      //println!("this is result {:?}", &result);
      if result == 0 || result == 100 {
         //println!("its a click right");
         return Self {
	   value: 0,
	   click: self.click + 1,
	   };
	 }
      else if result < 0 {
         println!("issue with right self");
	 }
      else {
        return Self {
	   value: result,
	   click: self.click,
	   };
      }
      self
      }
}

//read lines function is from the Rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut dial = Dial::new();
    if let Ok(lines) = read_lines("input_query.txt") {
    for line in lines.map_while(Result::ok) {
           let inp = line.trim();
           let ch = inp.chars().nth(0).unwrap();
           let test_value: i32 = FromStr::from_str(&inp[1..]).unwrap();
           if ch == 'L' {
               dial = dial - test_value;
	       }
           else if ch == 'R' {
               dial = dial + test_value;
	       }
	   //println!("dial is now {:?}", &dial);
           }
	}
    println!("Actual password is {}", &dial.click);
}