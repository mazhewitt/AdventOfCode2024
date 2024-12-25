use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use glam::IVec2;
use pathfinding::prelude::astar;

fn main() {
    println!("Hello, world!");
}

fn load_grid(filename: &str) -> Vec<Vec<char>> {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut grid = Vec::new();
    for line in contents.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

const DIRECTIONS: [IVec2; 4] =
    [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

fn neighbors(
    current: IVec2,
    bounds: IVec2,
    blocked_bytes: &HashSet<IVec2>,
) -> impl Iterator<Item=(IVec2, i32)> + '_ {
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
    walls: &HashSet<IVec2>,
    bounds: IVec2,
) -> Option<(Vec<IVec2>, i32)> {
    astar(
        &start,
        move |current| neighbors(*current, bounds, walls),
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

fn find_start_end(grid: &Vec<Vec<char>>) -> (IVec2, IVec2, HashSet<IVec2>) {
    let mut walls = HashSet::new();
    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);
            match cell {
                'S' => start = pos,
                'E' => end = pos,
                '#' => { walls.insert(pos); }
                _ => {}
            }
        }
    }
    (start, end, walls)
}

fn shortest_path(grid: &Vec<Vec<char>>, start: IVec2, end: IVec2, walls: &HashSet<IVec2>) -> Option<i32> {
    // use A* algorithm from pathfinding to find the shortest path
    let mut path = 0;
    let bounds = IVec2::new(grid[0].len() as i32, grid.len() as i32);
    if let Some((p, cost)) = find_path(start, end, walls, bounds) {
        path = cost;
    }
    Some(path)
}

