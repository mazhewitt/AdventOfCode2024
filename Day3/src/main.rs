use regex::Regex;

fn main() {
    
    let muls_str = load_input("input.txt");
    let muls = extract_mul(&muls_str);
    let result = mul(&muls);
    println!("Part 1 Result: {}", result);
    let do_muls = extract_do_muls(&muls_str);
    let result2 = mul(&do_muls);
    println!("Part 2 Result: {}", result2);
}

fn extract_do_muls(mul_input: &str) -> Vec<(i32, i32)> {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let reg_ex = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let mut should_mul = true;
    reg_ex.captures_iter(mul_input)
        .filter_map(|cap| {
            let matched = cap.get(1).unwrap().as_str();
            match matched {
                "do()" => {
                    should_mul = true;
                    None
                },
                "don't()" => {
                    should_mul = false;
                    None
                },
                mul_str => {
                    if should_mul {
                        // Use mul_regex to extract numbers from mul(x,y)
                        if let Some(nums) = mul_regex.captures(mul_str) {
                            let a = nums.get(1).unwrap().as_str().parse::<i32>().unwrap();
                            let b = nums.get(2).unwrap().as_str().parse::<i32>().unwrap();
                            Some((a, b))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }
        })
        .collect()
}

fn load_input(input_file: &str) -> String{
    std::fs::read_to_string(input_file).expect("Failed to read input file").to_string()
}

fn mul(muls: &Vec<(i32, i32)>) -> i32 {
    muls.iter().fold(0, |acc, (a, b)| acc + (a * b))
}

fn extract_mul(input_mul: &str) -> Vec<(i32, i32)> {
    let reg_ex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result: Vec<(i32, i32)> = reg_ex.captures_iter(input_mul)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            (a, b)
        }).collect();
    result
}


#[cfg(test)]
mod tests {
   
    use super::*;

    static INPUT:&str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    
    #[test]
    fn test_mul_extraction() {
        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let result = extract_mul(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_multiplication() {
        let expected = 161;
        let mul_input = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let result = mul(&mul_input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_load_do_muls() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = vec![(2,4),(8,5)];
        let result = extract_do_muls(input);
        assert_eq!(expected, result);
    }

 
}