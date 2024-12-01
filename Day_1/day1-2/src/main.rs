use std::collections::HashMap;
use std::env::current_dir;
use std::fs::read_to_string;
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

fn main() -> Result<()> {
    println!("Current directory is: {}", current_dir()?.display());
    let data = read_to_string("data/input1.txt")?;

    let (mut idx_left, mut idx_right) = tokenize_input(data.as_str())?;

    // Sort the values from small to large
    idx_left.sort();
    idx_right.sort();

    // Make a map of all the multiplication values
    let mut right_map: HashMap<u32, u32> = HashMap::new();
    for value in idx_right {
        *right_map.entry(value).or_insert(0) += 1;
    }

    let acc: u32 = idx_left.iter().map(|&a| a * *right_map.entry(a).or_insert(0)).sum();

    println!("The result is: {}", acc);

    Ok(())
}
