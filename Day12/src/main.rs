use std::collections::HashSet;
use pathfinding::grid::Grid;
fn main() {
    let grid_data = Farm::load_input("input.txt");
    let regions = grid_data.get_regions_for_grid();
    let result = calculate_total_score(&grid_data.grid_data, &regions);
    println!("Total score: {}", result);
}

type Cell = (usize, usize);
type Plot = HashSet<Cell>;

struct Farm {
    grid: Grid,
    grid_data: Vec<Vec<char>>,
}

impl Farm {
    fn _new(grid: Grid, grid_data: Vec<Vec<char>>) -> Self {
        Farm { grid, grid_data }
    }

    fn load_input(input_file: &str) -> Self {
        let input = std::fs::read_to_string(input_file).unwrap();
        Farm::load_pathfinding_grid(&input)
    }

    fn load_pathfinding_grid(input: &str) -> Self {
        let grid_data: Vec<Vec<char>> = input
            .lines()
            .filter(|line| !line.trim().is_empty()) // Skip empty lines
            .map(|line| line.chars().collect())     // Convert each line to Vec<char>
            .collect();

        let rows = grid_data.len();
        let cols = grid_data[0].len();
        let mut grid = Grid::new(rows, cols);
        grid.fill();
        Farm { grid, grid_data }
    }

    fn get_region_from_grid(&self, r: usize, c: usize) -> Plot {
        let plant_type = self.grid_data[r][c];

        self.grid.dfs_reachable((r, c), |(nr, nc)| {
            self.grid_data[nr][nc] == plant_type
        })
            .into_iter().collect()
    }

    fn get_regions_for_grid(&self) -> Vec<Plot> {
        let mut regions = Vec::new();
        let mut visited: Plot = HashSet::new();

        for r in 0..self.grid_data.len() {
            for c in 0..self.grid_data[0].len() {
                if visited.contains(&(r, c)) {
                    continue;
                }

                let region = self.get_region_from_grid(r, c);
                visited.extend(region.iter());
                regions.push(region);
            }
        }

        regions
    }
}

fn calculate_perimeter(region: &Plot, rows: usize, cols: usize) -> usize {
    let mut perimeter = 0;

    for &(r, c) in region {
        let neighbors = [
            (r.wrapping_sub(1), c),    // Up
            (r+1, c),                  // Down (ensure r+1 < rows)
            (r, c.wrapping_sub(1)),    // Left
            (r, c+1),                  // Right
        ];

        for &(nr, nc) in &neighbors {
            // Check boundary
            if nr >= rows || nc >= cols {
                perimeter += 1;
            } else if !region.contains(&(nr, nc)) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn calculate_score_for_region(grid_data: &[Vec<char>], region: &Plot) -> usize {
    let perimeter = calculate_perimeter(&region, grid_data.len(), grid_data[0].len());
    let score = region.len() * perimeter;
    score
}

fn calculate_total_score(grid_data: &[Vec<char>], regions: &[Plot]) -> usize {
    regions.iter().map(|region| calculate_score_for_region(grid_data, region)).sum::<usize>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_input() {
        let grid_data = Farm::load_input("test_input.txt");
        assert_eq!(grid_data.grid_data.len(), 10);
        assert_eq!(grid_data.grid_data[0].len(), 10);
        assert_eq!(grid_data.grid.size(), 100);
        assert_eq!(grid_data.grid_data[0][0], 'R');
    }

    #[test]
    fn test_get_region_from_grid() {
        let grid_data = Farm::load_input("test_input.txt");
        let region = grid_data.get_region_from_grid(0, 0);
        // this should find the Rs of which there are 12
        assert_eq!(region.len(), 12);
    }

    #[test]
    fn test_get_parimeter_for_region() {
        let grid_data = Farm::load_input("test_input.txt");
        let region = grid_data.get_region_from_grid(0, 0);
        let perimeter = calculate_perimeter(&region, grid_data.grid_data.len(), grid_data.grid_data[0].len());
        assert_eq!(perimeter, 18);
    }

    #[test]
    fn test_get_all_regions() {
        let grid_data = Farm::load_input("test_input.txt");
        let regions = grid_data.get_regions_for_grid();
        assert_eq!(regions.len(), 11);
    }

    #[test]
    fn can_calculate_score_for_region (){
        let grid_data = Farm::load_input("test_input.txt");
        let region = grid_data.get_region_from_grid(0, 0);
        let score = calculate_score_for_region(&grid_data.grid_data, &region);
        assert_eq!(score, 216);
    }

    #[test]
    fn test_can_calculate_total_score() {
        let grid_data = Farm::load_input("test_input.txt");
        let regions = grid_data.get_regions_for_grid();
        let result = calculate_total_score(&grid_data.grid_data, &regions);
        assert_eq!(result, 1930);
    }
}