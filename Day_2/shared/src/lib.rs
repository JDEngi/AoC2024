
pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(
        |s| s.split_ascii_whitespace().map(
            |t| t.parse::<i32>().unwrap()
        ).collect()
    ).collect()
}

pub fn safe_increase(a: i32, b: i32) -> bool {
    match b - a {
        1..=3 => true,
        _ => false
    }
}

pub fn safe_decrease(a: i32, b: i32) -> bool {
    match a - b {
        1..=3 => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use std::env::current_dir;

    #[test]
    fn check_current_dir() {
        println!("This test module is runnin in: {}", current_dir().unwrap().display());
    }

    #[test]
    fn test_tokenization() {
        let data = read_to_string("../data/test1.txt").unwrap();
        let test_data = vec![
            vec![7, 6, 4 ,2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]
        ];

        let data = parse_input(data.as_str());

        assert_eq!(data, test_data, "parsed data does not match");
    }

    #[test]
    fn test_safe_increase() {
        assert!(safe_increase(1, 2));   // Minumum increase
        assert!(safe_increase(1, 4));   // Maximum increase
        assert!(!safe_increase(1, 5));  // Increase too large
        assert!(!safe_increase(3, 3));  // Increase too small
        assert!(!safe_increase(5, 3));  // Not an increase
    }

    #[test]
    fn test_safe_decrease() {
        assert!(safe_decrease(2, 1));   // Minumum decrease
        assert!(safe_decrease(4, 1));   // Maximum decrease
        assert!(!safe_decrease(5, 1));  // Decrease too large
        assert!(!safe_decrease(3, 3));  // Decrease too small
        assert!(!safe_decrease(3, 5));  // Not an decrease
    }
}
