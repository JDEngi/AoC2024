// include the latest version of the regex crate in your Cargo.toml
extern crate regex;

use regex::Regex;
use std::fs;

fn main() {
  let regex = Regex::new(r"(?m)(?i)mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
  let string = fs::read_to_string("data/input1.txt").unwrap();
  
  // result will be an iterator over tuples containing the start and end indices for each match in the string
  let result = regex.captures_iter(&string);
  
  let mut accumulator = 0;
  for mat in result {
    println!("{:?}", mat);
    let a = mat["a"].parse::<i32>().unwrap();
    let b = mat["b"].parse::<i32>().unwrap();
    accumulator += a * b;
  }

  println!("Accumulated value is: {}", accumulator);
}
