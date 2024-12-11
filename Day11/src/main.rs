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
    std::fs::read_to_string(p0)
        .expect("Failed to read file")
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn blink_count(stones: Vec<usize>, blinks: usize) -> usize {
    let mut memo = HashMap::new();
    stones.iter().map(|&stone| transform_count(stone, blinks, &mut memo)).sum()
}

fn blink(stones: Vec<usize>, blinks: usize) -> Vec<usize> {
    let mut memo = HashMap::new();
    let mut current_stones = stones;

    for _ in 0..blinks {
        current_stones = current_stones
            .into_iter()
            .flat_map(|stone| transform(stone, &mut memo))
            .collect();
    }

    current_stones
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

    let result = process_transform(stone)
        .into_iter()
        .map(|next_stone| transform_count(next_stone, remaining_blinks - 1, memory))
        .sum();

    memory.insert((stone, remaining_blinks), result);
    result
}

fn transform(
    stone: usize,
    memory: &mut HashMap<(usize, usize), Vec<usize>>,
) -> Vec<usize> {
    if let Some(cached) = memory.get(&(stone, 1)) {
        return cached.clone();
    }

    let result = process_transform(stone);
    memory.insert((stone, 1), result.clone());
    result
}

fn process_transform(stone: usize) -> Vec<usize> {
    if stone == 0 {
        vec![1]
    } else if stone.to_string().len() % 2 == 0 {
        let digits = stone.to_string();
        let mid = digits.len() / 2;
        let left: usize = digits[..mid].parse().unwrap();
        let right: usize = digits[mid..].parse().unwrap();
        vec![left, right]
    } else {
        vec![stone * 2024]
    }
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
        let input = 253000;
        let output = transform(input, &mut HashMap::new());
        let expected = vec![253, 0];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_1_to_3_blinks() {
        let input = vec![125, 17];
        let output = blink(input, 3);
        let expected = vec![512072, 1, 20, 24, 28676032];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_test_input_to_25_blinks_with_count() {
        let input = load_input("test_input.txt");
        let output = blink_count(input, 25);
        assert_eq!(output, 55312);
    }
}
