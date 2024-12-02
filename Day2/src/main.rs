use std::fs;
use regex::Regex;



fn main() {
    //load the input
    let input_file = "input.txt";
    let reports = load_input(input_file);
    let safe_reports = count_number_of_safe_reports(reports);
    println!("Number of safe reports: {}", safe_reports);
    
}


fn is_safe(report: &[i32]) -> bool {
    report.windows(2).all(|w| {
        let diff = w[1] - w[0];
        diff >= 1 && diff <= 3
    }) || report.windows(2).all(|w| {
        let diff = w[0] - w[1];
        diff >= 1 && diff <= 3
    })
}

fn count_number_of_safe_reports(reports: Vec<Vec<i32>>) -> i32 {
    reports.iter().filter(|report| is_safe(report)).count() as i32
}

fn load_input(input_file: &str) -> Vec<Vec<i32>> {
    let content = fs::read_to_string(input_file).expect("Failed to read input file");
   
    content
        .lines()
        .map(|line| {
            let re = Regex::new(r"\d+").unwrap();
            re.find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect()
        }).collect()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_safe(){
        let report = vec![1,2,3,4,5];
        let expected = true;
        assert_eq!(expected, is_safe(&report));
        let report2 = vec![1,2,3,3,4];
        let expected2 = false;
        assert_eq!(expected2, is_safe(&report2));
        let report3 = vec![5,4,3,2,1];
        let expected3 = true;
        assert_eq!(expected3, is_safe(&report3));
    }

    #[test]
    fn test_count_number_of_safe_reports(){
        let reports = vec![vec![1,2,3,4,5], vec![1,2,3,3,4], vec![5,4,3,2,1]];
        let expected = 2;
        assert_eq!(expected, count_number_of_safe_reports(reports));
        
        let input_reports = load_input("test_input.txt");
        let expected_safe_reports = 3;
        let safe_reports = count_number_of_safe_reports(input_reports);
        assert_eq!(expected_safe_reports, safe_reports);
    }

    #[test]
    fn test_load_input(){
        let test_file = "test_input.txt";
        let reports = load_input(test_file);
        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9], 
            vec![86, 81, 78, 75, 73, 76, 73, 67], 
            vec![1, 2, 5, 6, 9, 11, 13, 15]
            ];
        assert_eq!(reports, expected);
    }


}