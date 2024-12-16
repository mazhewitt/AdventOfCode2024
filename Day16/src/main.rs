use pathfinding::prelude::{dijkstra};
use std::collections::HashSet;

fn main() {
    let input_file = "input.txt";
    let input = std::fs::read_to_string(input_file).expect("Error reading input file");
    let (maze, start, end) = parse_maze(&input);
    let result = find_shortest_paths(&maze, &start, end);

    if result.is_none() {
        println!("No path found!");
        return;
    }

    println!("Shortest path score: {}", result.unwrap().1);

    // Part 2: Find all tiles in best paths
    let best_path_tiles = find_tiles_in_best_paths(&maze, &start, end);
    println!(
        "Number of tiles in best paths: {}",
        best_path_tiles.len()
    );

    // Optional: Display the maze with marked tiles
    display_maze_with_paths(&maze, &best_path_tiles);
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

fn find_tiles_in_best_paths(
    maze: &Vec<Vec<char>>,
    start: &Node,
    end: (usize, usize),
) -> HashSet<(usize, usize)> {
    let paths = find_shortest_paths(maze, start, end);

    let mut best_path_tiles = HashSet::new();
    if let Some((path, _)) = paths {
        for node in path {
            best_path_tiles.insert((node.x, node.y));
        }
    }

    best_path_tiles
}

fn display_maze_with_paths(maze: &Vec<Vec<char>>, best_path_tiles: &HashSet<(usize, usize)>) {
    for (y, row) in maze.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if best_path_tiles.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", tile);
            }
        }
        println!();
    }
}


fn find_shortest_paths(maze: &Vec<Vec<char>>, start: &Node, end: (usize, usize)) -> Option<(Vec<Node>, usize)> {
    let result = dijkstra(
        start,
        |node| successors(&maze, node),
        |node| node.x == end.0 && node.y == end.1,
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
            
            let (maze, _, _) = parse_maze(input);
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

        let result = find_shortest_paths(&maze, &start, end);
        
        assert_eq!(result.unwrap().1, 7036);
    }

    #[test]
    fn test_load_input_file() {
        let input_file = "test_input.txt";
        let input = std::fs::read_to_string(input_file).expect("Error reading input file");
        let (maze, start, end) = parse_maze(&input);

        let result = find_shortest_paths(&maze, &start, end);

        assert_eq!(result.unwrap().1, 11048);
    }

    #[test]
    fn test_find_tiles_in_best_paths() {
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
            let best_path_tiles = find_tiles_in_best_paths(&maze, &start, end);
            assert_eq!(best_path_tiles.len(), 37);
        }

}