use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use grid::*;

fn main() {
    let input_file = "input.txt";
    let grid = load_grid(input_file).expect("Failed to load grid");
    let position = find_guard_position(&grid).expect("No Guard Found");
    let mut guard = Guard::new(position, (-1, 0));
    guard.move_until_left(&grid);
    println!("Visited: {}", guard.visited.len());
    
}

struct Guard {
    position: (usize, usize),
    direction: (isize, isize),
    visited: HashSet<(usize, usize)>,
}

impl Guard {
    fn new(position: (usize, usize), direction: (isize, isize)) -> Guard {
        let mut visited = HashSet::new();
        visited.insert(position);
        Guard {
            position,
            direction,
            visited,
        }
    }

    fn move_guard(&mut self, grid: &Grid<char>) -> bool {  
        let (row, col) = self.position;
        let (d_row, d_col) = self.direction;
        let new_row = (row as isize + d_row) as usize;
        let new_col = (col as isize + d_col) as usize;
        if grid.get(new_row, new_col).is_none() {
            return false
        }
        if grid.get(new_row, new_col) == Some(&'#') {
            //turn right
            self.direction = (d_col, -d_row);
        } 
        else {
            self.position = (new_row, new_col);
            self.visited.insert(self.position);
        }
        true
    }
    
    fn move_until_left(&mut self, grid: &Grid<char>) {
        while self.move_guard(grid) {}
    }
    
}


fn load_grid(input_file: &str) -> Result<grid::Grid<char>, io::Error> {
    let mut grid = Grid::new(0, 0);

    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push_row(row);
    }
    Ok(grid)    
}

fn find_guard_position(grid: &Grid<char>) -> Option<(usize, usize)> {
    for (row_idx, mut row) in grid.iter_rows().enumerate() {
        if let Some(col_idx) = row.position(|&ch| ch == '^') {
            return Some((row_idx, col_idx));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_grid() {
        let input_file = "test_input.txt";
        let grid = load_grid(input_file).unwrap();
        assert_eq!(grid.size(), (10, 10));
    }
    
    #[test]
    fn test_find_caret_position() {
        let input_file = "test_input.txt";
        let grid = load_grid(input_file).expect("Failed to load grid");
        let position = find_guard_position(&grid).expect("No Guard Found");
        assert_eq!(position, (6, 4));
    }

    #[test]
    fn test_move_until_left() {
        let input_file = "test_input.txt";
        let grid = load_grid(input_file).expect("Failed to load grid");
        let position = find_guard_position(&grid).expect("No Guard Found");
        let mut guard = Guard::new(position, (-1, 0));
        guard.move_until_left(&grid);
        assert_eq!(guard.visited.len(), 41);
    }
    
}