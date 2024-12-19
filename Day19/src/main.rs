use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start_time = Instant::now(); // Start the timer
    let filename = "input.txt";
    let (towel_patterns, designs) = load_input(filename);
    let results = can_make_designs(&towel_patterns, &designs);
    let num_results = results.iter().filter(|x| **x).count();
    println!("Number of designs that can be made: {}", num_results);
    
    let num_ways = num_ways_to_make_designs(&towel_patterns, &designs);
    println!("Number of ways to make the designs: {}", num_ways);
    let duration = start_time.elapsed(); // Calculate elapsed time
    println!("Time taken: {:.2?}", duration);
}

fn load_input(p0: &str) -> (Vec<String>, Vec<String>) {
    let file = File::open(p0).expect("file not found");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let towel_patterns: Vec<String> = lines[0]
        .split(',')
        .map(|x| x.trim().to_string())
        .collect();

    let designs: Vec<String> = lines.into_iter().skip(2).collect();

    (towel_patterns, designs)
}


fn can_make_designs(towel_patterns: &[String], designs: &[String]) -> Vec<bool> {
    let towel_set: HashSet<&str> = towel_patterns.iter().map(|x| x.as_str()).collect();
    let mut results = Vec::new();

    for design in designs {
        let n = design.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true; // Base case: empty string can always be formed

        for i in 1..=n {
            for j in 0..i {
                if dp[j] && towel_set.contains(&design[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        results.push(dp[n]);
    }

    results
}


fn num_ways_to_make_designs(towel_patterns: &Vec<String>, designs: &Vec<String>) -> usize {
    let towel_set: HashSet<&str> = towel_patterns.iter().map(|x| x.as_str()).collect();

    let mut total_ways = 0;
    for design in designs {
        let n = design.len();
        // dp[i] will represent the number of ways to form design[0..i]
        let mut dp = vec![0usize; n + 1];
        dp[0] = 1; // There is exactly one way to form an empty substring

        for i in 1..=n {
            for j in 0..i {
                let substring = &design[j..i];
                if towel_set.contains(substring) {
                    dp[i] += dp[j];
                }
            }
        }
        total_ways += dp[n];
    }
    total_ways
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_input() {
       let filename = "test_input.txt";
       let (towel_patterns, designs) = load_input(filename);
       assert_eq!(towel_patterns, vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
       assert_eq!(designs, vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"]);
    }

    #[test]
    fn test_can_make_designs() {
        let filename = "test_input.txt";
        let (towel_patterns, designs) = load_input(filename);
        let results = can_make_designs(&towel_patterns, &designs);
        let num_results = results.iter().filter(|x| **x).count();
        assert_eq!(num_results, 6);
    }
    
   #[test]
   fn test_ways_to_make_designs() {
       let filename = "test_input.txt";
       let (towel_patterns, _) = load_input(filename);
       let designs = vec!["gbbr".to_string()];
       let num_ways = num_ways_to_make_designs(&towel_patterns, &designs);
       assert_eq!(num_ways, 4);
   }

    #[test]
    fn test_ways_to_make_designs2() {
        let filename = "test_input.txt";
        let (towel_patterns, _) = load_input(filename);
        let designs = vec!["rrbgbr".to_string()];
        let num_ways = num_ways_to_make_designs(&towel_patterns, &designs);
        assert_eq!(num_ways, 6);
    }

    #[test]
    fn test_ways_to_make_designs3() {
        let filename = "test_input.txt";
        let (towel_patterns, _) = load_input(filename);
        let designs = vec!["brwrr".to_string()];
        let num_ways = num_ways_to_make_designs(&towel_patterns, &designs);
        assert_eq!(num_ways, 2);
    }
    
    #[test]
    fn test_ways_to_make_designs4() {
        let filename = "test_input.txt";
        let (towel_patterns, _) = load_input(filename);
        let designs = vec!["bbrgwb".to_string()];
        let num_ways = num_ways_to_make_designs(&towel_patterns, &designs);
        assert_eq!(num_ways, 0);
    }
    
    #[test]
    fn test_total_ways_to_make_designs() {
        let filename = "test_input.txt";
        let (towel_patterns, designs) = load_input(filename);
        let num_ways = num_ways_to_make_designs(&towel_patterns, &designs);
        assert_eq!(num_ways, 16);
    }
    
}