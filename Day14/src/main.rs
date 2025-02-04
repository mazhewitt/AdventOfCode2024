use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Neg;
use num::integer::lcm;
use image::{ImageBuffer, Pixel, Rgba, RgbaImage};
use image::png::PngEncoder;
use regex::Regex;

fn main() {
    let robots = Robot::from_file("input.txt");
    let bathroom_width = 101;
    let bathroom_height = 103;
    let num_moves = 100;
    let safety_score = move_and_get_safety_score(&robots, bathroom_width, bathroom_height, num_moves);
    println!("Safety score: {}", safety_score);
    let christmas_tree_time = find_christmas_tree(&robots, bathroom_width, bathroom_height);
    println!("Christmas tree time: {}", christmas_tree_time);   
    let moved_robots = robots.iter().map(|robot| robot.move_robot((christmas_tree_time) as i32, bathroom_width, bathroom_height)).collect::<Vec<Robot>>();
    generate_png(&moved_robots, bathroom_width as u32, bathroom_height as u32, ".", christmas_tree_time as i32).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn from_str(robot: &str) -> Robot {

        let robot_regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let robot_captures = robot_regex.captures(&robot).unwrap();

        Robot {
            pos: (robot_captures[1].parse().unwrap(), robot_captures[2].parse().unwrap()),
            vel: (robot_captures[3].parse().unwrap(), robot_captures[4].parse().unwrap()),
        }
    }

    fn from_file(file: &str) ->Vec<Robot> {
        let mut robots = Vec::new();
        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            robots.push(Robot::from_str(&line.unwrap()));
        }
        robots
    }

    fn move_robot(&self, num_moves:i32, width: i32, height: i32) -> Robot{
        let new_x = (self.pos.0 + self.vel.0 * num_moves).rem_euclid(width);
        let new_y = (self.pos.1 + self.vel.1 * num_moves).rem_euclid(height);
        Robot {
            pos: (new_x, new_y),
            vel: self.vel,
        }
    }

    fn determine_quadrant(&self, width: i32, height: i32) -> Option<u8> {
        let mid_x = width / 2;
        let mid_y = height / 2;
        let x = self.pos.0;
        let y = self.pos.1;

        // Exclude robots on the middle lines
        if x == mid_x || y == mid_y {
            return None;
        }

        // Determine the quadrant
        if x < mid_x && y < mid_y {
            Some(1) // Top-left
        } else if x >= mid_x && y < mid_y {
            Some(2) // Top-right
        } else if x < mid_x && y >= mid_y {
            Some(3) // Bottom-left
        } else if x >= mid_x && y >= mid_y {
            Some(4) // Bottom-right
        } else {
            panic!("Invalid quadrant");
        }
    }
}




fn generate_png(robots: &[Robot], width: u32, height: u32, output_dir: &str, t: i32) -> Result<(), Box<dyn Error>> {
    let mut img: RgbaImage = ImageBuffer::from_pixel(width, height, Rgba([0, 0, 0, 255]));

    // Draw robots
    for robot in robots {
        let (x, y) = robot.pos;
        if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
            img.put_pixel(x as u32, y as u32, Rgba([255, 255, 255, 255])); // White pixel for robot
        }
    }

    // Create the file name and write the PNG
    let file_name = format!("{}/frame_{:04}.png", output_dir, t); // Zero-padded for sorting
    let output_file = File::create(file_name)?;
    let encoder = PngEncoder::new(output_file);
    encoder.encode(
        &img,
        width,
        height,
        Rgba::<u8>::color_type(),
    )?;
    Ok(())
}

fn move_and_get_safety_score(robots: &Vec<Robot>, bathroom_width: i32, bathroom_height: i32, num_moves: i32) -> i32 {
    let mut moved_robots = Vec::new();
    for robot in robots.iter() {
        moved_robots.push(robot.move_robot(num_moves, bathroom_width, bathroom_height));
    }

    let (q1, q2, q3, q4) = moved_robots.iter().fold((0, 0, 0, 0), |acc, robot| {
        match robot.determine_quadrant(bathroom_width, bathroom_height) {
            Some(1) => (acc.0 + 1, acc.1, acc.2, acc.3),
            Some(2) => (acc.0, acc.1 + 1, acc.2, acc.3),
            Some(3) => (acc.0, acc.1, acc.2 + 1, acc.3),
            Some(4) => (acc.0, acc.1, acc.2, acc.3 + 1),
            None => acc,
            _ => acc,
        }
    });

    let safety_score = q1 * q2 * q3 * q4;
    safety_score
}

