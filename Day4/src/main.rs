use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    println!("Hello, world!");
}


fn read_file_to_2d_vec(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let grid: Vec<Vec<char>> = reader
        .lines()            
        .filter_map(|line| line.ok()) 
        .map(|line| line.chars().collect()) 
        .collect();
    Ok(grid)
}

pub fn find_xmas(grid: &Vec<Vec<char>>) -> usize {
    
    let xmas = "XMAS";
    
    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = xmas.chars().collect();
    let word_len = xmas.len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..=cols - word_len {
            if grid[row][col..col + word_len] == word_chars[..] {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_load_data() {
        // open the file and read the contents line by line into a 2d vec of char using the fs crate
        let input_file = "test_input.txt";
        let result = read_file_to_2d_vec(input_file).unwrap();
        assert_eq!(result.len(), 10); // Number of rows
        assert!(result.iter().all(|row| row.len() == 10)); // All rows have 10 columns
    }

    #[test]
    fn test_find_xmas_horizontally_left_to_right() {
        let grid = vec![
            vec!['A', 'B', 'C', 'D'],
            vec!['X', 'M', 'A', 'S'], // XMAS 
            vec!['W', 'X', 'M', 'A'],
            vec!['M', 'A', 'S', 'X'],
        ];

        let result = find_xmas(&grid);
        assert_eq!(result, 1);
    }
}


