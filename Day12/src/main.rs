use std::collections::{HashSet};
use pathfinding::grid::Grid;

fn main() {
    let grid_data = Farm::load_input("input.txt");
    let regions = grid_data.get_regions_for_grid();
    let result = calculate_total_score(&grid_data.grid_data, &regions);
    println!("Total score: {}", result);
    let edge_score = regions.iter().map(|region|caclulate_edge_score_for_region(region)).sum::<usize>();
    println!("Edge score: {}", edge_score);
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

fn calculate_perimeter(region: &Plot, rows: usize, cols: usize) -> (usize, HashSet<Cell>) {
    let mut perimeter = 0;
    let mut perimeter_cells = HashSet::new();

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
                perimeter_cells.insert((r, c));
            } else if !region.contains(&(nr, nc)) {
                perimeter += 1;
                perimeter_cells.insert((r, c));
            }
        }
    }

    (perimeter, perimeter_cells)
}

fn calculate_score_for_region(grid_data: &[Vec<char>], region: &Plot) -> usize {
    let (perimeter, _) = calculate_perimeter(&region, grid_data.len(), grid_data[0].len());
    let score = region.len() * perimeter;
    score
}

fn calculate_total_score(grid_data: &[Vec<char>], regions: &[Plot]) -> usize {
    regions.iter().map(|region| calculate_score_for_region(grid_data, region)).sum::<usize>()
}

pub fn sides(region: &Plot) -> usize {
    let mut corner_candidates = HashSet::new();

    for &(r, c) in region {
        let (r, c) = (r as isize, c as isize);
        corner_candidates.insert((2 * r - 1, 2 * c - 1));
        corner_candidates.insert((2 * r + 1, 2 * c - 1));
        corner_candidates.insert((2 * r + 1, 2 * c + 1));
        corner_candidates.insert((2 * r - 1, 2 * c + 1));
    }

    let mut corners = 0usize;

    for &(cr, cc) in &corner_candidates {

        let neighbors = [
            (cr - 1, cc - 1),
            (cr + 1, cc - 1),
            (cr + 1, cc + 1),
            (cr - 1, cc + 1),
        ];

        // Check which of these corresponds to a cell in the region
        let config: Vec<bool> = neighbors.iter().map(|&(nr, nc)| {
            let (cell_r, cell_c) = ((nr / 2) as usize, (nc / 2) as usize);
            if nr < 0 || nc < 0 {
                false
            } else {
                region.contains(&(cell_r, cell_c))
            }
        }).collect();

        let number = config.iter().filter(|&&x| x).count();

        match number {
            1 => {
                corners += 1;
            }
            2 => {
                // Two diagonally placed cells count as 2 corners
                if (config[0] && config[2] && !config[1] && !config[3])
                    || (config[1] && config[3] && !config[0] && !config[2])
                {
                    corners += 2;
                }
            }
            3 => {
                corners += 1;
            }
            _ => {}
        }
    }
    corners
}

fn caclulate_edge_score_for_region(region: &Plot) -> usize {
    let sides = sides(region);
    sides * region.len()
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
        let (perimeter, cells) = calculate_perimeter(&region, grid_data.grid_data.len(), grid_data.grid_data[0].len());
        assert_eq!(perimeter, 18);
        assert_eq!(cells.len(), 11);
    }

    #[test]
    fn test_calculate_num_edges() {
        let grid_data = Farm::load_input("test_input.txt");
        let region = grid_data.get_region_from_grid(0, 0);
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        println!("{}", input);
        let sides = sides(&region);
        assert_eq!(sides, 10);
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
        let edge_score = caclulate_edge_score_for_region(&region);
        assert_eq!(edge_score, 120);
    }



    #[test]
    fn test_can_calculate_total_score() {
        let grid_data = Farm::load_input("test_input.txt");
        let regions = grid_data.get_regions_for_grid();
        let result = calculate_total_score(&grid_data.grid_data, &regions);
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_calculate_edge_score() {
        let grid_data = Farm::load_input("test_input.txt");
        let regions = grid_data.get_regions_for_grid();


        let edge_score = regions.iter().map(|region|caclulate_edge_score_for_region(region)).sum::<usize>();
        assert_eq!(edge_score, 1206);
    }


}