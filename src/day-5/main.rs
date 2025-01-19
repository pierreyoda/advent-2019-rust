use anyhow::{anyhow, Context, Error, Result};

use advent_2019_common::{run_day_puzzle_solver, DayPuzzlePart};

type Scalar = usize;

#[derive(Clone, Debug)]
struct MemoryBank {
    tape: Vec<Scalar>,
}

impl TryFrom<String> for MemoryBank {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split = value.split(',');
        let mut tape = Vec::with_capacity(split.size_hint().0);
        for part in split {
            let part_scalar = part
                .parse()
                .with_context(|| format!("cannot parse tape scalar: {}", part))?;
            tape.push(part_scalar);
        }
        Ok(Self { tape })
    }
}

impl MemoryBank {
    pub fn new(tape: Vec<Scalar>) -> Self {
        Self { tape }
    }

    pub fn get_scalar_at(&self, index: usize) -> Result<Scalar> {
        self.tape
            .get(index)
            .with_context(|| format!("Memory overflow on read at index {}", index))
            .copied()
    }

    pub fn set_scalar_at(&mut self, index: usize, value: Scalar) -> Result<()> {
        let reference = self
            .tape
            .get_mut(index)
            .with_context(|| format!("Memory overflow on write at index {}", index))?;
        *reference = value;
        Ok(())
    }

    pub fn raw(&self) -> &Vec<Scalar> {
        &self.tape
    }
}

struct VirtualMachine {
    program_counter: usize,
    memory: MemoryBank,
}

const OPERATION_CODE_ADD: Scalar = 1;
const OPERATION_CODE_MULTIPLY: Scalar = 2;
const OPERATION_CODE_INPUT: Scalar = 3;
const OPERATION_CODE_OUTPUT: Scalar = 4;
const OPERATION_CODE_HALT: Scalar = 99;

impl VirtualMachine {
    pub fn from_tape(tape: &[Scalar]) -> Self {
        Self {
            memory: MemoryBank::new(tape.to_vec()),
            program_counter: 0,
        }
    }

    pub fn reset(&mut self, tape: &[Scalar]) {
        self.memory = MemoryBank::new(tape.to_vec());
        self.program_counter = 0;
    }

    pub fn run(&mut self) -> Result<()> {
        'vm: loop {
            if self.step()? {
                break 'vm;
            }
        }
        Ok(())
    }

    /// Returns true if the program must be halted.
    pub fn step(&mut self) -> Result<bool> {
        let current_step = self.memory.get_scalar_at(self.program_counter)?;
        let decoded_operation =
            Instruction::decode(self.program_counter, current_step, &self.memory)?;
        if decoded_operation.apply(&mut self.memory)? {
            return Ok(true);
        }
        self.program_counter += 4;
        Ok(false)
    }

    pub fn program_counter_snapshot(&self) -> usize {
        self.program_counter
    }

    pub fn memory_snapshot(&self) -> &MemoryBank {
        &self.memory
    }
}

#[derive(Debug)]
enum InstructionMode {
    /// Mode 0: instruction parameter interpreted as position.
    Position = 0,
    /// Mode 1: instruction parameter interpreted as value.
    Immediate = 1,
}

#[derive(Debug)]
enum Instruction {
    /// Structure: (lhs_at, rhs_at, output_at)
    Add(Scalar, Scalar, Scalar),
    /// Structure: (lhs_at, rhs_at, output_at)
    Multiply(Scalar, Scalar, Scalar),
    /// Takes a single integer as input and saves it to the position given by its only parameter
    Input(Scalar),
    /// Outputs the value of its only parameter.
    Output(Scalar),
    /// Immediately halts the program.
    Halt,
}

impl Instruction {
    pub fn code(&self) -> Scalar {
        match *self {
            Instruction::Add(_, _, _) => OPERATION_CODE_ADD,
            Instruction::Multiply(_, _, _) => OPERATION_CODE_MULTIPLY,
            Instruction::Input(_) => OPERATION_CODE_INPUT,
            Instruction::Output(_) => OPERATION_CODE_OUTPUT,
            Instruction::Halt => OPERATION_CODE_HALT,
        }
    }

    pub fn decode(pc: usize, code: Scalar, memory: &MemoryBank) -> Result<Self> {
        let code_chars: Vec<char> = code.to_string().chars().collect();
        let opcode_string = format!(
            "{}{}",
            code_chars[code_chars.len() - 2],
            code_chars[code_chars.len() - 1]
        );
        let opcode: Scalar = opcode_string
            .parse()
            .with_context(|| format!("opcode decoding: cannot parse value: {}", opcode_string))?;

        fn parse_instruction_mode(raw_mode: &str) -> Result<InstructionMode> {
            let parsed_mode: Scalar = raw_mode.parse().with_context(|| {
                format!(
                    "opcode instruction mode decoding: cannot parse value: {}",
                    raw_mode
                )
            })?;
            match parsed_mode {
                0 => Ok(InstructionMode::Position),
                1 => Ok(InstructionMode::Immediate),
                _ => Err(anyhow!(
                    "opcode instruction mode decoding: unknown mode: {}",
                    parsed_mode
                )),
            }
        }

        // let instruction_mode_1
        // let (instruction_mode_1, instruction_mode_2, instruction_mode_3) =
        //     (parse_instruction_mode(code_chars[0]));

        let is_add = match code {
            OPERATION_CODE_ADD => true,
            OPERATION_CODE_MULTIPLY => false,
            OPERATION_CODE_HALT => return Ok(Instruction::Halt),
            _ => return Err(anyhow!("Operation::decode unknown opcode {}", code)),
        };

        let (lhs_at, rhs_at, output_at) = (
            memory.get_scalar_at(pc + 1)?,
            memory.get_scalar_at(pc + 2)?,
            memory.get_scalar_at(pc + 3)?,
        );

        Ok((if is_add {
            Instruction::Add
        } else {
            Instruction::Multiply
        })(lhs_at, rhs_at, output_at))
    }

