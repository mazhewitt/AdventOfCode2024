use std::collections::{HashMap, HashSet, VecDeque};
use glam::IVec2;

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

#[derive(Debug)]
struct CheatRecord {
    cheat_start: IVec2,
    cheat_end: IVec2,
    distance: i32,
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

/// Returns the BFS distance from `start` to `end` *without cheating*,
/// or None if no path exists.
fn shortest_path_no_cheat(
    grid: &[Vec<char>],
    start: IVec2,
    end: IVec2,
    walls: &HashSet<IVec2>,
) -> Option<i32> {
    let bounds = IVec2::new(grid[0].len() as i32, grid.len() as i32);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((pos, dist)) = queue.pop_front() {
        if pos == end {
            return Some(dist);
        }
        for &dir in DIRECTIONS.iter() {
            let next = pos + dir;
            if next.x >= 0
                && next.y >= 0
                && next.x < bounds.x
                && next.y < bounds.y
                && !walls.contains(&next)
                && !visited.contains(&next)
            {
                visited.insert(next);
                queue.push_back((next, dist + 1));
            }
        }
    }
    None
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

/// Perform a BFS that accounts for one 2-step cheat window.
/// Returns a map from (cheat_start, cheat_end) -> best distance achieving that cheat.
/// (If you just need to count how many saves >= 100, you can reduce or adapt.)
fn bfs_with_cheat(
    grid: &[Vec<char>],
    start: IVec2,
    end: IVec2,
    walls: &HashSet<IVec2>,
    base_distance: i32, // the non-cheating distance for S->E
) -> HashMap<(IVec2, IVec2), i32> {
    let bounds = IVec2::new(grid[0].len() as i32, grid.len() as i32);

    // For each (position, cheat_state), store the *minimum* distance
    // we've seen so far. If we come back at a bigger/equal distance, skip.
    let mut visited = HashMap::<BfsNode, i32>::new();

    // We'll record finishing cheats in this map: (start, end) -> best distance
    let mut cheat_results = HashMap::<(IVec2, IVec2), i32>::new();

    // Initial BFS state: at start, cheat not used, distance=0
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
            // We can't possibly find a shorter path than the base distance
            continue;
        }

        // If we've reached E, we can see if we ended in some cheat state
        // that has a distinct start-end pair. The puzzle only cares
        // if we used a cheat that actually saved time.
        if pos == end {
            match cheat_state {
                // We either never cheated or we ended the cheat:
                CheatState::UsedUp(cs, ce) => {
                    // Save the distance under that (start, end) pair:
                    let entry = cheat_results.entry((cs, ce)).or_insert(dist);
                    if dist < *entry {
                        *entry = dist;
                    }
                }
                // If we ended exactly on track with only 1 cheat step used, that’s also a valid cheat,
                // so you might consider bridging that scenario. But to keep it simpler,
                // we assume we forced the cheat to end if we used it at all.
                _ => {
                    // Possibly we never cheated, or we ended in InUse with leftover steps —
                    // then that’s not a valid “finished cheat,” or it’s no cheat at all.
                }
            }
            continue;
        }

        // Explore neighbors
        for &dir in DIRECTIONS.iter() {
            let next_pos = pos + dir;
            // Skip out-of-bounds
            if next_pos.x < 0
                || next_pos.y < 0
                || next_pos.x >= bounds.x
                || next_pos.y >= bounds.y
            {
                continue;
            }

            let next_is_wall = walls.contains(&next_pos);

            // We’ll produce the next cheat state + distance (always dist+1):
            let (next_state, next_dist) = match cheat_state {
                //------------------------------------------------------
                // 1) We haven't used the cheat yet (NotUsed)
                //------------------------------------------------------
                CheatState::NotUsed => {
                    if next_is_wall {
                        // We can start the cheat IF we step through 1 wall
                        // cheat now "in use," with 1 more cheat step left after this one
                        let next_cs = CheatState::InUse(1, pos);
                        (next_cs, dist + 1)
                    } else {
                        // Normal move on track, still not using cheat
                        (CheatState::NotUsed, dist + 1)
                    }
                }

                //------------------------------------------------------
                // 2) Currently using the cheat: InUse(steps_left, cheat_start)
                //------------------------------------------------------
                CheatState::InUse(steps_left, cheat_start) => {
                    if next_is_wall {
                        // If we want to keep going through a second wall:
                        if steps_left == 1 {
                            continue;
                        } else {
                            continue;
                        }
                    } else {
                        if steps_left == 1 {
                            let next_cs = CheatState::UsedUp(cheat_start, pos);
                            (next_cs, dist + 1)
                        } else {
                            (CheatState::InUse(2, cheat_start), dist + 1)
                        }
                    }
                }

                //------------------------------------------------------
                // 3) We have used up our cheat: UsedUp(cheat_start, cheat_end)
                //------------------------------------------------------
                CheatState::UsedUp(cs, ce) => {
                    // We can no longer go through walls
                    if next_is_wall {
                        // No move
                        continue;
                    } else {
                        // Move on track
                        (CheatState::UsedUp(cs, ce), dist + 1)
                    }
                }
            };

            // If we haven't visited (next_pos, next_state) or we found a shorter path, enqueue
            let next_node = BfsNode {
                position: next_pos,
                cheat_state: next_state,
            };
            if let Some(&best_dist_so_far) = visited.get(&next_node) {
                // We have seen this state before
                if best_dist_so_far <= next_dist {
                    // We have already a better or equal distance for that state, skip
                    continue;
                }
            }
            visited.insert(next_node, next_dist);
            queue.push_back((next_node, next_dist));
        }
    }

    // Now `cheat_results` has (start, end) -> best distance if cheat was used
    // We'll return them so you can group by saving = base_distance - best_distance
    cheat_results
}

fn get_savings_count_with_cheats(grid: &Vec<Vec<char>>) -> HashMap<i32, i32> {
    let (start, end, walls) = find_start_end(&grid);
    // 1) Find base distance with no cheating
    let base_distance = shortest_path_no_cheat(&grid, start, end, &walls)
        .expect("No path found from S to E without cheating!");

    // 2) BFS that accounts for 2-step cheat once
    let cheat_results_map = bfs_with_cheat(&grid, start, end, &walls, base_distance);

    // 3) Convert to “savings” map: saving => how many distinct (start, end) yield that saving?
    let mut savings_count: HashMap<i32, i32> = HashMap::new();
    for ((cheat_start, cheat_end), dist) in cheat_results_map.iter() {
        let save = base_distance - dist;
        // Filter out cases that don’t actually save time (<=0).
        if save > 0 {
            *savings_count.entry(save).or_insert(0) += 1;
        }
    }

    // 4) Finally, how many cheats save at least 100 picoseconds?
    let big_savers: i32 = savings_count
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
    let savings_map = get_savings_count_with_cheats(&grid);
    let big_savers = savings_map.iter().filter(|(save, _)| **save >= 100).count();
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
        let (start, end, walls) = find_start_end(&grid);
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

        let savings_count = get_savings_count_with_cheats(&grid);
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
    }


}