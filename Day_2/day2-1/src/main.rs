use shared::{parse_input, safe_increase, safe_decrease};
use std::env::current_dir;
use std::fs::read_to_string;

fn select_fn(first: i32, last: i32) -> fn(i32, i32) -> bool {
    if first > last {safe_decrease}
    else {safe_increase} 
}

fn main() {
    println!("Starting in directory: {}", current_dir().unwrap().display());
    
    let data = read_to_string("data/input1.txt").unwrap();
    let data = parse_input(data.as_str());

    let safe_count = data.iter().filter(
        |report| match report.first() > report.last() {
            true => report.windows(2).all(|x| safe_decrease(x[0], x[1])),
            false => report.windows(2).all(|x| safe_increase(x[0], x[1]))
        }
    ).count();

    println!("There are: {} safe reports", safe_count)
    
}
