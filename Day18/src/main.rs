use std::collections::HashSet;
use std::time::Instant;
use pathfinding::prelude::{astar, bfs, dfs};
use glam::i32::IVec2;
fn main() {
    let start_time = Instant::now(); // Start the timer
    let size = 70;
    let input_file = "input.txt";
    // read contents of file into a string
    let input = std::fs::read_to_string(input_file).unwrap();
    let all_bytes = parse_coordinates(&input).unwrap().1;
    let blocked_bytes = build_blocked_bytes(all_bytes.clone(), 1024);
    let start = IVec2::new(0, 0);
    let end =  IVec2::new(70, 70);
    let path = find_path(start, end, &blocked_bytes, IVec2::new(70, 70));
    println!("Path length: {}", path.unwrap().1);

    if let Some(blocking_byte) = find_first_blocking_byte(&all_bytes, &start, &end, size) {
        println!("{},{}", blocking_byte.x, blocking_byte.y);
    } else {
        println!("No blocking byte found within the given input.");
    }
    let duration = start_time.elapsed(); // Calculate elapsed time
    println!("Time taken: {:.2?}", duration);
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

fn find_first_blocking_byte(
    all_bytes: &Vec<(i32, i32)>,
    start: &IVec2,
    end: &IVec2,
    size: i32,
) -> Option<IVec2> {
    let mut left = 1;
    let mut right = all_bytes.len();
    let mut result = None;

    while left <= right {
        let mid = left + (right - left) / 2;
        let blocked_bytes = build_blocked_bytes(all_bytes.clone(), mid);

        if find_path(*start, *end, &blocked_bytes, IVec2::new(size, size)).is_none() {
            // Path is blocked, search in the lower half
            result = Some(all_bytes[mid - 1].into());
            right = mid - 1;
        } else {
            // Path exists, search in the upper half
            left = mid + 1;
        }
    }

    result
}



fn neighbors(
    current: IVec2,
    bounds: IVec2,
    blocked_bytes: &HashSet<IVec2>,
) -> impl Iterator<Item = (IVec2, i32)> + '_ {
    DIRECTIONS
        .iter()
        .map(move |&dir| current + dir)
        .filter(move |&neighbor| {
            neighbor.x >= 0 && neighbor.x <= bounds.x
                && neighbor.y >= 0 && neighbor.y <= bounds.y
                && !blocked_bytes.contains(&neighbor)
        })
        .map(move |neighbor| (neighbor, 1))
}

fn find_path(
    start: IVec2,
    end: IVec2,
    blocked_bytes: &HashSet<IVec2>,
    bounds: IVec2,
) -> Option<(Vec<IVec2>, i32)> {
    astar(
        &start,
        move |current| neighbors(*current, bounds, blocked_bytes),
        |current| {
            let diff = end - *current;
            diff.x.abs() + diff.y.abs() // Manhattan distance heuristic
        },
        |current| *current == end,
    )
}



fn is_valid_position(p: IVec2, bounds: IVec2, blocked_bytes: &HashSet<IVec2>) -> bool {
    p.x >= 0 && p.x <= bounds.x && p.y >= 0 && p.y <= bounds.y && !blocked_bytes.contains(&p)
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
        let path = find_path(start, end, &blocked_bytes, IVec2::new(6, 6));
        assert_eq!(path.unwrap().1, 22);
    }


}
