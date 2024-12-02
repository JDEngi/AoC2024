use shared::{parse_input, safe_decrease, safe_increase};
use std::env::current_dir;
use std::fs::read_to_string;

fn is_safe_(input: &Vec<i32>) -> bool {
    let mut safe_increases = 0;
    let mut safe_decreases = 0;
    let mut unsafe_creases = 0;

    for x in input.windows(2){
        match x[1] - x[0] {
            -3..=-1 => safe_decreases += 1,
            1..=3 => safe_increases += 1,
            _ => unsafe_creases += 1
        }
    }

    println!("I{} D{} U{}", safe_increases, safe_decreases, unsafe_creases);
    if safe_increases > safe_decreases {
        (safe_decreases + unsafe_creases) <= 1
    } else {
        (safe_increases + unsafe_creases) <= 1
    }
}

fn is_safe(input: &Vec<i32>) -> bool {
    // Check which operation we need to use to check
    let check_fn = match input.last() > input.first() {
        true => safe_increase,
        false => safe_decrease
    };

    // Prime the loop
    let mut input_iter = input.iter();
    let mut a = input_iter.next().unwrap();
    let mut b = input_iter.next().unwrap();

    // Set fault condition to initial check
    // println!("First check is: {} and {}", a, b);
    let mut has_fault = !check_fn(*a, *b);
    a = b;
    if has_fault {
        b = input_iter.next().unwrap();
    }

    // println!("Starting with: {}", has_fault);

    for x in input_iter {
        if has_fault {
            if !check_fn(*a, *x){
                return false;
            } else {
                a = b;
                b = x;
            }

        } else {
            if !check_fn(*b, *x) {
                b = x;
                has_fault = true;
            } else {
                a = b;
                b = x;
            }
        }
    }

    println!("{:?} is safe", input);

    true
}

fn main() {
    println!("Starting in directory: {}", current_dir().unwrap().display());
    
    let data = read_to_string("data/test1.txt").unwrap();
    let data = parse_input(data.as_str());

    let safe_count = data.iter().filter(|&report| is_safe(report)).count();

    println!("There are: {} safe reports with the Problem Dampener engaged", safe_count);
}
