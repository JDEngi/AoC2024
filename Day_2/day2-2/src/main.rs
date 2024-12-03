use shared::{parse_input, safe_decrease, safe_increase};
use std::env::current_dir;
use std::fs::read_to_string;

fn is_safe(input: &Vec<i32>, pred: fn(i32, i32)->bool) -> bool {
    input.windows(2).all(|x| pred(x[0], x[1]))
}

fn brute_force(input: &Vec<i32>) -> bool {

    for idx in 0..input.len() {
        let mut input_clone = input.clone();
        input_clone.remove(idx);

        // Check which operation we need to use to check
        let check_fn = match input_clone.last() > input_clone.first() {
            true => safe_increase,
            false => safe_decrease
        };

        if is_safe(&input_clone, check_fn) { return true }
    }

    false
}

// Check if the report is safe or if it would be safe if bruteforced
fn check_safe(input: &Vec<i32>) -> bool {
    // Check which operation we need to use to check
    let check_fn = match input.last() > input.first() {
        true => safe_increase,
        false => safe_decrease
    };
    
    // Check if the report is safe right from the start
    if is_safe(input, check_fn) { return true }
    // Otherwise, check if it's safe if we bruteforce it
    else { return brute_force(input)}
}


#[cfg(test)]
mod tests {
    // allow use of functions in this script
    use super::*;

    #[test]
    fn test_sharp_increase()
    {
        let test_case = vec![1, 2, 10, 11, 12];

        assert!(!check_safe(&test_case));
    }

    #[test]
    fn test_second_position_error() {
        let test_case = vec![53, 54, 54, 55, 56, 57];

        assert!(check_safe(&test_case))
    }
    
    #[test]
    fn test_first_position_error() {
        let test_case = vec![54, 53, 54, 55, 56, 57];
        assert!(check_safe(&test_case))
    }
}

fn main() {
    println!("Starting in directory: {}", current_dir().unwrap().display());
    
    let data = read_to_string("data/input1.txt").unwrap();
    let data = parse_input(data.as_str());

    let safe_count = data.iter().filter(|&report| check_safe(report)).count();

    println!("There are: {} safe reports with the Problem Dampener engaged", safe_count);
}
