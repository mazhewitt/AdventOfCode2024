use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input_line = fs::read_to_string("input.txt").unwrap();
    let input = input_line.trim();
    let file_blocks = expand_file_map(&input);
    let compacted_blocks = compact_single_file_blocks(&file_blocks);
    let checksum = compute_checksum(&compacted_blocks);
    println!("The checksum is: {}", checksum);
    let compacted_whole_blocks = compact_whole_files(&file_blocks);
    let checksum_whole = compute_checksum(&compacted_whole_blocks);
    println!(
        "The checksum for compacting whole files is: {}",
        checksum_whole
    );
}

fn expand_file_map(input: &str) -> Vec<i32> {
    let mut file_id = 0;
    input
        .chars()
        .flat_map(|char| {
            let num = char.to_digit(10).unwrap() as usize;
            if file_id % 2 == 0 {
                let result = (0..num).map(|_| file_id / 2).collect::<Vec<_>>();
                file_id += 1;
                result
            } else {
                let result = (0..num).map(|_| -1).collect::<Vec<_>>();
                file_id += 1;
                result
            }
        })
        .collect()
}

fn compact_single_file_blocks(input_file_blocks: &[i32]) -> Vec<i32> {
    let mut output = input_file_blocks.to_vec();
    // Build a map of all the empty blocks, sorted from start to end
    let mut empty_blocks = input_file_blocks
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x == -1)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    // need to reverse it as we are going to pop from the end
    empty_blocks.reverse();

    for i in (0..input_file_blocks.len()).rev() {
        if input_file_blocks[i] != -1 {
            if let Some(empty_block) = empty_blocks.pop() {
                if empty_block < i {
                    output[empty_block] = output[i];
                    output[i] = -1;
                } else {
                    empty_blocks.push(empty_block);
                }
            } else {
                break;
            }
        }
    }

    output
}

fn compact_whole_files(input_file_blocks: &[i32]) -> Vec<i32> {
    let mut output = input_file_blocks.to_vec();
    let mut free_spaces = find_free_space(input_file_blocks);
    let mut file_id = -1;
    for i in (0..input_file_blocks.len()).rev() {
        if output[i] != -1 {
            if file_id == output[i] {
                continue;
            }
            file_id = output[i];
            let mut file_start = i;
            let mut file_length = 1;

            while file_start > 0 && output[file_start - 1] == file_id {
                file_start -= 1;
                file_length += 1;
            }
            let mut size_of_space_to_find = file_length;
            let mut moved = false;
            'moving: while !moved {
                // find the left most space for the file
                if let Some(free_space) = free_spaces.get_mut(&size_of_space_to_find) {
                    free_space.sort_by_key(|&x| Reverse(x));
                    let left_most_space_o = free_space.pop();
                    if left_most_space_o.is_none() {
                        size_of_space_to_find += 1;
                        continue 'moving;
                    }
                    let left_most_space = left_most_space_o.unwrap();
                    if left_most_space > file_start {
                        break 'moving;
                    }

                    for j in 0..file_length {
                        output[left_most_space + j] = file_id;
                    }
                    // make original space available
                    for k in 0..file_length {
                        output[file_start + k] = -1;
                    }
                    if free_space.is_empty() {
                        free_spaces.remove(&size_of_space_to_find);
                    }
                    if size_of_space_to_find > file_length {
                        let new_space_idx = left_most_space + file_length;
                        let new_space_size = size_of_space_to_find - file_length;
                        let entries = free_spaces.entry(new_space_size).or_insert(Vec::new());
                        entries.push(new_space_idx);
                    }
                    moved = true;
                } else {
                    size_of_space_to_find += 1;
                    if size_of_space_to_find > 9 {
                        break 'moving;
                    }
                }
            }
        }
    }

    output
}
fn find_free_space(input_file_blocks: &[i32]) -> HashMap<usize, Vec<usize>> {
    let mut free_spaces = HashMap::new();
    let mut current_free_start: Option<usize> = None;

    for (index, &block) in input_file_blocks.iter().enumerate() {
        match block {
            -1 => {
                if current_free_start.is_none() {
                    current_free_start = Some(index);
                }
            }
            _ => {
                if let Some(start) = current_free_start.take() {
                    let length = index - start;
                    let entries = free_spaces.entry(length).or_insert(Vec::new());
                    entries.push(start);
                }
            }
        }
    }
    free_spaces
}

fn compute_checksum(blocks: &[i32]) -> i64 {
    let mut checksum = 0;
    for (i, &f) in blocks.iter().enumerate() {
        if f != -1 {
            checksum += i as i64 * f as i64;
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_file_map() {
        let input = "12345";
        let result = expand_file_map(input);
        let expected = vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2];
        assert_eq!(result, expected);

        let long_input = "2333133121414131402";
        let long_result = expand_file_map(long_input);
        let long_expected = vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];
        assert_eq!(long_result, long_expected);
    }

    #[test]
    fn test_compact_single_file_blocks() {
        let input = vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2];
        let result = compact_single_file_blocks(&input);
        let expected = vec![0, 2, 2, 1, 1, 1, 2, 2, 2, -1, -1, -1, -1, -1, -1];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compute_checksum() {
        let input = vec![
            0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        ];
        let result = compute_checksum(&input);
        let expected = 1928;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compact_whole_files() {
        let input = vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];
        let result_disk = compact_whole_files(&input);
        println!("{:?}", result_disk);
        let result = compute_checksum(&result_disk);
        let expected = 2858;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_free_space() {
        let input = vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2];
        let result = find_free_space(&input);
        assert_eq!(*result.get(&4).unwrap().get(0).unwrap(), 6 as usize);
        assert_eq!(*result.get(&2).unwrap().get(0).unwrap(), 1 as usize);
        let input2 = vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];

        let free_spaces = find_free_space(&input2);
        assert!(free_spaces.get(&4).is_none());
        assert_eq!(free_spaces.get(&3).unwrap().len(), 3 as usize);
        assert_eq!(free_spaces.get(&1).unwrap().len(), 5 as usize);
        assert_eq!(free_spaces.len(), 2 as usize);
    }
}
