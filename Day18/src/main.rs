use std::collections::HashSet;
use pathfinding::prelude::{bfs, dfs};
use glam::i32::IVec2;
fn main() {
    let size = 70;
    let input_file = "input.txt";
    // read contents of file into a string
    let input = std::fs::read_to_string(input_file).unwrap();
    let blocked_bytes = build_blocked_bytes(parse_coordinates(&input).unwrap().1, 1024);
    let start = IVec2::new(0, 0);
    let end =  IVec2::new(70, 70);
    let path = find_path(start, end, blocked_bytes, IVec2::new(70, 70));
    println!("Path length: {}", path.unwrap().len()-1);
}

const DIRECTIONS: [IVec2; 4] =
    [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

use nom::{
    character::complete::{char, digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};




fn find_path(
    start: IVec2,
    end: IVec2,
    blocked_bytes: HashSet<IVec2>,
    bounds: IVec2,
) -> Option<Vec<IVec2>> {
    bfs(
        &start,
        |&pos| {
            DIRECTIONS
                .iter()
                .filter_map(|dir| {
                    let next_pos = pos + *dir;
                    if (0..=bounds.x).contains(&next_pos.x)
                        && (0..=bounds.y).contains(&next_pos.y)
                        && !blocked_bytes.contains(&next_pos)
                    {
                        Some(next_pos)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        },
        |&pos| pos == end,
    )
}

/// Parse a single coordinate `(x, y)`
fn parse_coordinate(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse::<i32>()), // Parse `x`
        char(','),                                    // Separator
        map_res(digit1, |s: &str| s.parse::<i32>()), // Parse `y`
    )(input)
}

/// Parse the full input into a list of `(x, y)` coordinates
fn parse_coordinates(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(newline, parse_coordinate)(input)
}

fn build_blocked_bytes(vec: Vec<(i32, i32)>, num_bytes: usize) -> HashSet<IVec2> {
    vec[0..num_bytes]
        .iter()
        .map(|&(x, y)| IVec2::new(x, y))
        .collect()
}




#[cfg(test)]
mod tests {
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
    fn test_build_corrupted(){
        let size = 7;
        let input_file = "test_input.txt";
        // read contents of file into a string
        let input = std::fs::read_to_string(input_file).unwrap();
        let coordinates = parse_coordinates(&input).unwrap().1;
        let blocked_bytes = build_blocked_bytes(coordinates, 12);
        assert_eq!(blocked_bytes.len(), 12);
    }



    #[test]
    fn test_find_path() {
        let size = 7;
        let input_file = "test_input.txt";
        // read contents of file into a string
        let input = std::fs::read_to_string(input_file).unwrap();
        let blocked_bytes = build_blocked_bytes(parse_coordinates(&input).unwrap().1, 12);
        let start = IVec2::new(0, 0);
        let end =  IVec2::new(6, 6);
        let path = find_path(start, end, blocked_bytes, IVec2::new(6, 6));
        assert_eq!(path.unwrap().len()-1, 22);
    }


}
