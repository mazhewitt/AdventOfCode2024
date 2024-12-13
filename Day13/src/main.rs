use regex::Regex;

fn main() {
    let claw_machines = ClawMachine::from_file("input.txt");

    let total_minimum_cost: u128 = claw_machines.iter()
        .map(|machine| machine.calculate_minimum_cost().unwrap_or(0))
        .sum();

    println!("The smallest cost to win is: {}", total_minimum_cost);

    let large_offset: i128 = 10_000_000_000_000;
    let total_minimum_cost_with_offset: u128 = claw_machines.iter()
        .map(|machine| machine.calculate_minimum_cost_with_offset(large_offset).unwrap_or(0))
        .sum();

    println!("The smallest cost to win with added offset is: {}", total_minimum_cost_with_offset);
}

type Offset = (i128, i128);

struct ClawMachine {
    button_a: Offset,
    button_b: Offset,
    prize: Offset,
}

impl ClawMachine {
    fn new() -> ClawMachine {
        ClawMachine {
            button_a: (0, 0),
            button_b: (0, 0),
            prize: (0, 0),
        }
    }

    fn from_serialised(data: &str) -> Option<ClawMachine> {
        let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
        if let Some(cap) = re.captures(data) {
            let button_a_x: i128 = cap[1].parse().ok()?;
            let button_a_y: i128 = cap[2].parse().ok()?;
            let button_b_x: i128 = cap[3].parse().ok()?;
            let button_b_y: i128 = cap[4].parse().ok()?;
            let prize_x: i128 = cap[5].parse().ok()?;
            let prize_y: i128 = cap[6].parse().ok()?;

            Some(ClawMachine {
                button_a: (button_a_x, button_a_y),
                button_b: (button_b_x, button_b_y),
                prize: (prize_x, prize_y),
            })
        } else {
            None
        }
    }

    fn from_file(file: &str) -> Vec<ClawMachine> {
        let data = std::fs::read_to_string(file).expect("Failed to read the file");
        let mut claw_machines = Vec::new();
        let normalized_data = data.replace("\r\n", "\n");

        let blocks = normalized_data.split("\n\n");
        for block in blocks {
            if let Some(machine) = ClawMachine::from_serialised(block) {
                claw_machines.push(machine);
            } else {
                eprintln!("Failed to parse data block:\n{}", block);
            }
        }

        claw_machines
    }

    fn calculate_minimum_cost(&self) -> Option<u128>{
        self.calculate_minimum_cost_i(self.prize.0, self.prize.1, Some(100))
    }

    fn calculate_minimum_cost_with_offset(&self, large_offset: i128) -> Option<u128>{
        self.calculate_minimum_cost_i(self.prize.0+large_offset, self.prize.1+large_offset, None)
    }

    fn calculate_minimum_cost_i(&self, prize_x: i128, prize_y: i128, press_limit: Option<i128>) -> Option<u128> {
        let determinant = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;
        if determinant == 0 {
            return None;
        }

        let numerator_a = prize_x * self.button_b.1 - prize_y * self.button_b.0;
        if numerator_a % determinant != 0 {
            return None;
        }

        let press_a = numerator_a / determinant;

        let numerator_b = self.button_a.0 * prize_y - self.button_a.1 * prize_x;
        if numerator_b % determinant != 0 {
            return None;
        }

        let press_b = numerator_b / determinant;

        if press_a < 0 || press_b < 0 {
            return None;
        }

        if let Some(limit) = press_limit {
            if press_a > limit || press_b > limit {
                return None;
            }
        }

        Some((press_a * 3 + press_b) as u128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_serialised() {
        let data = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400";
        let machine = ClawMachine::from_serialised(data).unwrap();
        assert_eq!(machine.button_a, (94, 34));
        assert_eq!(machine.button_b, (22, 67));
        assert_eq!(machine.prize, (8400, 5400));
    }

    #[test]
    fn test_new() {
        let machine = ClawMachine::new();
        assert_eq!(machine.button_a, (0, 0));
        assert_eq!(machine.button_b, (0, 0));
        assert_eq!(machine.prize, (0, 0));
    }

    #[test]
    fn test_from_file() {
        let machines = ClawMachine::from_file("test_input.txt");
        assert_eq!(machines.len(), 4);
        assert_eq!(machines[0].button_a, (94, 34));
        assert_eq!(machines[0].button_b, (22, 67));
        assert_eq!(machines[0].prize, (8400, 5400));
    }

    #[test]
    fn test_calculate_minimum_cost() {
        let data = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400";
        let machine = ClawMachine::from_serialised(data).unwrap();
        let cost = machine.calculate_minimum_cost();
        assert_eq!(cost.unwrap(), 280);
    }

    #[test]
    fn test_no_solution() {
        let data = "Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176";
        let machine = ClawMachine::from_serialised(data).unwrap();
        let cost = machine.calculate_minimum_cost();
        assert_eq!(cost, None);
    }

    #[test]
    fn test_total_minimum_cost() {
        let machines = ClawMachine::from_file("test_input.txt");
        let total_cost: u128 = machines.iter()
            .map(|machine| machine.calculate_minimum_cost().unwrap_or(0))
            .sum();
        assert_eq!(total_cost, 480);
    }

    #[test]
    fn test_calculate_minimum_cost_with_addition() {
        let data = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400";
        let machine = ClawMachine::from_serialised(data).unwrap();
        let offset = 10_000_000_000_000;
        let cost = machine.calculate_minimum_cost_with_offset(offset);
        assert_eq!(cost, None);
    }
}
