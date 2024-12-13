use std::collections::{HashSet};
use pathfinding::grid::Grid;

fn main() {

}

type Cell = (usize, usize);
type Plot = HashSet<Cell>;

struct Farm {

}

impl Farm {


    fn load_input(input_file: &str) -> Self {
        let input = std::fs::read_to_string(input_file).unwrap();
        Farm::load_pathfinding_grid(&input)
    }

    fn load_pathfinding_grid(input: &str) -> Self {

        Farm {  }
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_input() {
        let farm = Farm::load_input("test_input.txt");

    }

    #[test]
    fn test_get_region_from_grid() {
        let farm = Farm::load_input("test_input.txt");

        // this should find the Rs of which there are 12
        assert_eq!(region.len(), 12);
    }

    #[test]
    fn test_get_parimeter_for_region() {
        let farm = Farm::load_input("test_input.txt");
        let perimeter = 0;
        
        assert_eq!(perimeter, 18);
    }

    #[test]
    fn test_calculate_num_edges() {
        let grid_data = Farm::load_input("test_input.txt");

        let sides = 0;
        assert_eq!(sides, 10);
    }

    #[test]
    fn test_get_all_regions() {
        let grid_data = Farm::load_input("test_input.txt");
        let regions = 0;
        assert_eq!(regions, 11);
    }

    #[test]
    fn can_calculate_score_for_region (){
        let grid_data = Farm::load_input("test_input.txt");
        let region = 0;
        let score = 0;
        assert_eq!(score, 216);
        let edge_score = 0;
        assert_eq!(edge_score, 120);
    }



    #[test]
    fn test_can_calculate_total_score() {
        let farm_data = Farm::load_input("test_input.txt");
        let result = 0;
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_calculate_edge_score() {
        let farm_data = Farm::load_input("test_input.txt");
        let edge_score = 0;
        assert_eq!(edge_score, 1206);
    }


}