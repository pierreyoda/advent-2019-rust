use anyhow::{anyhow, Context, Result};

use advent_2019_common::run_with_scaffolding;
use num_traits::clamp;

type Scalar = usize;

type MemoryBank = Vec<Scalar>;

trait Memory {
    fn get_scalar_at(&self, index: usize) -> Result<Scalar>;
    fn set_scalar_at(&mut self, index: usize, value: Scalar) -> Result<()>;
}

impl Memory for MemoryBank {
    fn get_scalar_at(&self, index: usize) -> Result<Scalar> {
        self.get(index)
            .copied()
            .with_context(|| format!("Memory overflow on read at index {}", index))
    }

    fn set_scalar_at(&mut self, index: usize, value: Scalar) -> Result<()> {
        let reference = self
            .get_mut(index)
            .with_context(|| format!("Memory overflow on write at index {}", index))?;
        *reference = value;
        Ok(())
    }
}

struct VirtualMachine {
    program_counter: usize,
    memory: MemoryBank,
}

const OPERATION_CODE_ADD: Scalar = 1;
const OPERATION_CODE_MULTIPLY: Scalar = 2;
const OPERATION_CODE_HALT: Scalar = 99;

impl VirtualMachine {
    pub fn from_tape(inputs: Vec<usize>) -> Self {
        Self {
            memory: inputs,
            program_counter: 0,
        }
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
            Operation::decode(self.program_counter, current_step, &self.memory)?;
        if decoded_operation.apply(&mut self.memory)? {
            return Ok(true);
        }
        self.program_counter += 4;
        self.program_counter = clamp(self.program_counter, 0, self.memory.len() - 1);
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
enum Operation {
    /// Structure: (lhs_at, rhs_at, output_at)
    Add(usize, usize, usize),
    /// Structure: (lhs_at, rhs_at, output_at)
    Multiply(usize, usize, usize),
    /// Immediately halts the program.
    Halt,
}

impl Operation {
    pub fn code(&self) -> Scalar {
        match *self {
            Operation::Add(_, _, _) => OPERATION_CODE_ADD,
            Operation::Multiply(_, _, _) => OPERATION_CODE_MULTIPLY,
            Operation::Halt => OPERATION_CODE_HALT,
        }
    }

    pub fn decode(pc: usize, code: Scalar, memory: &dyn Memory) -> Result<Self> {
        let is_add = match code {
            OPERATION_CODE_ADD => true,
            OPERATION_CODE_MULTIPLY => false,
            OPERATION_CODE_HALT => return Ok(Operation::Halt),
            _ => return Err(anyhow!("Operation::decode unknown opcode {}", code)),
        };

        let (lhs_at, rhs_at, output_at) = (
            memory.get_scalar_at(pc + 1)?,
            memory.get_scalar_at(pc + 2)?,
            memory.get_scalar_at(pc + 3)?,
        );

        Ok((if is_add {
            Operation::Add
        } else {
            Operation::Multiply
        })(lhs_at, rhs_at, output_at))
    }

    /// Returns true for a `HALT` opcode.
    pub fn apply(&self, memory: &mut dyn Memory) -> Result<bool> {
        Ok(match *self {
            Operation::Add(lhs_at, rhs_at, output_at) => {
                let (lhs, rhs) = (memory.get_scalar_at(lhs_at)?, memory.get_scalar_at(rhs_at)?);
                let result = lhs + rhs;
                memory.set_scalar_at(output_at, result)?;
                false
            }
            Operation::Multiply(lhs_at, rhs_at, output_at) => {
                let (lhs, rhs) = (memory.get_scalar_at(lhs_at)?, memory.get_scalar_at(rhs_at)?);
                let result = lhs * rhs;
                memory.set_scalar_at(output_at, result)?;
                false
            }
            Operation::Halt => true,
        })
    }
}

fn main() -> Result<()> {
    run_with_scaffolding("day-2", b',', |mut tape| {
        *tape.get_mut(1).unwrap() = 12;
        *tape.get_mut(2).unwrap() = 2;

        let mut vm = VirtualMachine::from_tape(tape);
        vm.run()?;
        vm.memory_snapshot().get_scalar_at(0)
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::VirtualMachine;

    #[test]
    fn test_virtual_machine_stepping() {
        let tape_1 = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut vm = VirtualMachine::from_tape(tape_1.clone());
        assert_eq!(vm.memory_snapshot(), &tape_1);

        let tape_2 = vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        vm.step().unwrap();
        assert_eq!(vm.memory_snapshot(), &tape_2);

        let tape_3 = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        vm.step().unwrap();
        assert_eq!(vm.memory_snapshot(), &tape_3);
    }

    #[test]
    fn test_virtual_machine_running() {
        let mut vm1 = VirtualMachine::from_tape(vec![1, 0, 0, 0, 99]);
        vm1.run().unwrap();
        assert_eq!(vm1.memory_snapshot(), &vec![2, 0, 0, 0, 99]);

        let mut vm2 = VirtualMachine::from_tape(vec![2, 3, 0, 3, 99]);
        vm2.run().unwrap();
        assert_eq!(vm2.memory_snapshot(), &vec![2, 3, 0, 6, 99]);

        let mut vm3 = VirtualMachine::from_tape(vec![2, 4, 4, 5, 99, 0]);
        vm3.run().unwrap();
        assert_eq!(vm3.memory_snapshot(), &vec![2, 4, 4, 5, 99, 9801]);

        let mut vm4 = VirtualMachine::from_tape(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        vm4.run().unwrap();
        assert_eq!(vm4.memory_snapshot(), &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_virtual_machine_bug() {
        let tape_1 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut vm = VirtualMachine::from_tape(tape_1.clone());
        assert_eq!(vm.program_counter_snapshot(), 0);
        assert_eq!(vm.memory_snapshot(), &tape_1);

        vm.step().unwrap();
        assert_eq!(vm.program_counter_snapshot(), 4);
        assert_eq!(vm.memory_snapshot(), &vec![1, 1, 1, 4, 2, 5, 6, 0, 99]);

        vm.step().unwrap();
        assert_eq!(vm.program_counter_snapshot(), 8);
        assert_eq!(vm.memory_snapshot(), &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
