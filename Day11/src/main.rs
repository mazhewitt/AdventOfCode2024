use std::collections::HashMap;

fn main() {
    let input_file = "input.txt";
    let input = load_input(input_file);
    let input2 = input.clone();
    let result = blink_count(input, 25);
    println!("The number of stones after 25 blinks is: {}", result);
    let result = blink_count(input2, 75);
    println!("The number of stones after 75 blinks is: {}", result);

}


fn load_input(p0: &str) -> Vec<usize> {
    // split the line into a vector of strings and then convert each string to a usize
    std::fs::read_to_string(p0)
        .expect("Failed to read file")
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}


fn blink_count(stones: Vec<usize>, blinks: usize) -> usize {
    let mut memo = HashMap::new();
    let mut count = 0;

    for &stone in &stones {
        count += transform_count(stone, blinks, &mut memo);
    }

    count
}

fn transform_count(
    stone: usize,
    remaining_blinks: usize,
    memory: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if remaining_blinks == 0 {
       return 1;
    }

    if let Some(&cached) = memory.get(&(stone, remaining_blinks)) {
        return cached;
    }

    let result = if stone == 0 {
        return transform_count(1,remaining_blinks-1,memory);
    } else if stone.to_string().len() % 2 == 0 {
        let digits = stone.to_string();
        let mid = digits.len() / 2;
        let left: usize = digits[..mid].parse().unwrap();
        let right: usize = digits[mid..].parse().unwrap();
        transform_count(left, remaining_blinks - 1, memory)
            + transform_count(right, remaining_blinks - 1, memory)
    } else {
        transform_count(stone * 2024, remaining_blinks - 1, memory)
    };

    memory.insert((stone, remaining_blinks), result);
    result
}

fn blink(stones: Vec<usize>, blinks:usize) -> Vec<usize> {
    let mut memo = HashMap::new();
    let mut current_stones = stones;

    for _ in 0..blinks {
        let mut next_stones = Vec::new();
        for &stone in &current_stones {
            next_stones.extend(transform(stone, 1, &mut memo));
        }
        current_stones = next_stones;
    }

    current_stones
}

fn transform(stone: usize, remaining_blinks: usize, memory: &mut HashMap<(usize, usize), Vec<usize>>) -> Vec<usize> {
    if remaining_blinks == 0 {
        return vec![stone];
    }
    if let Some(cached) = memory.get(&(stone, remaining_blinks)) {
        return cached.clone();
    }
    // if the stone is zero, then we return 1
    let result = if stone == 0 {
        vec![1]
    }
    //If the stone is engraved with a number that has an even number of digits, it is replaced by two stones
    else if stone.to_string().len() % 2 == 0 {
        let digits = stone.to_string();
        let mid = digits.len() / 2;
        let left: usize = digits[..mid].parse().unwrap();
        let right: usize = digits[mid..].parse().unwrap();
        [transform(left, remaining_blinks - 1, memory), transform(right, remaining_blinks - 1, memory)]
            .concat()
    } else {
        transform(stone * 2024, remaining_blinks - 1, memory)
    };

    memory.insert((stone, remaining_blinks), result.clone());
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_input() {
        let input = load_input("test_input.txt");
        assert_eq!(input.len(), 2);
    }

    #[test]
    fn test_transform() {
        let mut input = 253000;
        let output = transform(input, 1, &mut HashMap::new());
        let mut expected = vec![253, 0];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_1_to_3_blinks() {
        let input =vec![125, 17];
        let output = blink(input, 3);
        let expected = vec![512072, 1, 20, 24, 28676032];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_4_to_5_blinks() {
        let input =vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032];
        let output = blink(input, 1);
        let expected = vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_5_to_6_blinks() {
        let input =vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32,];
        let output = blink(input, 1);
        let expected = vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_4_to_6_blinks() {
        let input =vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032];
        let output = blink(input, 2);
        let expected = vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_blink() {
        let input =vec![125, 17];
        let output = blink(input, 1);
        assert_eq!(output.len(), 3);
        let expected = vec![253000, 1, 7];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_test_input_to_25_blinks() {
        let input = load_input("test_input.txt");
        let output = blink(input, 25);
        assert_eq!(output.len(), 55312);
    }
    #[test]
    fn test_test_input_to_25_blinks_with_count() {
        let input = load_input("test_input.txt");
        let output = blink_count(input, 25);
        assert_eq!(output, 55312);
    }
}
