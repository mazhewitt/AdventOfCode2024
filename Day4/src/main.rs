use std::fs::File;
use std::io;
use std::io::BufRead;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let input_file = "input.txt";
    let input_grid = read_file_to_2d_vec(input_file).unwrap();
    let result = find_all_xamases(&input_grid);
    println!("Number of XMASes found: {}", result);
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const SAMX: [char; 4] = ['S', 'A', 'M', 'X'];


#[derive(Debug, PartialEq, EnumIter)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
    DiagonalTopLeftToBottomRight,
    DiagonalTopRightToBottomLeft,
    DiagonalBottomLeftToTopRight,
    DiagonalBottomRightToTopLeft,
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



pub fn find_xmas(grid: &Vec<Vec<char>>, direction: Direction) -> usize {

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    match direction {
        Direction::LeftToRight => {
            count = find_left_to_right(grid, rows, cols, &XMAS, 4);
        }
        Direction::RightToLeft => {
            count = find_left_to_right(grid, rows, cols, &SAMX, 4);
        }
        Direction::TopToBottom => {
            count = find_top_to_bottom(grid, rows, cols, &XMAS, 4);
        }
        Direction::BottomToTop => {
            count = find_top_to_bottom(grid, rows, cols, &SAMX, 4);
        }
        Direction::DiagonalTopLeftToBottomRight => {
            count = find_diagonal_top_left_to_bottom_right(grid, rows, cols, &XMAS, 4);
        }
        Direction::DiagonalTopRightToBottomLeft => {
            count = find_diagonal_top_right_to_bottom_left(grid, rows, cols, &XMAS, 4);
        }
        Direction::DiagonalBottomLeftToTopRight => {
            count = find_diagonal_top_right_to_bottom_left(grid, rows, cols, &SAMX, 4);
        }
        Direction::DiagonalBottomRightToTopLeft => {
            count = find_diagonal_top_left_to_bottom_right(grid, rows, cols, &SAMX, 4);
        }
    }

    count
}

fn find_left_to_right(grid: &Vec<Vec<char>>, rows: usize, cols: usize, word_chars: &[char], word_len: usize) -> usize {
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

fn find_top_to_bottom(grid: &Vec<Vec<char>>, rows: usize, cols: usize, word_chars: &[char], word_len: usize) -> usize {
    let mut count = 0;
    for col in 0..cols {
        for row in 0..=rows - word_len {
            if (0..word_len).all(|i| grid[row + i][col] == word_chars[i]) {
                count += 1;
            }
        }
    }
    count
}

fn find_diagonal_top_left_to_bottom_right(
    grid: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    word_chars: &[char],
    word_len: usize,
) -> usize {
    let mut count = 0;
    for row in 0..=rows - word_len {
        for col in 0..=cols - word_len {
            if (0..word_len).all(|i| grid[row + i][col + i] == word_chars[i]) {
                count += 1;
            }
        }
    }
    count
}

fn find_diagonal_top_right_to_bottom_left(
    grid: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    word_chars: &[char],
    word_len: usize,
) -> usize {
    let mut count = 0;
    for row in 0..=rows - word_len {
        for col in (word_len - 1..cols).rev() {
            if (0..word_len).all(|i| grid[row + i][col - i] == word_chars[i]) {
                count += 1;
            }
        }
    }
    count
}

fn find_all_xamases(input_grid: &Vec<Vec<char>>) -> usize {
    Direction::iter().map(|direction| find_xmas(input_grid, direction)).sum()
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

        let result = find_xmas(&grid, Direction::LeftToRight);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_find_xmas_right_to_left() {
        let grid = vec![
            vec!['A', 'B', 'C', 'D'],
            vec!['S', 'A', 'M', 'X'],
            vec!['W', 'X', 'M', 'A'],
            vec!['M', 'A', 'S', 'X'],
        ];

        let result = find_xmas(&grid, Direction::RightToLeft);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_xmas_top_to_bottom() {
        let grid = vec![
            vec!['X', 'A', 'M', 'S'],
            vec!['M', 'B', 'C', 'D'],
            vec!['A', 'E', 'F', 'G'],
            vec!['S', 'H', 'I', 'J'],
        ];

        let result = find_xmas(&grid, Direction::TopToBottom);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_xmas_bottom_to_top() {
        let grid = vec![
            vec!['S', 'A', 'M', 'S'],
            vec!['A', 'B', 'C', 'D'],
            vec!['M', 'E', 'F', 'G'],
            vec!['X', 'H', 'I', 'J'],
        ];

        let result = find_xmas(&grid, Direction::BottomToTop);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_xmas_diagonal_top_left_to_bottom_right() {
        let grid = vec![
            vec!['X', 'A', 'B', 'C'],
            vec!['D', 'M', 'E', 'F'],
            vec!['G', 'H', 'A', 'I'],
            vec!['J', 'K', 'L', 'S'],
        ];

        let result = find_xmas(&grid, Direction::DiagonalTopLeftToBottomRight);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_xmas_diagonal_top_right_to_bottom_left() {
        let grid = vec![
            vec!['A', 'B', 'C', 'X'],
            vec!['D', 'M', 'M', 'F'],
            vec!['G', 'A', 'A', 'I'],
            vec!['S', 'K', 'L', 'S'],
        ];

        let result = find_xmas(&grid, Direction::DiagonalTopRightToBottomLeft);

        assert_eq!(result, 1);
    }
    #[test]
    fn test_find_xmas_diagonal_bottom_left_to_top_right() {
        let grid = vec![
            vec!['A', 'B', 'C', 'S'],
            vec!['D', 'M', 'A', 'F'],
            vec!['G', 'M', 'A', 'I'],
            vec!['X', 'K', 'L', 'S'],
        ];

        let result = find_xmas(&grid, Direction::DiagonalBottomLeftToTopRight);

        assert_eq!(result, 1);
    }
    #[test]
    fn test_find_xmas_diagonal_bottom_right_to_top_left() {
        let grid = vec![
            vec!['S', 'B', 'C', 'S'],
            vec!['D', 'A', 'A', 'F'],
            vec!['G', 'M', 'M', 'I'],
            vec!['M', 'K', 'L', 'X'],
        ];

        let result = find_xmas(&grid, Direction::DiagonalBottomRightToTopLeft);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_all_xmases(){
        let input_file = "test_input.txt";
        let input_grid = read_file_to_2d_vec(input_file).unwrap();
        let expected = 18;
        let result = find_all_xamases(&input_grid);
        assert_eq!(result, expected);
    }


}


