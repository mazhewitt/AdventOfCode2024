use std::time::Instant;

#[derive(Debug)]
enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
    Invalid,
}

impl From<u8> for Instruction {
    fn from(opcode: u8) -> Self {
        match opcode {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => Self::Invalid,
        }
    }
}

struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    ip: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, program: Vec<u8>) -> Self {
        Self {
            register_a: a,
            register_b: b,
            register_c: c,
            ip: 0,
            program,
            output: Vec::new(),
        }
    }

    fn run(&mut self, stop_on_output: bool) {
        self.output.clear();
        while self.ip < self.program.len() {
            let opcode = Instruction::from(self.program[self.ip]);
            let operand = self.program[self.ip + 1];
            match self.execute(opcode, operand) {
                Ok(_) => (),
                Err(_) => break,
            }
            if stop_on_output && !self.output.is_empty() {
                break; // Stop immediately after output if flagged
            }
        }
    }

    fn execute(&mut self, opcode: Instruction, operand: u8) -> Result<(), ()> {
        match opcode {
            Instruction::ADV => self.register_a >>= self.get_combo_value(operand),
            Instruction::BXL => self.register_b ^= operand as usize,
            Instruction::BST => self.register_b = self.get_combo_value(operand) % 8,
            Instruction::JNZ => {
                if self.register_a != 0 {
                    self.ip = operand as usize;
                    return Ok(()); // Skip normal IP increment
                }
            }
            Instruction::BXC => self.register_b ^= self.register_c,
            Instruction::OUT => self.output.push((self.get_combo_value(operand) % 8) as u8),
            Instruction::BDV => self.register_b = self.register_a >> self.get_combo_value(operand),
            Instruction::CDV => self.register_c = self.register_a >> self.get_combo_value(operand),
            Instruction::Invalid => return Err(()), // Invalid opcode halts execution
        }
        self.ip += 2; 
        Ok(())
    }

    fn get_combo_value(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid combo operand"),
        }
    }
}

// Totally taken from Reddit - thanks people!
fn solve_for_a(target: &[u8], a: usize, program: &[u8]) -> Option<usize> {
    if target.is_empty() {
        return Some(a); // Base case
    }

    for t in 0..8 {
        let candidate_a = (a << 3) | t;
        let mut computer = Computer::new(candidate_a, 0, 0, program.to_vec());
        computer.run(true); // Stop after one output
        if computer.output.last().copied() == target.last().copied() {
            if let Some(result) = solve_for_a(&target[..target.len() - 1], candidate_a, program) {
                return Some(result);
            }
        }
    }
    None
}

fn main() {
    let start = Instant::now(); // Start the timer
    let program = vec![2, 4, 1, 6, 7, 5, 4, 4, 1, 7, 0, 3, 5, 5, 3, 0];

    // Run the program with initial values
    let mut computer = Computer::new(37293246, 0, 0, program.clone());
    computer.run(false);
    let output = computer
        .output
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");
    println!("Program output: {}", output);

    // Solve for the lowest positive A
    if let Some(a) = solve_for_a(&program, 0, &program) {
        println!("The lowest positive value of A is: {}", a);
    } else {
        println!("No solution found");
    }

    let duration = start.elapsed(); // Calculate elapsed time
    println!("Time taken: {:.2?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adv_instruction() {
        let program = vec![0, 2]; // ADV with combo operand 2 -> divide A by 4
        let mut computer = Computer::new(10, 0, 0, program);
        computer.run(false);
        assert_eq!(computer.register_a, 2); // 10 / 4 = 2
    }

    #[test]
    fn test_bst_instruction() {
        let program = vec![2, 6]; // BST with combo operand 6 -> C = 9
        let mut computer = Computer::new(0, 0, 9, program);
        computer.run(false);
        assert_eq!(computer.register_b, 1); // 9 % 8 = 1
    }

    #[test]
    fn test_out_instruction() {
        let program = vec![5, 4, 5, 5]; // OUT A, OUT B
        let mut computer = Computer::new(10, 3, 0, program);
        computer.run(false);
        assert_eq!(computer.output, vec![2, 3]); // A % 8 = 2, B % 8 = 3
    }

    #[test]
    fn test_bxc_instruction() {
        let program = vec![4, 0]; // BXC -> B ^= C
        let mut computer = Computer::new(0, 2024, 43690, program);
        computer.run(false);
        assert_eq!(computer.register_b, 44354); // 2024 ^ 43690 = 44354
    }


    #[test]
    fn test_jnz_instruction() {
        let program = vec![3, 4, 5, 0, 1, 0]; // JNZ 4 (jump), OUT A (should not execute)
        let mut computer = Computer::new(1, 0, 0, program);
        computer.run(false);
        assert!(computer.output.is_empty()); // Jump skips over the OUT instruction
    }

    #[test]
    fn test_program_output_sequence() {
        let program = vec![5, 0, 5, 1, 5, 4];
        let mut computer = Computer::new(10, 0, 0, program);
        computer.run(false);
        assert_eq!(computer.output, vec![0, 1, 2]);
    }

}
