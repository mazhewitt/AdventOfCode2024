use std::collections::{HashMap, HashSet, VecDeque};
use glam::IVec2;
use pathfinding::prelude::astar;

/// Four cardinal directions: up, right, down, left
const DIRECTIONS: [IVec2; 4] = [IVec2::Y, IVec2::X, IVec2::NEG_Y, IVec2::NEG_X];

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum CheatState {
        NotUsed,
        InUse(u8, IVec2),
        UsedUp(IVec2, IVec2),
    }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct BfsNode {
    position: IVec2,
    cheat_state: CheatState,
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

fn shortest_path_no_cheat(grid: &Vec<Vec<char>>, start: IVec2, end: IVec2, walls: &HashSet<IVec2>) -> Option<i32> {
    // use A* algorithm from pathfinding to find the shortest path
    let mut path = 0;
    let bounds = IVec2::new(grid[0].len() as i32, grid.len() as i32);
    if let Some((_, cost)) = find_path(start, end, walls, bounds) {
        path = cost;
    }
    Some(path)
}


/// Perform a BFS that accounts for one 2-step cheat window.
/// Returns a map from (cheat_start, cheat_end) -> best distance achieving that cheat.
/// (If you just need to count how many saves >= 100, you can reduce or adapt.)
fn bfs_with_cheat(
    grid: &[Vec<char>],
    start: IVec2,
    end: IVec2,
    walls: &HashSet<IVec2>,
    base_distance: i32, // the non-cheating distance for S->E
    num_cheats: u8,
) -> HashMap<(IVec2, IVec2), i32> {
    let bounds = IVec2::new(grid[0].len() as i32, grid.len() as i32);

    let mut visited = HashMap::<BfsNode, i32>::new();

    let mut cheat_results = HashMap::<(IVec2, IVec2), i32>::new();
    let init_state = BfsNode {
        position: start,
        cheat_state: CheatState::NotUsed,
    };
    visited.insert(init_state, 0);

    let mut queue = VecDeque::new();
    queue.push_back((init_state, 0)); // (BfsNode, distance)

    while let Some((node, dist)) = queue.pop_front() {
        let BfsNode {
            position: pos,
            cheat_state,
        } = node;

        if dist >= base_distance {
            continue;
        }


        match cheat_state {
            CheatState::UsedUp(cs, ce) => {

                let path_to_end = find_path(pos, end, walls, bounds);

                if let Some((_, dist_to_end)) = path_to_end {
                    let total_dist = dist + dist_to_end;

                    let entry = cheat_results.entry((cs, ce)).or_insert(total_dist);
                    if total_dist < *entry {
                        *entry = total_dist;
                    }
                }

                continue;
            }
            _ => {
                if pos == end {
                    continue;
                }
            }
        }



        // Explore neighbors
        for &dir in DIRECTIONS.iter() {
            let next_pos = pos + dir;
             if next_pos.x < 0
                || next_pos.y < 0
                || next_pos.x >= bounds.x
                || next_pos.y >= bounds.y
            {
                continue;
            }

            let next_is_wall = walls.contains(&next_pos);

             let (next_state, next_dist) = match cheat_state {
                 CheatState::NotUsed => {
                    if next_is_wall {
                        // Start cheating, 1 move used up
                        (CheatState::InUse(num_cheats-1, pos), dist + 1)
                    } else {
                        // Normal move
                        (CheatState::NotUsed, dist + 1)
                    }
                }
               CheatState::InUse(steps_left, cheat_start) => {
                    if steps_left == 0 {
                         if next_is_wall {
                            continue;
                        } else {
                            // We move one step on track, but we've ended cheat last step
                            (CheatState::UsedUp(cheat_start, pos), dist + 1)
                        }
                    } else {
                        if next_is_wall {
                             let r = steps_left - 1;
                            if r == 0 {
                                continue;
                            }
                            (CheatState::InUse(r, cheat_start), dist + 1)
                        } else {
                            let r = steps_left - 1;

                            let remain_node = BfsNode {
                                position: next_pos,
                                cheat_state: CheatState::InUse(r, cheat_start),
                            };
                            let end_node = BfsNode {
                                position: next_pos,
                                cheat_state: CheatState::UsedUp(cheat_start, next_pos),
                            };
                             push_if_better(&mut visited, &mut queue, remain_node, dist + 1);
                            push_if_better(&mut visited, &mut queue, end_node, dist + 1);
                            continue;
                        }

                    }
                }
                CheatState::UsedUp(cs, ce) => {
                     if next_is_wall {
                        continue;
                    }
                    (CheatState::UsedUp(cs, ce), dist + 1)
                }
            };

            let next_node = BfsNode {
                position: next_pos,
                cheat_state: next_state,
            };
            push_if_better(&mut visited, &mut queue, next_node, next_dist);
        }
    }

    cheat_results
}

fn push_if_better(
    visited: &mut HashMap<BfsNode, i32>,
    queue: &mut VecDeque<(BfsNode, i32)>,
    node: BfsNode,
    new_dist: i32,
) {
    if let Some(&best_so_far) = visited.get(&node) {
        if best_so_far <= new_dist {
            return;
        }
    }

    visited.insert(node, new_dist);
    queue.push_back((node, new_dist));
}

fn get_savings_count_with_cheats(grid: &Vec<Vec<char>>, cheats: u8) -> HashMap<i32, i32> {
    let (start, end, walls) = find_start_end(&grid);
     let base_distance = shortest_path_no_cheat(&grid, start, end, &walls)
        .expect("No path found from S to E without cheating!");

    let cheat_results_map = bfs_with_cheat(&grid, start, end, &walls, base_distance, cheats);

    let mut savings_count: HashMap<i32, i32> = HashMap::new();
    for ((_, _), dist) in cheat_results_map.iter() {
        let save = base_distance - dist;
         if save > 0 {
            *savings_count.entry(save).or_insert(0) += 1;
        }
    }

     let _: i32 = savings_count
        .iter()
        .filter_map(|(save, count)| {
            if *save >= 100 {
                Some(*count)
            } else {
                None
            }
        })
        .sum();
    savings_count
}

fn main() {
    let grid = load_grid("input.txt");
    let savings_map     = get_savings_count_with_cheats(&grid, 2);
    let big_savers: i32 = savings_map
        .iter()
        .filter_map(|(save, count)| {
            if *save >= 100 {
                Some(*count)
            } else {
                None
            }
        })
        .sum();
    println!("Savings map: {savings_map:?}");
    println!("Cheats saving >=100 picoseconds = {big_savers}");
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
        let (start, end, _) = find_start_end(&grid);
        assert_eq!(start, IVec2::new(1, 3));
        assert_eq!(end, IVec2::new(5, 7));
    }


    #[test]
    fn test_shortest_path() {
        let grid = load_grid("test_input.txt");
        let (start, end, walls) = find_start_end(&grid);
        let path = shortest_path_no_cheat(&grid, start, end, &walls);
        assert_eq!(path.unwrap(), 84);
    }

    #[test]
    fn test_distance_with_cheats() {
        let grid = load_grid("test_input.txt");

        let savings_count = get_savings_count_with_cheats(&grid, 2);
        //There are 14 cheats that save 2 picoseconds.
        assert_eq!(savings_count.get(&2).unwrap(), &14);
        // There are 14 cheats that save 4 picoseconds.
        assert_eq!(savings_count.get(&4).unwrap(), &14);
        // There are 2 cheats that save 6 picoseconds.
        assert_eq!(savings_count.get(&6).unwrap(), &2);
        // There are 4 cheats that save 8 picoseconds.
        assert_eq!(savings_count.get(&8).unwrap(), &4);
        // There are 2 cheats that save 10 picoseconds.
        assert_eq!(savings_count.get(&10).unwrap(), &2);
        // There are 3 cheats that save 12 picoseconds.
        assert_eq!(savings_count.get(&12).unwrap(), &3);
        // There is one cheat that saves 20 picoseconds.
        assert_eq!(savings_count.get(&20).unwrap(), &1);
        // There is one cheat that saves 36 picoseconds.
        assert_eq!(savings_count.get(&36).unwrap(), &1);
        // There is one cheat that saves 38 picoseconds.
        assert_eq!(savings_count.get(&38).unwrap(), &1);
        // There is one cheat that saves 40 picoseconds.
        assert_eq!(savings_count.get(&40).unwrap(), &1);
        // There is one cheat that saves 64 picoseconds.
        assert_eq!(savings_count.get(&64).unwrap(), &1);

        println!("Savings map: {savings_count:?}");
        let big_savers: i32 = savings_count
            .iter()
            .filter_map(|(save, count)| {
                if *save >= 1 {
                    Some(*count)
                } else {
                    None
                }
            })
            .sum();
        println!("Cheats saving >=100 picoseconds = {big_savers}");
    }

    #[test]
    fn no_walls_in_grid() {
        let grid_chars = [
            "SE.",
            "...",
            "...",
        ];
        let grid: Vec<Vec<char>> = grid_chars.iter()
            .map(|row| row.chars().collect())
            .collect();


        let cheat_results = get_savings_count_with_cheats(&grid, 20);
        // no walls to skip.
        assert!(cheat_results.is_empty(), "Should have no cheat results if no walls exist");
    }

    #[test]
    fn single_wall_off_path() {
        let grid_chars = [
            "S.E",
            "...",
            "..#",
        ];
        let grid: Vec<Vec<char>> = grid_chars.iter()
            .map(|row| row.chars().collect())
            .collect();
        let (start, end, walls) = find_start_end(&grid);
        let base = shortest_path_no_cheat(&grid, start, end, &walls).unwrap();
        assert_eq!(base, 2, "S->E distance is 2 steps along top row");

        let cheat_results = get_savings_count_with_cheats(&grid, 20);
        // We expect no shorter path than base, so no cheat results
        assert!(cheat_results.is_empty());
    }

    #[test]
    fn single_wall_direct_block() {
        let grid_chars = [
            "S#E",
            "...",
            "...",
        ];

        let grid: Vec<Vec<char>> = grid_chars.iter()
            .map(|row| row.chars().collect())
            .collect();
        let (start, end, walls) = find_start_end(&grid);

        let base = shortest_path_no_cheat(&grid, start, end, &walls).unwrap();
        let cheat_results = get_savings_count_with_cheats(&grid, 20);

         assert!(!cheat_results.is_empty(), "Should be at least one cheat crossing the wall.");
        let min_distance = cheat_results.values().min().unwrap();
        let saving = base - min_distance;
        assert!(saving > 0, "Expected a positive saving, found none");
    }
    #[test]
    fn partial_cheat_usage() {
        let grid_chars = [
            "S##....E",
            "........",
            "........",
            ".........",
        ];

        let grid: Vec<Vec<char>> = grid_chars.iter()
            .map(|row| row.chars().collect())
            .collect();
        let (start, end, walls) = find_start_end(&grid);
        let base = shortest_path_no_cheat(&grid, start, end, &walls).unwrap();

        let cheat_results = get_savings_count_with_cheats(&grid, 20);

         let best_distance = cheat_results.values().min().unwrap();
        let saving = base - best_distance;
        assert!(saving >= 2, "We expected at least 2 steps saved by partial cheat");
    }

    #[test]
    fn cheat_must_end_on_track() {
        let grid_chars = [
            "S######E",
        ];

        let grid: Vec<Vec<char>> = grid_chars.iter()
            .map(|row| row.chars().collect())
            .collect();

        let cheat_results =  get_savings_count_with_cheats(&grid, 20);
        assert!(!cheat_results.is_empty(), "We should be able to cross 6 walls with a 20-step cheat");
    }

    #[test]
    fn test_example_part2_savings() {
        let grid = load_grid("test_input.txt");
        let cheat_results = get_savings_count_with_cheats(&grid, 20);

        assert_eq!(cheat_results.get(&50).unwrap_or(&0), &32);
        assert_eq!(cheat_results.get(&52).unwrap_or(&0), &31);
        assert_eq!(cheat_results.get(&76).unwrap_or(&0), &3);
    }
}