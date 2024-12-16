use std::cmp::PartialEq;
use std::collections::{HashSet, VecDeque};
use std::fs;



fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Error reading the file");
    let mut warehouse = Warehouse::from_str(&input_str, 1);
    while warehouse.move_robot() {

    }
    let gps = warehouse.calculate_gps_sum_part1();
    println!("GPS sum: {}", gps);
    
    let mut warehouse = Warehouse::from_str(&input_str, 2);
    while warehouse.move_robot() {
    }
    let gps = warehouse.calculate_gps_sum_part1();
    println!("GPS sum: {}", gps);
}


enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum ObjectType {
    Box,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Object {
    object_type: ObjectType,
    position: (usize, usize),
    width: usize,
}

impl Object {
    fn new(object_type: ObjectType, position: (usize, usize), width: usize) -> Self {
        Object {
            object_type,
            position,
            width,
        }
    }

    // Helper to get all occupied positions by the object
    fn occupied_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for w in 0..self.width {
            positions.push((self.position.0 + w, self.position.1));
        }
        positions
    }

    fn overlaps(&self, other: &Object) -> bool {
        if self.position.1 != other.position.1 {
            return false;
        }

        let self_start = self.position.0;
        let self_end = self_start + self.width; 
        let other_start = other.position.0;
        let other_end = other_start + other.width; 

        self_start < other_end && other_start < self_end
    }
    fn paint (&self, paper: &mut Vec<Vec<char>>) {
        for (x, y) in self.occupied_positions() {
            paper[y][x] = match self.object_type {
                ObjectType::Box => {
                    if self.width == 1 {
                        'O'
                    }
                    else {
                        // if x,y is self.position, then it is the leftmost position
                        if x == self.position.0 {
                            '['
                        }
                        else {
                            ']'
                        }
                    }
                },
                ObjectType::Wall => '#',
            };
        }
    }
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
    objects: HashSet<Object>,
    robot: Robot,
    width_multiplier: usize,
    size: (usize, usize),
}



impl Warehouse {
    fn new() -> Warehouse {
        Warehouse {
            objects: HashSet::new(),
            robot: Robot {
                position: (0,0),
                instructions: VecDeque::new(),
            },
            width_multiplier: 1,
            size: (0,0),
        }
    }

    fn add_object(&mut self, object: Object) {
        self.objects.insert(object);
    }

    fn add_robot(&mut self, robot: Robot) {
        self.robot = robot;
    }

    pub fn move_robot(&mut self) -> bool {
        let direction = self.robot.instructions.pop_front();
        if direction.is_none() {
            return false;
        }
        let direction = direction.unwrap();
        // Check if the robot can move in the given direction
        let new_position = self.compute_new_position(self.robot.position, &direction);
        // is there an object at the new position?
        let  object_at_new_position= Warehouse::get_object_at(&self.objects, new_position);
        if object_at_new_position.is_none() {
            self.robot.position = new_position;
            return true;
        }
        else if object_at_new_position.unwrap().object_type == ObjectType::Wall {
            return true;
        }
        else {
            if let Some(mut object_set) = self.get_moveable_set(object_at_new_position.unwrap(), &direction) {
                // Convert to vec to sort
                let mut objects_to_move: Vec<_> = object_set.into_iter().collect();
                
                // does the moveable set contain the a wall
                if objects_to_move.iter().any(|obj| obj.object_type == ObjectType::Wall) {
                    return true;
                }
                
                // Sort objects based on direction
                match direction {
                    Direction::Right => objects_to_move.sort_by(|a, b| b.position.0.cmp(&a.position.0)),
                    Direction::Left => objects_to_move.sort_by(|a, b| a.position.0.cmp(&b.position.0)),
                    Direction::Down => objects_to_move.sort_by(|a, b| b.position.1.cmp(&a.position.1)),
                    Direction::Up => objects_to_move.sort_by(|a, b| a.position.1.cmp(&b.position.1)),
                }

                // Now move them in the correct order
                for object in objects_to_move {
                    let new_position = self.compute_new_position(object.position, &direction);
                    self.objects.remove(&object);
                    self.add_object(Object::new(object.object_type, new_position, object.width));
                }

                self.robot.position = new_position;
            }
        }
        true
    }

    
    
