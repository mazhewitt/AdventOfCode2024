use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Error reading the file");
    let mut warehouse = Warehouse::from_str(&input_str);
    while warehouse.move_robot() {

    }
    let gps = warehouse.calculate_gps_sum();
    println!("GPS sum: {}", gps);
}


enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum ObjectType {
    Box,
    Wall,
}

#[derive(Debug, PartialEq)]
struct Object {
    object_type: ObjectType,
    position: (usize, usize),
}

struct Robot {
    position: (usize, usize),
    instructions: VecDeque<Direction>,
}

impl Robot {
    fn to_str(&self) -> String {
        self.instructions
            .iter()
            .map(|direction| match direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            })
            .collect()
    }
}

struct Warehouse {
    objects: HashMap<(usize, usize),Object>,
    robot: Robot
}



impl Warehouse {
    fn new() -> Warehouse {
        Warehouse {
            objects: HashMap::new(),
            robot: Robot {
                position: (0,0),
                instructions: VecDeque::new(),
            }
        }
    }

    fn add_object(&mut self, object: Object) {
        self.objects.insert(object.position, object);
    }

    fn add_robot(&mut self, robot: Robot) {
        self.robot = robot;
    }

    pub fn move_robot(&mut self) -> bool {
        if let Some(direction) = self.robot.instructions.pop_front() {
            let new_position = self.compute_new_position(self.robot.position, &direction);

            if self.move_to(&direction, new_position) {
                self.robot.position = new_position;
            }
            return true;
        }
        false
    }

    fn move_to(&mut self, direction: &Direction, position: (usize, usize)) -> bool {
        if let Some(object) = self.objects.get(&position) {
            match object.object_type {
                ObjectType::Wall => return false,
                ObjectType::Box => {
                    // Check if the box can be pushed
                    let next_position = self.compute_new_position(position, direction);
                    if !self.move_to(direction, next_position){
                        return false;
                    }
                    else { // Move the box
                        self.move_object(direction, position);}
                }
            }
        }
        true
    }

    fn move_object(&mut self, direction: &Direction, position: (usize, usize)) {
        if let Some(mut object) = self.objects.remove(&position) {
            let new_position = self.compute_new_position(position, direction);
            object.position = new_position;
            self.objects.insert(new_position, object);
        }
    }

    fn compute_new_position(&self, position: (usize, usize), direction: &Direction) -> (usize, usize) {
        match direction {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
        }
    }

    fn from_str(input: &str) -> Warehouse {
        let mut warehouse = Warehouse::new();
        let mut robot = Robot {
            position: (0,0),
            instructions: VecDeque::new(),
        };
        let mut reading_movements = false;

        let mut y = 0;

        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                reading_movements = true;
                continue;
            }
            if reading_movements {
                for c in line.chars() {
                    let direction = match c {
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _ => panic!("Invalid direction"),
                    };
                    robot.instructions.push_back(direction);
                }
                continue;
            }
            else {
                let mut x = 0;
                for c in line.chars() {
                    match c {
                        '#' => {
                            warehouse.add_object(Object {
                                object_type: ObjectType::Wall,
                                position: (x, y),
                            });
                        },
                        'O' => {
                            warehouse.add_object(Object {
                                object_type: ObjectType::Box,
                                position: (x, y),
                            });
                        },
                        '@' => {
                            robot.position = (x, y);
                        },
                        _ => {},
                    }
                    x += 1;
                }
                y += 1;
            }
        }
        warehouse.add_robot(robot);
        warehouse
    }

    fn to_str(&self) -> String {
        // Determine the dimensions of the warehouse
        let max_x = self.objects.keys().map(|(x, _)| *x).max().unwrap_or(0);
        let max_y = self.objects.keys().map(|(_, y)| *y).max().unwrap_or(0);
        let robot_pos = self.robot.position;

        let mut grid = vec![vec!['.'; max_x + 1]; max_y + 1];

        // Place walls and boxes
        for (&(x, y), object) in &self.objects {
            grid[y][x] = match object.object_type {
                ObjectType::Wall => '#',
                ObjectType::Box => 'O',
            };
        }

        // Place the robot
        grid[robot_pos.1][robot_pos.0] = '@';

        // Convert the grid to a string
        let warehouse_map = grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");



        // Combine warehouse map and instructions with a blank line in between
        format!("{warehouse_map}")
    }

    pub fn calculate_gps_sum(&self) -> usize {
        self.objects
            .values()
            .filter_map(|object| {
                if let ObjectType::Box = object.object_type {
                    Some(100 * object.position.1 + object.position.0)
                } else {
                    None
                }
            })
            .sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warehouse_from_str() {
        let warehouse_str="########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<^<";
        let warehouse = Warehouse::from_str(warehouse_str);
        let output_warehouse_str = warehouse.to_str();
        let output_robot_str = warehouse.robot.to_str();
        let expected_warehouse_str = format!("{}\n\n{}", output_warehouse_str, output_robot_str);

        assert_eq!(warehouse_str, expected_warehouse_str);
    }

    #[test]
    fn test_move_robot() {
        let warehouse_str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<^<";
        let mut warehouse = Warehouse::from_str(warehouse_str);
        warehouse.move_robot();
        let output_warehouse_str = warehouse.to_str();
        let expected_warehouse_str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        assert_eq!(output_warehouse_str, expected_warehouse_str);
        warehouse.move_robot();
        let output_warehouse_str = warehouse.to_str();
        let expected_warehouse_str = "########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        assert_eq!(output_warehouse_str, expected_warehouse_str);
    }


    #[test]
    fn test_small_warehouse() {
        let warehouse_str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let mut warehouse = Warehouse::from_str(warehouse_str);
        while warehouse.move_robot() {
            println!("{}\n{}\n\n", warehouse.to_str(), warehouse.robot.to_str());
        }
        let output_warehouse_str = warehouse.to_str();
        let expected_warehouse_str = "########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########";
        assert_eq!(output_warehouse_str, expected_warehouse_str);

    }

    #[test]
    fn test_move_right_with_OO() {
        let warehouse_str = "########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

>>vv<v>>v<<";
        let mut warehouse = Warehouse::from_str(warehouse_str);
        warehouse.move_robot();
        let output_warehouse_str = warehouse.to_str();
        let expected_warehouse_str = "########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        assert_eq!(output_warehouse_str, expected_warehouse_str);
    }



    #[test]
    fn test_imput_example() {
        let input_str = fs::read_to_string("test_input.txt").expect("Error reading the file");
        let output_str = "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########";
        let mut warehouse = Warehouse::from_str(&input_str);
        while warehouse.move_robot() {
            println!("{}\n{}\n\n", warehouse.to_str(), warehouse.robot.to_str());
        }
        assert_eq!(warehouse.to_str(), output_str);
        let gps = warehouse.calculate_gps_sum();
        assert_eq!(gps, 10092);
    }

}