mod parse;

fn main() {
    let (_remaining_input, instructions) =
        Instruction::parse_input(include_bytes!("../input")).expect("should parse everything");
    println!("q1: {}", q1(instructions));
}

fn q1_simulation(instructions: Vec<Instruction>) -> (Cpu, Vec<i64>) {
    let mut cpu = Cpu::default();
    let outputs = cpu.simulate(instructions);
    (cpu, outputs)
}

fn q1(instructions: Vec<Instruction>) -> RegisterVal {
    let values_over_time = q1_simulation(instructions).1;
    values_over_time
        .iter()
        .enumerate()
        .map(|(t, x)| (t + 1, x))
        .filter(|(t, _x)| t >= &20 && ((t - 20) % 40 == 0))
        .map(|(t, x)| {
            let signal_strength = (t as i64) * x;
            // eprintln!("t={t}, x={x}, signal={signal_strength}");
            (t, signal_strength)
        })
        .map(|(_t, x)| x)
        .sum()
}

type RegisterVal = i64;

#[derive(Clone, Debug)]
enum Instruction {
    Addx(RegisterVal),
    Noop,
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

struct Cpu {
    x: RegisterVal,
    in_progress: Option<(Instruction, usize)>,
    instructions_executed: usize,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x: 1,
            in_progress: Default::default(),
            instructions_executed: Default::default(),
        }
    }
}

impl Cpu {
    /// Outputs a vec showing the value of register x at each time.
    fn simulate(&mut self, mut instructions: Vec<Instruction>) -> Vec<RegisterVal> {
        instructions.reverse();
        let mut register_values = Vec::new();
        loop {
            register_values.push(self.x);

            match self.in_progress.take() {
                // Instruction is ready
                Some((instruction, 1)) => self.apply(instruction),

                // Instruction needs more time
                Some((instruction, ttl)) => self.in_progress = Some((instruction, ttl - 1)),

                // Get a new instruction
                None => match instructions.pop() {
                    Some(ins) => {
                        let num_cycles = ins.cycles();
                        if num_cycles > 1 {
                            // The instruction requires more cycles to complete.
                            self.in_progress = Some((ins, num_cycles - 1))
                        } else {
                            // The instruction can be executed now.
                            self.apply(ins)
                        }
                    }
                    // No more instructions left, program complete, so stop simulating.
                    None => break,
                },
            }
        }
        register_values
    }

    fn apply(&mut self, instruction: Instruction) {
        self.instructions_executed += 1;
        match instruction {
            Instruction::Addx(n) => self.x += n,
            Instruction::Noop => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1_normal() {
        let (_remaining_input, instructions) =
            Instruction::parse_input(include_bytes!("../example"))
                .expect("should parse everything");
        assert_eq!(13140, q1(instructions));
    }

    #[test]
    fn test_q1_tiny() {
        let (_remaining_input, instructions) =
            Instruction::parse_input(include_bytes!("../tiny")).expect("should parse everything");

        // At the start of the first cycle, the noop instruction begins execution.
        // During the first cycle, X is 1.
        // After the first cycle, the noop instruction finishes execution, doing nothing.

        // At the start of the second cycle, the addx 3 instruction begins execution.
        // During the second cycle, X is still 1.

        // During the third cycle, X is still 1.
        // After the third cycle, the addx 3 instruction finishes execution, setting X to 4.

        // At the start of the fourth cycle, the addx -5 instruction begins execution.
        // During the fourth cycle, X is still 4.

        // During the fifth cycle, X is still 4.
        // After the fifth cycle, the addx -5 instruction finishes execution, setting X to -1.

        assert_eq!(vec![1, 1, 1, 4, 4, -1], q1_simulation(instructions).1);
    }
}