    fn get_moveable_set(&self, object: &Object, direction: &Direction) -> Option<HashSet<Object>> {
        if object.object_type == ObjectType::Wall {
            let mut object_set =  HashSet::new();
            object_set.insert(object.clone());
            return Some(object_set)
        }
        let mut movables =  HashSet::new();
        //for each of my positions, check if there is an object at the new position
        let my_positions = object.occupied_positions();
        for pos in my_positions {
            let new_position = self.compute_new_position(pos, direction);
            let object_at_new_position = Warehouse::get_object_at(&self.objects, new_position);
            if object_at_new_position.is_some() {
                if object_at_new_position.unwrap() == object {
                    continue;
                }
               let movables_o = self.get_moveable_set(object_at_new_position.unwrap(), direction);
                if movables_o.is_some() {
                    movables_o.unwrap().iter().for_each(|o| {movables.insert(o.clone());});
                }
                else { 
                    return None;
                }
            }
        }
        movables.insert(object.clone());
        Some(movables)
    }


    fn compute_new_position(&self, position: (usize, usize), direction: &Direction) -> (usize, usize) {
        match direction {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
        }
    }

    fn get_object_at(objects: &HashSet<Object>, position: (usize, usize)) -> Option<&Object> {
        objects.iter().find(|obj| obj.occupied_positions().contains(&position))
    }

    fn from_str(input: &str, widith_multiplier: usize) -> Warehouse {
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
                                position: (x * widith_multiplier, y),
                                width: widith_multiplier,
                            });
                        },
                        'O' => {
                            warehouse.add_object(Object {
                                object_type: ObjectType::Box,
                                position: (x * widith_multiplier, y),
                                width: widith_multiplier,
                            });
                        },
                        '@' => {
                            robot.position = (x * widith_multiplier, y);
                        },
                        _ => {},
                    }
                    x += 1;
                }
                y += 1;
                warehouse.size = (std::cmp::max(warehouse.size.0, x)+widith_multiplier-1, std::cmp::max(warehouse.size.1, y));
            }
        }
        warehouse.add_robot(robot);
        warehouse.width_multiplier = widith_multiplier;
        warehouse
    }

    fn to_str(&self) -> String {
        let mut paper = vec![vec!['.'; self.size.0]; self.size.1];
        for obj in &self.objects {
            obj.paint(&mut paper);
        }
        paper[self.robot.position.1][self.robot.position.0] = '@';
        let mut output = String::new();
        for row in paper {
            output.push_str(&row.iter().collect::<String>());
            output.push('\n');
        }
        output
    }
    
    
    fn calculate_gps_sum_part1(&self) -> i32 {
        let mut sum = 0;
        for obj in &self.objects {
            if obj.object_type == ObjectType::Box {
                sum += (obj.position.0) as i32 + (obj.position.1*100) as i32;
            }
        }
        sum
    }
    fn calculate_gps_sum_part2(&self) -> i32 {
        let mut sum = 0;
        for obj in &self.objects {
            if obj.object_type == ObjectType::Box {
                // For the scaled-up boxes, the position of obj is the top-left (closest) edge of the box.
                // Thus the GPS coordinate can be calculated the same way:
                // GPS = 100 * (distance from top) + (distance from left)
                // Here, position.1 is the y-coordinate (distance from top)
                // and position.0 is the x-coordinate (distance from left).
                sum += (obj.position.0 as i32) + (obj.position.1 as i32 * 100);
            }
        }
        sum
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
        let warehouse = Warehouse::from_str(warehouse_str, 1);
        let output_warehouse_str = warehouse.to_str();
        let output_robot_str = warehouse.robot.to_str();
        let expected_warehouse_str = format!("{}\n{}", output_warehouse_str, output_robot_str);

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
        let mut warehouse = Warehouse::from_str(warehouse_str, 1);
        warehouse.move_robot();
        let output_warehouse_str = warehouse.to_str();
        let expected_warehouse_str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
";
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
########
";
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
        let mut warehouse = Warehouse::from_str(warehouse_str, 1);
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
########
";
        assert_eq!(output_warehouse_str, expected_warehouse_str);

    }

    #[test]
    fn test_move_right_with_oo() {
        let warehouse_str = "########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

>>vv<v>>v<<";
        let mut warehouse = Warehouse::from_str(warehouse_str, 1);
        warehouse.move_robot();
        let output_warehouse_str = warehouse.to_str();
        let expected_warehouse_str = "########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
";
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
##########
";
        let mut warehouse = Warehouse::from_str(&input_str, 1);
        while warehouse.move_robot() {
            println!("{}\n{}\n\n", warehouse.to_str(), warehouse.robot.to_str());
        }
        assert_eq!(warehouse.to_str(), output_str);
        let gps = warehouse.calculate_gps_sum_part1();
        assert_eq!(gps, 10092);
    }


    //// ------------------- Day 15 Part 2 ------------------- /////
    
    #[test]
    fn test_can_expand() {
        let warehouse_str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let warehouse = Warehouse::from_str(warehouse_str, 2);
        let expamded = warehouse.to_str();
        let expected = "##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############
";
        assert_eq!(expamded, expected);
    }
    #[test]
