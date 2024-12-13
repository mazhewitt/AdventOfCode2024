use good_lp::{variables, variable, Solution, SolverModel, default_solver};
use regex::Regex;


fn main() {
    let claw_machines = ClawMachine::from_file("input.txt");

    let smallest_cost: usize = claw_machines.iter()
        .map(|claw_machine| claw_machine.smallest_cost_to_win().unwrap_or(0)).sum();

    println!("The smallest cost to win is: {}", smallest_cost);
}

type Offset = (usize, usize);

struct ClawMachine {
    button_a: Offset,
    button_b: Offset,
    prize: Offset,
}


impl ClawMachine {
    fn _new() -> ClawMachine {
        ClawMachine {
            button_a: (0, 0),
            button_b: (0, 0),
            prize: (0, 0),
        }
    }


    fn from_serialised(data: &str) -> Option<ClawMachine> {
        let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
        if let Some(cap) = re.captures(data) {
            let button_a_x: usize = cap[1].parse().ok()?;
            let button_a_y: usize = cap[2].parse().ok()?;
            let button_b_x: usize = cap[3].parse().ok()?;
            let button_b_y: usize = cap[4].parse().ok()?;
            let prize_x: usize = cap[5].parse().ok()?;
            let prize_y: usize = cap[6].parse().ok()?;

            Some(ClawMachine {
                button_a: (button_a_x, button_a_y),
                button_b: (button_b_x, button_b_y),
                prize: (prize_x, prize_y),
            })
        } else {
            None
        }
    }

    pub fn from_file(file: &str) -> Vec<ClawMachine> {
        let data = std::fs::read_to_string(file).expect("Failed to read the file");
        let mut claw_machines = Vec::new();
        let normalized_data = data.replace("\r\n", "\n");

        // Split data blocks by empty lines
        let blocks = normalized_data.split("\n\n");
        for block in blocks {
            if let Some(claw_machine) = ClawMachine::from_serialised(block) {
                claw_machines.push(claw_machine);
            } else {
                eprintln!("Failed to parse data block:\n{}", block);
            }
        }

        claw_machines
    }

    pub(crate) fn smallest_cost_to_win(&self) -> Option<usize> {
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (prize_x, prize_y) = self.prize;


        variables! {
        vars:
            0 <= x <= 100; // Number of times to press A
            0 <= y <= 100; // Number of times to press B
        }


        let solution = vars
            .minimise(3 * x + 1 * y) // Minimize the total cost
            .using(default_solver)
            .with((ax as f64) * x + (bx as f64) * y << prize_x as f64) // Align X
            .with((ay as f64) * x + (by as f64) * y << prize_y as f64) // Align Y
            .solve();

        match solution {
            Ok(result) => {
                let x_val = result.value(x) as i32;
                let y_val = result.value(y) as i32;
                let cost = 3 * x_val + 1 * y_val;
                Some(cost as usize)
            }
            Err(_) => None, // No solution found

        }
    }

}
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_claw_machine_from_serialised() {
            let serialised = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400";
            let claw_machine = ClawMachine::from_serialised(serialised).unwrap();
            assert_eq!(claw_machine.button_a, (94, 34));
            assert_eq!(claw_machine.button_b, (22, 67));
            assert_eq!(claw_machine.prize, (8400, 5400));
        }

        #[test]
        fn test_claw_machine_new() {
            let claw_machine = ClawMachine::_new();
            assert_eq!(claw_machine.button_a, (0, 0));
            assert_eq!(claw_machine.button_b, (0, 0));
            assert_eq!(claw_machine.prize, (0, 0));
        }

        #[test]
        fn test_can_load_claw_machines_from_file() {
            let claw_machines = ClawMachine::from_file("test_input.txt");
            assert_eq!(claw_machines.len(), 4);
            assert_eq!(claw_machines[0].button_a, (94, 34));
            assert_eq!(claw_machines[0].button_b, (22, 67));
            assert_eq!(claw_machines[0].prize, (8400, 5400));
        }

        #[test]
        fn test_can_find_smallest_number_to_win() {
            let serialised = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400";
            let claw_machine = ClawMachine::from_serialised(serialised).unwrap();
            let smallest_number = claw_machine.smallest_cost_to_win();
            assert_eq!(smallest_number.unwrap(), 280);
        }

        #[test]
        fn test_when_no_solution() {
            let serialised = "Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176";
            let claw_machine = ClawMachine::from_serialised(serialised).unwrap();
            let smallest_number = claw_machine.smallest_cost_to_win();
            assert_eq!(smallest_number, None);
        }

        #[test]
        fn test_can_find_smallest_number_to_win_for_multiple_machines() {
            let claw_machines = ClawMachine::from_file("test_input.txt");

            let smallest_cost: usize = claw_machines.iter()
                .map(|claw_machine| claw_machine.smallest_cost_to_win().unwrap_or(0)).sum();
            assert_eq!(smallest_cost, 480);
        }
    }