use pathfinding::grid::Grid;

fn main() {
    let size = 70;
    let input_file = "input.txt";
    // read contents of file into a string
    let input = std::fs::read_to_string(input_file).unwrap();
    let blocked_bytes = parse_coordinates(&input).unwrap().1;
    let slice_use = &blocked_bytes[0..1023];
    let grid = build_grid(size, slice_use);

    let start = (0, 0);
    let end = (70, 70);
    let path = dfs(
        start,
        |&pos| grid.neighbours(pos),
        |&pos| pos == end,
    );
    println!("Path length: {}", path.unwrap().len()-1);
}


use nom::{
    character::complete::{char, digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use pathfinding::prelude::{bfs, dfs};

fn build_grid(size: usize, blocking_bytes: &[(usize, usize)]) -> Grid {

    let mut grid = Grid::new(size, size);
    grid.fill();
    for coordinate in blocking_bytes {
        grid.remove_vertex(*coordinate);
    }
    grid
}

fn find_path(
    grid: &Grid,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    dfs(
        start,
        |&pos| grid.neighbours(pos),
        |&pos| pos == end,
    )
}

/// Parse a single coordinate `(x, y)`
fn parse_coordinate(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse::<usize>()), // Parse `x`
        char(','),                                    // Separator
        map_res(digit1, |s: &str| s.parse::<usize>()), // Parse `y`
    )(input)
}

/// Parse the full input into a list of `(x, y)` coordinates
fn parse_coordinates(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(newline, parse_coordinate)(input)
}


fn successors(pos: (usize, usize), grid: &Grid) -> Vec<(usize, usize)> {
    grid.neighbours(pos)
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use pathfinding::prelude::bfs;
    use super::*;

    #[test]
    fn test_parse_single_coordinate() {
        let input = "5,4";
        let expected = (5, 4);
        let result = parse_coordinate(input).unwrap();
        assert_eq!(result.1, expected);
    }

    #[test]
    fn test_parse_multiple_coordinates() {
        let input = "5,4\n4,2\n4,5";
        let expected = vec![(5, 4), (4, 2), (4, 5)];
        let result = parse_coordinates(input).unwrap();
        assert_eq!(result.1, expected);
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";
        let result = parse_coordinates(input);
        assert!(result.is_err()); // Parsing empty input should fail
    }

    #[test]
    fn test_build_grid(){
        let size = 7;
        let input_file = "test_input.txt";
        // read contents of file into a string
        let input = std::fs::read_to_string(input_file).unwrap();
        let coordinates = parse_coordinates(&input).unwrap().1;
        let grid = build_grid(size, &coordinates);
        assert_eq!(grid.size(), (size*size));
    }
    #[test]
    fn test_find_path() {
        let size = 7;
        let input_file = "test_input.txt";
        // read contents of file into a string
        let input = std::fs::read_to_string(input_file).unwrap();
        let blocked_bytes = parse_coordinates(&input).unwrap().1;
        let slice_use = &blocked_bytes[0..11];
        let grid = build_grid(size, slice_use);
        println!("{:?}", grid);
        let start = (0, 0);
        let end = (6, 6);
        let path = find_path(&grid, start, end);
        assert_eq!(path.unwrap().len()-1 , 22)

    }

    #[test]
    fn test_find_neighbours(){
        let size = 7;

        let blocked_bytes = vec![(1,1), (1,2)];
        let grid = build_grid(size, &blocked_bytes);
        let neighbours = grid.neighbours((2,2));
        assert_eq!(neighbours.len(), 3);
    }
}
