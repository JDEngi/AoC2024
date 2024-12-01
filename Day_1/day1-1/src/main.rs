use std::{env::current_dir, fs::read_to_string};
use anyhow::Result;


pub fn tokenize_input(input_str: &str) -> Result<(Vec<u32>, Vec<u32>)>
{
    let mut vec_a: Vec<u32> = vec![];
    let mut vec_b: Vec<u32> = vec![];

    for line in input_str.lines() {
        let tokens = line.split_ascii_whitespace();
        let mut numbers = tokens.map(|s| s.parse::<u32>().unwrap());

        let (val_a, val_b) = (
            numbers.next().unwrap(),
            numbers.next().unwrap(),
        );

        vec_a.push(val_a);
        vec_b.push(val_b);
    }

    Ok((vec_a, vec_b))
}

#[cfg(test)]
mod tests{
    // Use same includes as our main project
    use super::*;

    #[test]
    fn tokenize() {
        // Read the test file
        let test_data = read_to_string("../data/test1.txt").unwrap();
        let true_a: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let true_b: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        
        // Tokenize each line
        let result = tokenize_input(test_data.as_str());
        if let Ok((vals_a, vals_b)) = result {
            // Print list from two lists
            println!("Test input data is: ");
            for (a, b) in vals_a.iter().zip(vals_b.iter()) {
                println!("a: {}, b: {}", a, b);
            }

            assert_eq!(vals_a, true_a, "a values do not match");
            assert_eq!(vals_b, true_b, "b values do not match");
        }
    }
}

fn main() -> Result<()>{
    println!("Current directory is: {}", current_dir()?.display());
    let data = read_to_string("data/input1.txt")?;

    let (mut idx_left, mut idx_right) = tokenize_input(data.as_str())?;

    // Sort the values from small to large
    idx_left.sort();
    idx_right.sort();

    let acc = idx_left.iter().zip(idx_right.iter()).fold(0, |acc, (&a, &b)| acc + a.abs_diff(b));

    println!("The result is: {}", acc);

    Ok(())
}
