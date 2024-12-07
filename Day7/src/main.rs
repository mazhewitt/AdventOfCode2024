use std::collections::VecDeque;
use std::vec;

fn main() {
    let input_file = "input.txt";
    let inputs = load_input(input_file);
    let total:usize = inputs.iter()
        .filter(|(array, target)| can_reach_target(array.clone(), *target, false))
        .map(|(_, target)|target).sum();
    println!("Total Part1: {}", total);
    let total:usize = inputs.iter()
        .filter(|(array, target)| can_reach_target(array.clone(), *target, true))
        .map(|(_, target)|target).sum();
    println!("Total Part2: {}", total);
}

fn load_input(p0: &str) -> Vec<(Vec<usize>,usize)> {
    let mut inputs = vec::Vec::new();
    let file = std::fs::read_to_string(p0).unwrap();
    // line looks like "292: 11 6 16 20"
    for line in file.lines() {
        let mut parts = line.split(": ");
        let target = parts.next().unwrap().parse().unwrap();
        let array = parts.next().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();
        inputs.push((array, target));
    }
    inputs
}

pub fn can_reach_target(array: Vec<usize>, target: usize, with_concat:bool) -> bool {
    if array.is_empty() {
        return false;
    }
    let mut queue = VecDeque::new();
    queue.push_back((array[0], 1));

    while let Some((current_value, index)) = queue.pop_front() {
        if current_value == target {
            return true;
        }
        if index == array.len() {
            continue;
        }
        let next_number = array[index];
        queue.push_back((current_value + next_number, index + 1));
        queue.push_back((current_value * next_number, index + 1));
        if with_concat {
            queue.push_back(((current_value.to_string() + &next_number.to_string()).parse().unwrap(), index + 1));
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_operator() {
        let array = vec![10, 19];
        let target = 190;
        assert!(can_reach_target(array, target, false));
    }

    #[test]
    fn test_multiple_operators() {
        let array = vec![81, 40, 27];
        let target = 3267;
        assert!(can_reach_target(array, target, false));
    }

    #[test]
    fn test_unreachable_target() {
        let array = vec![10, 5];
        let target = 100;
        assert!(!can_reach_target(array, target, false));
    }

    #[test]
    fn test_load_input() {
        let input_file = "test_input.txt";
        let inputs = load_input(input_file);
        assert_eq!(inputs.len(), 9);
    }
    

    #[test]
    fn test_sum_valid() {
        let input_file = "test_input.txt";
        let inputs = load_input(input_file);
        let total:usize = inputs.iter()
            .filter(|(array, target)| can_reach_target(array.clone(), *target, false))
            .map(|(_, target)|target).sum();
        assert_eq!(total, 3749);
        let total:usize = inputs.iter()
            .filter(|(array, target)| can_reach_target(array.clone(), *target, true))
            .map(|(_, target)|target).sum();
        assert_eq!(total, 11387);
    }

}