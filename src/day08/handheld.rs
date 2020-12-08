use thiserror::Error;

use instruction::Instruction;

#[allow(clippy::empty_enum)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("can not parse instruction from line: {0}")]
    InstructionParse(instruction::Error),

    #[error("instruction overflow")]
    InstructionOverflow,

    #[error("found already executed instruction")]
    DuplicateInstructionFound,
}

#[derive(Debug)]
pub struct Handheld {
    pub accumulator: isize,

    pub instruction_pointer: usize,
    pub instructions: Vec<Instruction>,

    pub executed_instructions: Vec<usize>,
}

impl std::str::FromStr for Handheld {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Error::InstructionParse)?;

        Ok(Self {
            accumulator: 0,

            instruction_pointer: 0,
            instructions,

            executed_instructions: Vec::new(),
        })
    }
}

impl Handheld {
    pub fn run(self) -> Result<Self, Error> {
        let mut local = self;

        loop {
            if let Err(err) = local.step() {
                match err {
                    Error::DuplicateInstructionFound => break,

                    err => return Err(err),
                }
            }
        }

        Ok(local)
    }

    pub fn step(&mut self) -> Result<(), Error> {
        let instruction = self
            .instructions
            .get(self.instruction_pointer)
            .ok_or(Error::InstructionOverflow)?;

        if self
            .executed_instructions
            .contains(&self.instruction_pointer)
        {
            return Err(Error::DuplicateInstructionFound);
        }

        self.executed_instructions.push(self.instruction_pointer);

        match instruction {
            Instruction::Nop => self.instruction_pointer += 1,
            Instruction::Acc(amount) => {
                self.accumulator += amount;
                self.instruction_pointer += 1;
            }

            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_wrap)]
            Instruction::Jmp(amount) => {
                self.instruction_pointer = (self.instruction_pointer as isize + amount) as usize
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod run {
        #[test]
        fn input_example_1() {
            const INPUT: &str = include_str!("input_example1.txt");
            let expected = 5;

            let handlheld = INPUT.parse::<super::Handheld>().expect("invalid input");
            let got = handlheld.run().expect("run failure").accumulator;

            assert_eq!(expected, got);
        }
    }

    mod step {
        #[test]
        fn input_example_1_single_step() {
            const INPUT: &str = include_str!("input_example1.txt");
            let expected_instruction_pointer = 1;
            let expected_accumulator = 0;
            let expected_executed_instructions = vec![0];

            let mut got = INPUT.parse::<super::Handheld>().expect("invalid input");

            got.step().expect("step failure");

            let got_instruction_pointer = got.instruction_pointer;
            let got_accumulator = got.accumulator;
            let got_executed_instructions = got.executed_instructions;

            assert_eq!(expected_instruction_pointer, got_instruction_pointer);
            assert_eq!(expected_accumulator, got_accumulator);
            assert_eq!(expected_executed_instructions, got_executed_instructions);
        }

        #[test]
        fn input_example_1_two_step() {
            const INPUT: &str = include_str!("input_example1.txt");
            let expected_instruction_pointer = 2;
            let expected_accumulator = 1;
            let expected_executed_instructions = vec![0, 1];

            let mut got = INPUT.parse::<super::Handheld>().expect("invalid input");

            got.step().expect("step failure");
            got.step().expect("step failure");

            let got_instruction_pointer = got.instruction_pointer;
            let got_accumulator = got.accumulator;
            let got_executed_instructions = got.executed_instructions;

            assert_eq!(expected_instruction_pointer, got_instruction_pointer);
            assert_eq!(expected_accumulator, got_accumulator);
            assert_eq!(expected_executed_instructions, got_executed_instructions);
        }
    }
}

mod instruction {
    use thiserror::Error;

    #[allow(clippy::empty_enum)]
    #[derive(Debug, Error)]
    pub enum Error {
        #[error("invalid instruction found while parsing: {0}")]
        Instruction(String),

        #[error("invalid argument for accumulator instruction found: {0}")]
        AccArg(std::num::ParseIntError),

        #[error("invalid argument for jump instruction found: {0}")]
        JmpArg(std::num::ParseIntError),
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum Instruction {
        /// Increase or decrease the accumulator of the handlheld by the isize.
        Acc(isize),

        /// Update the instruction_pointer to a new value relative to the jmp
        /// instruction_pointer by the given isize.
        Jmp(isize),

        /// Do nothing
        Nop,
    }

    impl std::str::FromStr for Instruction {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let split = s.split_whitespace().collect::<Vec<_>>();

            match split.as_slice() {
                ["nop", _] => Ok(Self::Nop),

                ["acc", num] => Ok(Self::Acc(
                    num.trim_start_matches('+').parse().map_err(Error::AccArg)?,
                )),

                ["jmp", num] => Ok(Self::Jmp(
                    num.trim_start_matches('+').parse().map_err(Error::JmpArg)?,
                )),

                _ => Err(Error::Instruction(s.to_string())),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod from_str {
            #[test]
            fn nop() {
                const INPUT: &str = "nop +0";
                let expected = super::Instruction::Nop;
                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn acc_plus() {
                const INPUT: &str = "acc +1";
                let expected = super::Instruction::Acc(1);
                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn acc_minus() {
                const INPUT: &str = "acc -1";
                let expected = super::Instruction::Acc(-1);
                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn jmp_plus() {
                const INPUT: &str = "jmp +1";
                let expected = super::Instruction::Jmp(1);
                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn jmp_minus() {
                const INPUT: &str = "jmp -1";
                let expected = super::Instruction::Jmp(-1);
                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn input_example_1() {
                use super::Instruction::{
                    Acc,
                    Jmp,
                    Nop,
                };

                const INPUT: &str = include_str!("input_example1.txt");
                let expected = vec![
                    Nop,
                    Acc(1),
                    Jmp(4),
                    Acc(3),
                    Jmp(-3),
                    Acc(-99),
                    Acc(1),
                    Jmp(-4),
                    Acc(6),
                ];
                let got = INPUT
                    .lines()
                    .map(|line| line.parse().expect("invalid input"))
                    .collect::<Vec<_>>();

                assert_eq!(expected, got)
            }
        }
    }
}
