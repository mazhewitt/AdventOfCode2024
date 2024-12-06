use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use grid::*;
use rayon::prelude::*;

fn main() {
    let input_file = "input.txt";
    let grid = load_grid(input_file).expect("Failed to load grid");
    let position = find_guard_position(&grid).expect("No Guard Found");
    let mut guard = Guard::new(position, Direction::UP);
    guard.move_until_left_or_looped(&grid);
    println!("Visited: {}", guard.visited.len());
    let loops = find_looping_positions_parallel(grid, position);
    println!("Loops: {}", loops);

}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Direction {
    d_row: isize,
    d_col: isize,
}

impl Direction {
    // Directions as constants
    const UP: Direction = Direction { d_row: -1, d_col: 0 };
    const DOWN: Direction = Direction { d_row: 1, d_col: 0 };
    const LEFT: Direction = Direction { d_row: 0, d_col: -1 };
    const RIGHT: Direction = Direction { d_row: 0, d_col: 1 };

    // Method to turn right
    fn turn_right(&self) -> Direction {
        match *self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            _ => *self, // Default case if no match (shouldn't happen for valid directions)
        }
    }
}

struct Guard {
    position: (usize, usize),
    direction: Direction,
    visited_with_direction: HashSet<((usize, usize), Direction)>,
    visited: HashSet<(usize, usize)>,
}

#[derive(Debug, PartialEq)]
enum LoopOrExit {
    Loop,
    Exit,
    Moving
}



impl Guard {
    fn new(position: (usize, usize), direction: Direction) -> Guard {
        let mut visited = HashSet::new();
        visited.insert(position);
        let mut visited_with_direction = HashSet::new();
        visited_with_direction.insert((position, direction));
        Guard {
            position,
            direction,
            visited_with_direction,
            visited,
        }
    }

    fn move_guard(&mut self, grid: &Grid<char>) -> LoopOrExit {
        let (row, col) = self.position;
        let new_row = (row as isize + self.direction.d_row) as usize;
        let new_col = (col as isize + self.direction.d_col) as usize;
        if grid.get(new_row, new_col).is_none() {
            return LoopOrExit::Exit
        }
        if grid.get(new_row, new_col) == Some(&'#') {
            //turn right
            self.direction = self.direction.turn_right();
        }
        else if self.visited_with_direction.contains(&((new_row, new_col), self.direction)) {
            return LoopOrExit::Loop
        }
        else {
            self.position = (new_row, new_col);
            self.visited_with_direction.insert((self.position, self.direction));
            self.visited.insert(self.position);
        }
        LoopOrExit::Moving
    }

    fn move_until_left_or_looped(&mut self, grid: &Grid<char>) -> LoopOrExit{
        let mut move_state = LoopOrExit::Moving;
        while move_state == LoopOrExit::Moving {
            move_state = self.move_guard(grid);
        }
        move_state
    }
}

fn will_loop(grid: &Grid<char>, position: (usize, usize), direction: Direction) -> bool {
    let mut guard = Guard::new(position, direction);
    let looped_or_left = guard.move_until_left_or_looped(grid);
    if looped_or_left == LoopOrExit::Loop {
        return true;
    }
    false
}

// given a grid, add a # to each position and see if the guard will loop
fn find_looping_positions(grid: &mut Grid<char>, guard_position: (usize,usize)) -> usize {
    let mut looping_positions = 0;

    let row_count = grid.rows();
    let col_count = grid.cols();

    for row_idx in 0..row_count {
        for col_idx in 0..col_count {
            if (row_idx, col_idx) == guard_position {
                continue;
            }
            if *grid.get(row_idx, col_idx).unwrap() == '#' {
                continue;
            }
            *grid.get_mut(row_idx, col_idx).unwrap() = '#';
            if will_loop(grid, guard_position, Direction::UP) {
                looping_positions += 1;
            }
            *grid.get_mut(row_idx, col_idx).unwrap() = '.';
        }
    }

    looping_positions
}

fn find_looping_positions_parallel(grid_o:  Grid<char>, guard_position: (usize, usize)) -> usize {
    let row_count = grid_o.rows();
    let col_count = grid_o.cols();
    
    (0..row_count)
        .into_par_iter()
        .map(|row_idx| {
            let mut local_looping_positions = 0;
            let mut grid = grid_o.clone();
            for col_idx in 0..col_count {
                if (row_idx, col_idx) == guard_position {
                    continue;
                }
                if *grid.get(row_idx, col_idx).unwrap() == '#' {
                    continue;
                }

                *grid.get_mut(row_idx, col_idx).unwrap() = '#';
                if will_loop(&grid, guard_position, Direction::UP) {
                    local_looping_positions += 1;
                }
                *grid.get_mut(row_idx, col_idx).unwrap() = '.';
            }

            local_looping_positions
        })
        .sum()
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
        let mut guard = Guard::new(position, Direction::UP);
        guard.move_until_left_or_looped(&grid);
        assert_eq!(guard.visited.len(), 41);
    }

    #[test]
    fn test_will_loop() {
        let input_file = "looping_grid.txt";
        let grid = load_grid(input_file).expect("Failed to load grid");
        let position = find_guard_position(&grid).expect("No Guard Found");
        assert_eq!(will_loop(&grid, position, Direction::UP), true);
    }

    #[test]
    fn test_count_loops() {
        let input_file = "test_input.txt";
        let grid = load_grid(input_file).expect("Failed to load grid");
        let position = find_guard_position(&grid).expect("No Guard Found");
        let loops = find_looping_positions_parallel(grid, position);
        assert_eq!(loops, 6);
    }
}