fn test_expanded_move_2() {
    let warehouse_str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
    let mut warehouse = Warehouse::from_str(warehouse_str, 2);
    warehouse.move_robot();
    warehouse.move_robot();
    let expamded = warehouse.to_str();
    let expected = "##############
##......##..##
##..........##
##...[][]...##
##....[].@..##
##..........##
##############
";
    assert_eq!(expamded, expected);
}

    #[test]
    fn test_expanded_move_3() {
        let warehouse_str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let mut warehouse = Warehouse::from_str(warehouse_str, 2);
        warehouse.move_robot();
        warehouse.move_robot();
        warehouse.move_robot();
        let expamded = warehouse.to_str();
        let expected = "##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.......@..##
##############
";
        assert_eq!(expamded, expected);
    }

    #[test]
    fn test_expanded_move_4() {
        let warehouse_str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let mut warehouse = Warehouse::from_str(warehouse_str, 2);
        warehouse.move_robot();
        warehouse.move_robot();
        warehouse.move_robot();
        warehouse.move_robot();
        let expamded = warehouse.to_str();
        let expected = "##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##......@...##
##############
";
        assert_eq!(expamded, expected);
    }

    #[test]
    fn test_expanded_move_5() {
        let warehouse_str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let mut warehouse = Warehouse::from_str(warehouse_str, 2);
        warehouse.move_robot();
        warehouse.move_robot();
        warehouse.move_robot();
        warehouse.move_robot();
        warehouse.move_robot();
        let expamded = warehouse.to_str();
        let expected = "##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##############
";
        assert_eq!(expamded, expected);
    }

    #[test]
    fn test_expanded_move_6() {
        let warehouse_str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let mut warehouse = Warehouse::from_str(warehouse_str, 2);
        warehouse.move_robot();
        warehouse.move_robot();
        warehouse.move_robot();
        warehouse.move_robot();
        warehouse.move_robot();
        println!("{}", warehouse.to_str());
        warehouse.move_robot();
        println!("{}", warehouse.to_str());
        let expamded = warehouse.to_str();
        let expected = "##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############
";
        assert_eq!(expamded, expected);
    }
    
    #[test]
    fn test_input_part2() {
        let input_str = fs::read_to_string("test_input.txt").expect("Error reading the file");
        let mut warehouse = Warehouse::from_str(&input_str, 2);
        while warehouse.move_robot() {
        }
        
        let expected = "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################
";
        assert_eq!(warehouse.to_str(), expected);

    }

}