    /// Returns true for a `HALT` opcode.
    pub fn apply(&self, memory: &mut MemoryBank) -> Result<bool> {
        Ok(match *self {
            Instruction::Add(lhs_at, rhs_at, output_at) => {
                let (lhs, rhs) = (memory.get_scalar_at(lhs_at)?, memory.get_scalar_at(rhs_at)?);
                let result = lhs + rhs;
                memory.set_scalar_at(output_at, result)?;
                false
            }
            Instruction::Multiply(lhs_at, rhs_at, output_at) => {
                let (lhs, rhs) = (memory.get_scalar_at(lhs_at)?, memory.get_scalar_at(rhs_at)?);
                let result = lhs * rhs;
                memory.set_scalar_at(output_at, result)?;
                false
            }
            _ => false, // TODO:
            Instruction::Halt => true,
        })
    }
}

fn compute_solution_1(tape: &[Scalar]) -> Result<Scalar> {
    let mut vm = VirtualMachine::from_tape(tape);
    vm.run()?;
    vm.memory_snapshot().get_scalar_at(0)
}

const COMPUTE_SOLUTION_2_TARGET: Scalar = 19690720;

fn compute_solution_2(tape: &[Scalar]) -> Result<Scalar> {
    // brute-force
    let mut vm = VirtualMachine::from_tape(&[]);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut vm_tape = tape.to_vec();
            *vm_tape.get_mut(1).unwrap() = noun;
            *vm_tape.get_mut(2).unwrap() = verb;
            vm.reset(&vm_tape);
            vm.run()?;
            if vm.memory_snapshot().get_scalar_at(0)? == COMPUTE_SOLUTION_2_TARGET {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(anyhow!(
        "compute_solution_2: could not find solution in problem space"
    ))
}

fn main() -> Result<()> {
    // Part 1
    let _ = run_day_puzzle_solver(2, DayPuzzlePart::One, b'\n', |input: Vec<MemoryBank>| {
        let mut memory_bank = input[0].clone();
        memory_bank.set_scalar_at(1, 12)?;
        memory_bank.set_scalar_at(2, 2)?;
        Ok(compute_solution_1(memory_bank.raw()))
    })?;

    // Part 2
    let _ = run_day_puzzle_solver(2, DayPuzzlePart::Two, b'\n', |input: Vec<MemoryBank>| {
        let memory_bank = input[0].clone();
        Ok(compute_solution_2(memory_bank.raw()))
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::VirtualMachine;

    #[test]
    fn test_day_5_virtual_machine_stepping() {
        let tape_1 = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut vm = VirtualMachine::from_tape(&tape_1);
        assert_eq!(vm.memory_snapshot().raw(), &tape_1);

        let tape_2 = [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        vm.step().unwrap();
        assert_eq!(vm.memory_snapshot().raw(), &tape_2);

        let tape_3 = [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        vm.step().unwrap();
        assert_eq!(vm.memory_snapshot().raw(), &tape_3);
    }

    #[test]
    fn test_day_5_virtual_machine_running() {
        let mut vm1 = VirtualMachine::from_tape(&[1, 0, 0, 0, 99]);
        vm1.run().unwrap();
        assert_eq!(vm1.memory_snapshot().raw(), &[2, 0, 0, 0, 99]);

        let mut vm2 = VirtualMachine::from_tape(&[2, 3, 0, 3, 99]);
        vm2.run().unwrap();
        assert_eq!(vm2.memory_snapshot().raw(), &[2, 3, 0, 6, 99]);

        let mut vm3 = VirtualMachine::from_tape(&[2, 4, 4, 5, 99, 0]);
        vm3.run().unwrap();
        assert_eq!(vm3.memory_snapshot().raw(), &[2, 4, 4, 5, 99, 9801]);

        let mut vm4 = VirtualMachine::from_tape(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        vm4.run().unwrap();
        assert_eq!(vm4.memory_snapshot().raw(), &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_day_5_virtual_machine_bug() {
        let tape_1 = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut vm = VirtualMachine::from_tape(&tape_1);
        assert_eq!(vm.program_counter_snapshot(), 0);
        assert_eq!(vm.memory_snapshot().raw(), &tape_1);

        vm.step().unwrap();
        assert_eq!(vm.program_counter_snapshot(), 4);
        assert_eq!(
            vm.memory_snapshot().raw(),
            &vec![1, 1, 1, 4, 2, 5, 6, 0, 99]
        );

        vm.step().unwrap();
        assert_eq!(vm.program_counter_snapshot(), 8);
        assert_eq!(
            vm.memory_snapshot().raw(),
            &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