fn distances_walls_to_end(grid: &Vec<Vec<char>>, end: IVec2, walls: &HashSet<IVec2>) -> HashMap<IVec2, i32> {


    // calculate the distance to the start form all the walls and store in a hashmap
    let mut wall_distances_to_end = HashMap::new();
    for wall in walls.iter() {
        let mut use_walls = walls.clone();
        use_walls.remove(wall);
        let distance = shortest_path(&grid, end, *wall, &use_walls);
        if distance.is_some() {
            wall_distances_to_end.insert(*wall, distance.unwrap());
        }

    }
    wall_distances_to_end
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct RaceTrackNode {
    position: IVec2,
    distance_from_start: i32,
    remaining_cheats: i32,
    cheat_start: Option<IVec2>,
}

fn calculate_cheats(grid: &Vec<Vec<char>>, start: IVec2, end: IVec2, walls: &HashSet<IVec2>) -> HashMap<i32, i32> {
    let start_to_end = shortest_path(&grid, start, end, &walls).expect("No path found from start to end");
    //BFS from the start to the end
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(RaceTrackNode {
        position: start,
        distance_from_start: 0,
        remaining_cheats: 2,
        cheat_start: Option::None,
    });
    let mut cheat_map = HashMap::new();
    while let Some(current) = queue.pop_front() {
        if current.position == end {
            if current.distance_from_start < start_to_end && current.cheat_start.is_some() {
                let saving = start_to_end - current.distance_from_start;
                cheat_map.insert(current.cheat_start, saving);
            }
            continue;
        }

        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        if current.distance_from_start > start_to_end {
            continue;
        }
        // add the neighbors
        for &dir in DIRECTIONS.iter() {
            let next_position = current.position + dir;
            if next_position.x >= 0 && next_position.x < grid[0].len() as i32
                && next_position.y >= 0 && next_position.y < grid.len() as i32 {
                if walls.contains(&next_position) {
                    // if we have a cheat
                    if current.remaining_cheats > 1 {
                        // push a new node with the cheat active
                        if current.cheat_start.is_none() {
                            let new_node = RaceTrackNode {
                                position: next_position,
                                distance_from_start: current.distance_from_start + 1,
                                remaining_cheats: current.remaining_cheats - 1,
                                cheat_start: Some(current.position),
                            };
                            queue.push_back(new_node);
                        }
                        else {
                            let new_node = RaceTrackNode {
                                position: next_position,
                                distance_from_start: current.distance_from_start + 1,
                                remaining_cheats: current.remaining_cheats - 1,
                                cheat_start: current.cheat_start,
                            };
                            queue.push_back(new_node);
                        }
                    }
                    continue
                }
                if current.cheat_start.is_some() {
                    // if we have a cheat
                    if current.remaining_cheats > 0 {
                        let new_node = RaceTrackNode {
                            position: next_position,
                            distance_from_start: current.distance_from_start + 1,
                            cheat_start: current.cheat_start,
                            remaining_cheats: current.remaining_cheats - 1,
                        };
                        queue.push_back(new_node);
                    } else {
                        let new_node = RaceTrackNode {
                            position: next_position,
                            distance_from_start: current.distance_from_start + 1,
                            cheat_start: current.cheat_start,
                            remaining_cheats: 0,
                        };
                        queue.push_back(new_node);
                    }
                } else {
                    let new_node = RaceTrackNode {
                        position: next_position,
                        distance_from_start: current.distance_from_start + 1,
                        cheat_start: current.cheat_start,
                        remaining_cheats: current.remaining_cheats,
                    };
                    queue.push_back(new_node);
                }
            }
        }
    }
    // count the number of cheats for each saving
    let mut savings_map = HashMap::new();
    for (_, saving) in cheat_map.iter() {
        let count = savings_map.entry(*saving).or_insert(0);
        *count += 1;
    }
    savings_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_grid() {
        // load the grid as Vec<Vec<char>>
        let grid = load_grid("test_input.txt");
        assert_eq!(grid[0][0], '#');
        assert_eq!(grid[3][1], 'S');
        assert_eq!(grid[7][5], 'E');
    }

    #[test]
    fn find_walls_start_end() {
        let grid = load_grid("test_input.txt");
        let (start, end, walls) = find_start_end(&grid);
        assert_eq!(start, IVec2::new(1, 3));
        assert_eq!(end, IVec2::new(5, 7));
    }


    #[test]
    fn test_shortest_path() {
        let grid = load_grid("test_input.txt");
        let (start, end, walls) = find_start_end(&grid);
        let path = shortest_path(&grid, start, end, &walls);
        assert_eq!(path.unwrap(), 84);
    }


    #[test]
    fn test_small_example() {
        let grid_chars = r"##########
#.S......#
########.#
#........#
#E.......#
##########
";
        let grid = grid_chars.lines().map(|l| l.chars().collect()).collect();
        let (start, end, walls) = find_start_end(&grid);
        let path = shortest_path(&grid, start, end, &walls);
        assert_eq!(path.unwrap(), 16);
        let savings_map = calculate_cheats(&grid, start, end, &walls);
        println!("{:?}", savings_map);

    }

    #[test]
    fn test_distance_with_cheats() {
        let grid = load_grid("test_input.txt");
        let (start, end, walls) = find_start_end(&grid);
        let savings_map = calculate_cheats(&grid, start, end, &walls);

        println!("{:?}", savings_map);
    }



    #[test]
    fn test_single_cheat_position() {
        let grid = load_grid("test_input.txt");
        let (start, end, walls) = find_start_end(&grid);

        let chosen_wall = IVec2::new(8, 1);
        let mut cheat_walls = walls.clone();
        cheat_walls.remove(&chosen_wall);

        let cheat_path_length = shortest_path(&grid, start, end, &cheat_walls)
            .expect("No path found with cheat");
        assert_eq!(cheat_path_length, 72);

        let mut cheat_walls = walls.clone();
        cheat_walls.remove(&chosen_wall);


        let distance_from_chosen_wall_to_start = shortest_path(&grid, start, chosen_wall, &cheat_walls)
            .expect("No path found from chosen wall to start");
        let distance_from_chosen_wall_to_end = shortest_path(&grid, end, chosen_wall, &cheat_walls)
            .expect("No path found from chosen wall to end");
        assert_eq!(distance_from_chosen_wall_to_start + distance_from_chosen_wall_to_end, 72);

    }


}

