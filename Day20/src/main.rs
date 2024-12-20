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
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use glam::IVec2;
    use pathfinding::prelude::astar;
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
    fn find_walls_start_end(){
        let grid = load_grid("test_input.txt");
        let (start, end, walls) = find_start_end(&grid);
        assert_eq!(start, IVec2::new(1 , 3));
        assert_eq!(end,  IVec2::new(5, 7));
    }

    fn find_start_end(grid: &Vec<Vec<char>>) -> (IVec2, IVec2, HashSet<IVec2>) {
        let mut walls = HashSet::new();
        let mut start = IVec2::ZERO;
        let mut end =  IVec2::ZERO;
        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let pos = IVec2::new(x as i32, y as i32);
                match cell {
                    'S' => start = pos,
                    'E' => end = pos,
                    '#' => { walls.insert(pos); },
                    _ => {}
                }
            }
        }
        (start, end, walls)
    }

    #[test]
    fn test_shortest_path() {
        let grid = load_grid("test_input.txt");
        let (start, end, walls) = find_start_end(&grid);
        let path = shortest_path(&grid, start, end, &walls);
        assert_eq!(path, 10);
    }

    fn shortest_path(grid: &Vec<Vec<char>>, start: IVec2, end: IVec2, walls: &HashSet<IVec2>) -> usize {
        // use A* algorithm from pathfinding to find the shortest path
        let mut path = 0;


        path
    }

    const DIRECTIONS: [IVec2; 4] =
        [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

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
}