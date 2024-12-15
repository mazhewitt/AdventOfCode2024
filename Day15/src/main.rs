use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}


enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum ObjectType {
    Box,
    Wall,
}

struct Object {
    object_type: ObjectType,
    position: (usize, usize),
}

struct Robot {
    position: (usize, usize),
    instructions: Vec<Direction>,
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
                instructions: Vec::new(),
            }
        }
    }

    fn add_object(&mut self, object: Object) {
        self.objects.insert(object.position, object);
    }

    fn add_robot(&mut self, robot: Robot) {
        self.robot = robot;
    }

    fn move_robot(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.robot.position.1 += 1;
            },
            Direction::Down => {
                self.robot.position.1 -= 1;
            },
            Direction::Left => {
                self.robot.position.0 -= 1;
            },
            Direction::Right => {
                self.robot.position.0 += 1;
            },
        }
    }
    
    fn from_str(input: &str) -> Warehouse {
        let mut warehouse = Warehouse::new();
        let mut robot = Robot {
            position: (0,0),
            instructions: Vec::new(),
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
                    robot.instructions.push(direction);
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
}