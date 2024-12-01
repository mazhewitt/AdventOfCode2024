use std::fs;
use regex::Regex;

fn main() {
    let (left, right) = load_input("input.txt");
    let total_distance = calculate_total_distance(left, right);
    println!("Total distance: {}", total_distance);
}


pub fn calculate_total_distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    //sort the left and right arrays
    let mut left_sorted = left;
    let mut right_sorted = right;

    left_sorted.sort();
    right_sorted.sort();

    assert_eq!(left_sorted.len(), right_sorted.len());

    left_sorted.iter().zip(right_sorted.iter()).map(|(l, r)| {
        (l - r).abs()
    }).sum()
}

fn load_input(input_file: &str) -> (Vec<i32>, Vec<i32>) {
    let content = fs::read_to_string(input_file).expect("Failed to read input file");
    let re = Regex::new(r"(\d+)\s+(\d+)").expect("Invalid regex");

    let (left, right): (Vec<_>, Vec<_>) = content
        .lines()
        .filter_map(|line| re.captures(line))
        .map(|cap| {
            let left = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let right = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            (left, right)
        })
        .unzip();

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_distance() {
        let left = vec![3,4,2,1,3,3];
        let right = vec![4,3,5,3,9,3];

        let expected = 11;
        assert_eq!(expected, calculate_total_distance(left, right));
    }

    #[test]
    fn test_load_input() {
        let test_file = "test_input.txt";
        let (left, right) = load_input(test_file);
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }


}