use grid::*;
use std::collections::{HashSet, VecDeque};
fn main() {
    let input_file = "input.txt";
    let grid_str = std::fs::read_to_string(input_file).unwrap();
    let (grid, summits, trail_heads) = load_grid_from_str(&grid_str);
    let total_summits_reached = find_summets(&grid, summits, trail_heads);
    println!("Part1: {:?}", total_summits_reached);
}

type Summits = HashSet<(usize, usize)>;
type TrailHeads = HashSet<(usize, usize)>;

fn load_grid_from_str(input: &str) -> (Grid<u8>, Summits, TrailHeads) {
    // parse the grid from the string
    let lines: Vec<&str> = input.lines().collect();
    let mut grid = Grid::new(lines.len(), lines[0].len());
    let mut summits = HashSet::new();
    let mut trail_heads = HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '9' => {
                    summits.insert((i, j));
                }
                '0' => {
                    trail_heads.insert((i, j));
                }
                _ => {}
            }
            *grid.get_mut(i, j).unwrap() = ch.to_digit(10).unwrap() as u8;
        }
    }
    (grid, summits, trail_heads)
}

fn find_summets(grid: &Grid<u8>, summits: Summits, trail_heads: TrailHeads) -> usize {
    //bfs to find the number of summits reachable from the trail heads

    let mut num_summits_reached = 0;
    for (i, j) in trail_heads {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((i, j, 0));
        // find neighbouring cells around the trail head which have a 1 heigher value
        while let Some((i, j, height)) = queue.pop_front() {
            if !visited.contains(&(i, j)) {
                if summits.contains(&(i, j)) {
                    num_summits_reached += 1;
                }
                visited.insert((i, j));
                for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                    let new_i = (i as i32 + di) as usize;
                    let new_j = (j as i32 + dj) as usize;

                    let new_height = grid.get(new_i, new_j);

                    if new_height.is_some() && *new_height.unwrap() == height + 1 {
                        queue.push_back((new_i, new_j, *new_height.unwrap()));
                    }
                }
            }
        }
    }

    num_summits_reached
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse_grid() {
        let grid_str = "0123\n1234\n8765\n9876";
        let (grid, summits, trail_heads) = load_grid_from_str(grid_str);
        assert_eq!(*grid.get(0, 0).unwrap(), 0);
        assert_eq!(*grid.get(0, 1).unwrap(), 1);
        assert_eq!(*grid.get(0, 2).unwrap(), 2);
        assert_eq!(*grid.get(0, 3).unwrap(), 3);
        assert_eq!(*grid.get(1, 3).unwrap(), 4);
        assert_eq!(trail_heads.len(), 1);
        assert_eq!(summits.len(), 1);
    }

    #[test]
    fn can_find_path() {
        let grid_str = "0123\n1234\n8765\n9876";
        let (grid, summits, trail_heads) = load_grid_from_str(grid_str);
        let total_summits_reached = find_summets(&grid, summits, trail_heads);
        assert_eq!(total_summits_reached, 1)
    }

    #[test]
    fn can_find_test_paths() {
        let input_file = "test_input.txt";
        let grid_str = std::fs::read_to_string(input_file).unwrap();
        let (grid, summits, trail_heads) = load_grid_from_str(&grid_str);
        let total_summits_reached = find_summets(&grid, summits, trail_heads);
        assert_eq!(total_summits_reached, 36)
    }
    #[test]
    fn can_find_test_paths2() {
        let input_file = "test_input2.txt";
        let grid_str = std::fs::read_to_string(input_file).unwrap();
        let (grid, summits, trail_heads) = load_grid_from_str(&grid_str);
        assert_eq!(summits.len(), 2);
        assert_eq!(trail_heads.len(), 1);
        let total_summits_reached = find_summets(&grid, summits, trail_heads);
        assert_eq!(total_summits_reached, 2);
    }
}
