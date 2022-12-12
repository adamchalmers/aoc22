mod parse;

fn main() {
    let (_remaining_input, instructions) =
        Instruction::parse_input(include_bytes!("../input")).expect("should parse everything");
    println!("q1: {}", q1(instructions.clone()));
    q2(q1_simulation(instructions));
}

fn q1_simulation(instructions: Vec<Instruction>) -> impl Iterator<Item = RegisterVal> {
    Execution::new(Cpu::default(), instructions)
}

fn q2(mut sprite_positions: impl Iterator<Item = RegisterVal>) {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    for _y in 0..HEIGHT {
        for x in 0..WIDTH {
            let register_val = sprite_positions.next().unwrap();
            let pixel = if register_val.abs_diff(x as _) <= 1 {
                "#"
            } else {
                "."
            };
            print!("{pixel}");
        }
        println!()
    }
}

fn q1(instructions: Vec<Instruction>) -> RegisterVal {
    let values_over_time = q1_simulation(instructions);
    values_over_time
        .enumerate()
        .map(|(t, x)| (t + 1, x))
        // Consider the signal strength (the cycle number multiplied by the value of the X register)
        // during the 20th cycle and every 40 cycles after that.
        .filter(|(cycle_num, _x)| cycle_num >= &20 && ((cycle_num - 20) % 40 == 0))
        .map(|(cycle_num, x)| (cycle_num as RegisterVal) * x)
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
    fn apply(&mut self, instruction: Instruction) {
        self.instructions_executed += 1;
        match instruction {
            Instruction::Addx(n) => self.x += n,
            Instruction::Noop => {}
        }
    }
}

struct Execution {
    cpu: Cpu,
    instructions: Vec<Instruction>,
    done: bool,
}

impl Execution {
    fn new(cpu: Cpu, mut instructions: Vec<Instruction>) -> Self {
        instructions.reverse();
        Self {
            cpu,
            instructions,
            done: false,
        }
    }
}

impl Iterator for Execution {
    type Item = RegisterVal;

    /// Outputs a vec showing the value of register x at each time.
    fn next(&mut self) -> Option<RegisterVal> {
        let value_during_this_cycle = self.cpu.x;
        if self.done {
            return None;
        }
        match self.cpu.in_progress.take() {
            // Instruction is ready
            Some((instruction, 1)) => self.cpu.apply(instruction),

            // Instruction needs more time
            Some((instruction, ttl)) => self.cpu.in_progress = Some((instruction, ttl - 1)),

            // Get a new instruction
            None => match self.instructions.pop() {
                Some(ins) => {
                    let num_cycles = ins.cycles();
                    if num_cycles > 1 {
                        // The instruction requires more cycles to complete.
                        self.cpu.in_progress = Some((ins, num_cycles - 1))
                    } else {
                        // The instruction can be executed now.
                        self.cpu.apply(ins)
                    }
                }
                // No more instructions left, program complete, so stop simulating.
                None => {
                    self.done = true;
                }
            },
        }
        Some(value_during_this_cycle)
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
    fn test_q2() {
        let (_remaining_input, instructions) =
            Instruction::parse_input(include_bytes!("../example"))
                .expect("should parse everything");
        let it = q1_simulation(instructions);
        q2(it);
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

        assert_eq!(
            vec![1, 1, 1, 4, 4, -1],
            q1_simulation(instructions).collect::<Vec<_>>()
        );
    }
}