fn calculate_entropy(robots: &[Robot], bin_size: i32) -> f64 {


    let mut counts = HashMap::new();
    for &robot in robots {
        let bin_x = robot.pos.0 / bin_size;
        let bin_y = robot.pos.1 / bin_size;
        *counts.entry((bin_x, bin_y)).or_insert(0) += 1;
    }
    let total = robots.len() as f64;

    // Compute probabilities and Shannon entropy
    counts
        .values()
        .map(|&count| {
            let p = count as f64 / total;
            p * p.log2()
        })
        .sum::<f64>()
        .neg()
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

// Compute the cycle length for all robots
fn calculate_cycle_length(robots: &[Robot], width: i32, height: i32) -> i64 {
    robots.iter().map(|robot| {
        // Horizontal cycle
        let tx = width / gcd(width, robot.vel.0);

        // Vertical cycle
        let ty = height / gcd(height, robot.vel.1);

        // Combine horizontal and vertical cycles for this robot
        lcm(tx as i64, ty as i64)
    }).fold(1, lcm)
}

fn find_christmas_tree(robots: &[Robot], bathroom_width: i32, bathroom_height: i32) -> i64 {
    let cycle_length = calculate_cycle_length(&robots, bathroom_width, bathroom_height);
    // for each time until cycle length, move the robots, then measure the entropy, get the time with the lowest entropy
    let mut min_entropy = std::f64::INFINITY;
    let mut min_entropy_time = 0;
    for t in 1..cycle_length {
        let moved_robots = robots.iter().map(|robot| robot.move_robot(t as i32, bathroom_width, bathroom_height)).collect::<Vec<Robot>>();
        let entropy = calculate_entropy(&moved_robots, 10);
        if entropy < min_entropy {
            min_entropy = entropy;
            min_entropy_time = t;
        }
    }
    min_entropy_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_from_str() {
        let robot = Robot::from_str("p=1,2 v=3,4");
        assert_eq!(robot.pos, (1, 2));
        assert_eq!(robot.vel, (3, 4));
    }

    #[test]
    fn test_robot_from_file() {
        let robots = Robot::from_file("test_input.txt");
        assert_eq!(robots.len(), 12);
    }

    #[test]
    fn test_move_robot() {
        let mut robot = Robot::from_str("p=2,4 v=2,-3");
        robot = robot.move_robot(5, 11, 7);
        assert_eq!(robot.pos, (1, 3));
    }

    #[test]
    fn test_find_quad(){
        let robot = Robot::from_str("p=1,1 v=2,-3");
        let quad = robot.determine_quadrant(11, 7);
        assert_eq!(quad, Some(1));
    }

    #[test]
    fn test_test_scenario() {
        let mut robots = Robot::from_file("test_input.txt");
        let bathroom_width = 11;
        let bathroom_height = 7;
        let num_moves = 100;


        let safety_score = move_and_get_safety_score(&mut robots, bathroom_width, bathroom_height, num_moves);
        assert_eq!(safety_score, 12);

    }

    #[test]
    fn test_calculate_cycle_length() {
        let robots = vec![
            Robot { pos: (0, 0), vel: (3, -3) },
            Robot { pos: (6, 3), vel: (-1, -3) },
        ];
        let bathroom_width = 101;
        let bathroom_height = 103;

        let cycle_length = calculate_cycle_length(&robots, bathroom_width, bathroom_height);
 
        let robot_0 = robots[0].move_robot(cycle_length as i32, bathroom_width, bathroom_height);
        let robot_1 = robots[1].move_robot(cycle_length as i32, bathroom_width, bathroom_height);
        assert_eq!(robot_0.pos, robots[0].pos);
        assert_eq!(robot_1.pos, robots[1].pos);
        let moved_robots = vec![robot_0, robot_1];
        assert_eq!(robots, moved_robots);
        
    }
    
    #[test]
    fn test_can_find_christmas_tree() {
        let mut robots = Robot::from_file("input.txt");
        let bathroom_width = 101;
        let bathroom_height = 103;

        let min_entropy_time = find_christmas_tree(&mut robots, bathroom_width, bathroom_height);
        assert_eq!(min_entropy_time, 6577);
    }

    
}