use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs;

fn main() {

    let input_file = "input.txt";
    let (rules, pages) = load_input(input_file);
    let comparator = RuleComparator::new(rules);

    let result = sum_middle_nums_for_sorted(&pages, &comparator);
    println!("Sum of middle numbers: {}", result);
    
    let result2 = sum_middle_nums_for_unsorted(&pages, &comparator);
    println!("Sum of middle numbers for unsorted: {}", result2);
}

struct RuleComparator{
    rules: BTreeSet<(usize, usize)>
}

impl RuleComparator {
    fn new(rules: BTreeSet<(usize, usize)>) -> Self {
        RuleComparator { rules }
    }

    fn compare(&self, left: usize, right: usize) -> core::cmp::Ordering {
        if self.rules.contains(&(left, right)){
            Ordering::Less
        }
        else if self.rules.contains(&(right, left)){
            Ordering::Greater
        }
        else {
            Ordering::Equal
        }
    }
}


fn load_input(input_file: &str) -> (BTreeSet<(usize, usize)>, Vec<Vec<usize>>) {
    parse_input(&fs::read_to_string(input_file).expect("Failed to read input file"))
}

fn parse_input(input: &str) -> (BTreeSet<(usize, usize)>, Vec<Vec<usize>>) {
    let mut rules = BTreeSet::new();
    let mut pages = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            let mut parts = line.split('|');
            let left = parts.next().unwrap().parse().unwrap();
            let right = parts.next().unwrap().parse().unwrap();
            rules.insert((left, right));
        } else if line.contains(',') {
            let group = line.split(',')
                .map(|num| num.parse().unwrap())
                .collect();
            pages.push(group);
        }
    }

    (rules, pages)

}

fn is_in_order(input: &Vec<usize>, comparator: &RuleComparator) -> bool {
    input.windows(2).all(|pair| comparator.compare(pair[0], pair[1]) != Ordering::Greater)
}

fn sum_middle_nums_for_sorted(input: &Vec<Vec<usize>>, rules: &RuleComparator) -> usize {
    input.iter()
        .filter(|pages| is_in_order(pages, rules))
        .map(|group| group[group.len() / 2])
        .sum()
}

fn sum_middle_nums_for_unsorted(input: &Vec<Vec<usize>>, rules: &RuleComparator) -> usize {
    input.iter()
        .filter(|pages| !is_in_order(pages, rules))
        // sort the group and get the middle number
        .map(|group| {
            let mut sorted = group.clone();
            sorted.sort_by(|a, b| rules.compare(*a, *b));
            sorted[sorted.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_rules() {
        let input = "75|47\n97|13\n97|61";
        let expected = BTreeSet::from_iter(vec![(75, 47), (97, 13), (97, 61)]);
        let (rules,_) = parse_input(input);
        assert_eq!(rules, expected);
    }

    #[test]
    fn test_parse_groups() {
        let input = "97,61,53,29,13\n75,29,13";
        let expected = vec![
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
        ];
        let (_,pages) = parse_input(input);
        assert_eq!(pages, expected);
    }
    
    #[test]
    fn test_load_input(){
        let input_file = "test_input.txt";
        let (rules, pages) = load_input(input_file);
        assert_eq!(rules.len(), 21);
        assert_eq!(pages.len(), 6);
    }

    #[test]
    fn test_sort_by_rules(){
        let mut input = vec![75,47,61,53,29];
        let expected = input.clone();
        let input_file = "test_input.txt";
        let (rules, _) = load_input(input_file);
        let comparator = RuleComparator::new(rules);
        input.sort_by(|a, b| comparator.compare(*a, *b));
        
        assert_eq!(input, expected);
    }

    #[test]
    fn test_is_in_order(){
        let input = vec![75,47,61,53,29];
        let input_file = "test_input.txt";
        let (rules, _) = load_input(input_file);
        let comparator = RuleComparator::new(rules);
        let result = is_in_order(&input, &comparator);
        assert!(result);
        let input2 = vec![75,97,47,61,53];
        let result2 = is_in_order(&input2, &comparator);
        assert!(!result2);
        
    }

    #[test]
    fn test_middle_number_for_ordered(){
       
        let input_file = "test_input.txt";
        let (rules, pages) = load_input(input_file);
        let comparator = RuleComparator::new(rules);
        let expected = 143;
        let result = sum_middle_nums_for_sorted(&pages, &comparator);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_middle_number_for_unordered(){

        let input_file = "test_input.txt";
        let (rules, pages) = load_input(input_file);
        let comparator = RuleComparator::new(rules);
        let expected = 123;
        let result = sum_middle_nums_for_unsorted(&pages, &comparator);
        assert_eq!(result, expected);
    }

    
}