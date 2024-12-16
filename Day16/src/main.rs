use pathfinding::prelude::yen;
use std::collections::HashSet;

fn main() {
    let input_file = "input.txt";
    let input = std::fs::read_to_string(input_file).expect("Error reading input file");
    let (maze, start, end) = parse_maze(&input);
    let num_paths = 1;
    let result = find_shortest_paths(&maze, &start, end, num_paths);
    println!("Shortest path: {}", result[0].1);
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Node {
    x: usize,
    y: usize,
    dir: usize, // 0: East, 1: South, 2: West, 3: North
}

fn parse_maze(input: &str) -> (Vec<Vec<char>>, Node, (usize, usize)) {
    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = None;
    let mut end = None;

    for (y, row) in maze.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == 'S' {
                start = Some(Node { x, y, dir: 0 });
            } else if tile == 'E' {
                end = Some((x, y));
            }
        }
    }

    (maze, start.unwrap(), end.unwrap())
}

fn successors(
    maze: &Vec<Vec<char>>,
    node: &Node,
) -> Vec<(Node, usize)> {
    let mut result = Vec::new();
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    // Move forward
    let (dx, dy) = directions[node.dir];
    let new_x = node.x as isize + dx;
    let new_y = node.y as isize + dy;

    if new_x >= 0
        && new_y >= 0
        && new_x < maze[0].len() as isize
        && new_y < maze.len() as isize
        && maze[new_y as usize][new_x as usize] != '#'
    {
        result.push((
            Node {
                x: new_x as usize,
                y: new_y as usize,
                dir: node.dir,
            },
            1,
        ));
    }

    // Turn left and right
    result.push((
        Node {
            x: node.x,
            y: node.y,
            dir: (node.dir + 3) % 4, // Left
        },
        1000,
    ));
    result.push((
        Node {
            x: node.x,
            y: node.y,
            dir: (node.dir + 1) % 4, // Right
        },
        1000,
    ));

    result
}


fn find_shortest_paths(maze: &Vec<Vec<char>>, start: &Node, end: (usize, usize), num_paths: usize) -> Vec<(Vec<Node>, usize)> {
    let result = yen(
        start,
        |node| successors(&maze, node),
        |node| node.x == end.0 && node.y == end.1,
        num_paths,
    );
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_maze() {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

        let (maze, start, end) = parse_maze(input);
        assert_eq!(maze.len(), 15);
        assert_eq!(maze[0].len(), 15);
        assert_eq!(start.x, 1);
        assert_eq!(start.y, 13);
        assert_eq!(start.dir, 0);
        assert_eq!(end.0, 13);
        assert_eq!(end.1, 1);
    }
    
    #[test]
    fn test_successors() {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
            
            let (maze, start, end) = parse_maze(input);
            let node = Node { x: 1, y: 13, dir: 0 };
            let result = successors(&maze, &node);
            assert_eq!(result.len(), 3);
            assert_eq!(result[0].0.x, 2);
            assert_eq!(result[0].0.y, 13);
            assert_eq!(result[0].0.dir, 0);
            assert_eq!(result[1].0.x, 1);
            assert_eq!(result[1].0.y, 13);
            assert_eq!(result[1].0.dir, 3);
            assert_eq!(result[2].0.x, 1);
            assert_eq!(result[2].0.y, 13);
            assert_eq!(result[2].0.dir, 1);
    
    }

    #[test]
    fn test_walk_maze() {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

        let (maze, start, end) = parse_maze(input);
        let num_paths = 1;
        let result = find_shortest_paths(&maze, &start, end, num_paths);
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 7036);
    }

    #[test]
    fn test_load_input_file() {
        let input_file = "test_input.txt";
        let input = std::fs::read_to_string(input_file).expect("Error reading input file");
        let (maze, start, end) = parse_maze(&input);
        let num_paths = 1;
        let result = find_shortest_paths(&maze, &start, end, num_paths);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 7036);
    }
}