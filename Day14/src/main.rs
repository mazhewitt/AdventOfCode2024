use std::fs::File;
use std::io::{BufRead, BufReader};
use image::{codecs::gif::GifEncoder, Delay, Frame, ImageBuffer, Rgba};
use image::gif::Repeat;
use regex::Regex;

fn main() {
    let mut robots = Robot::from_file("input.txt");
    let bathroom_width = 101;
    let bathroom_height = 103;
    let num_moves = 1000;
    let safety_score = move_and_get_safety_score(&mut robots, bathroom_width, bathroom_height, num_moves);
    println!("Safety score: {}", safety_score);
    generate_gif(&robots, bathroom_width as u32, bathroom_height as u32, num_moves, "output.gif");

}

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
        let new_x = (self.pos.0 + self.vel.0 * num_moves).rem_euclid(width );
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


fn generate_gif(robots: &[Robot], width: u32, height: u32, max_time: i32, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create output file and GIF encoder
    let output_file = File::create(output)?;
    let mut encoder = GifEncoder::new(output_file);
    encoder.set_repeat(Repeat::Infinite)?;

    for t in 0..=max_time {
        // Create an empty RGBA image
        let mut img = ImageBuffer::from_pixel(width, height, Rgba([0, 0, 0, 255]));

        // Draw robots
        for robot in robots {
            let (x, y) = robot.move_robot(t, width as i32, height as i32).pos;
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                img.put_pixel(x as u32, y as u32, Rgba([255, 255, 255, 255])); // White pixel for robot
            }
        }

        // Draw timestamp in top-right corner
        let timestamp = format!("{:02}", t);
        let start_x = width.saturating_sub(10) as i32; // Adjust as needed for text length
        let start_y = 2;
        for (i, ch) in timestamp.chars().enumerate() {
            let pixel_x = start_x + i as i32;
            if pixel_x >= 0 && pixel_x < width as i32 {
                img.put_pixel(pixel_x as u32, start_y as u32, Rgba([128, 128, 128, 255])); // Gray pixel for text
            }
        }

        let delay = Delay::from_numer_denom_ms(300, 1); // Delay in milliseconds (adjust as needed)
        let frame = Frame::from_parts(img, 0, 0, delay);
        encoder.encode_frame(frame)?;
    }

    Ok(())
}

fn move_and_get_safety_score(robots: &mut Vec<Robot>, bathroom_width: i32, bathroom_height: i32, num_moves: i32) -> i32 {
    for robot in robots.iter_mut() {
        *robot = robot.move_robot(num_moves, bathroom_width, bathroom_height);
    }

    let (q1, q2, q3, q4) = robots.iter().fold((0, 0, 0, 0), |acc, robot| {
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
        let mut robot = Robot::from_str("p=1,1 v=2,-3");
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

    
}