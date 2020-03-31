use crate::opcode::{Opcode, OpcodeArgs, OpcodeArity, U24};

#[derive(Debug, PartialEq)]
/// Respresents a single instruction to be executed within a Virtual Machine
/// Instructions consist of an opcode and up to 3 arguments.
pub struct Instruction {
    opcode: Opcode,
    args: OpcodeArgs,
}

/// Tracks the state of a Virtual Machine
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
}

impl VM {
    /// Creates a new Virtual Machine instance
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
        }
    }
    /// Checks whether the progam counter has reached the end of the program (there are no more bytes to read)
    fn eof(&self) -> bool {
        self.eof_with_offset(0)
    }
    /// Checks whether the progam counter would reach the end of the program given the requested offset
    /// # Panics
    /// Panics if the offset causes the program counter to underflow
    fn eof_with_offset(&self, offset: isize) -> bool {
        if offset < 0 {
            self.pc - (offset.wrapping_abs() as usize) >= self.program.len()
        } else {
            self.pc + (offset as usize) >= self.program.len()
        }
    }
    fn decode_error(&self, opcode: Opcode, arity: OpcodeArity) -> Result<Instruction, String> {
        Err(format!("Could to decode arguments for opcode {:?} - {} argument(s) needed, but bytecode reached EOF.", opcode, arity.argc))
    }
    /// Decodes an instruction and advances the program counter accordingly
    fn decode_instruction(&mut self) -> Result<Instruction, String> {
        let mut op_ext: Option<Opcode> = None;

        // First, check for an opcode that extends arguments
        let opcode = match self.decode_opcode() {
            op @ Some(Opcode::EXT1) | op @ Some(Opcode::EXT2) | op @ Some(Opcode::EXT3) => {
                op_ext = op;
                self.decode_opcode()
            }
            op @ _ => op,
        };

        // Next, attempt to build an instruction
        if let Some(opcode) = opcode {
            // Determine the arguments needed for this opcode
            let mut arity = opcode.arity();
            // Adjust for any extended arguments
            match op_ext {
                Some(Opcode::EXT1) => {
                    if arity.arg1_size == 8 {
                        arity.arg1_size = 16;
                    }
                }
                Some(Opcode::EXT2) => {
                    if arity.arg2_size == 8 {
                        arity.arg2_size = 16;
                    }
                }
                Some(Opcode::EXT3) => {
                    if arity.arg1_size == 8 {
                        arity.arg1_size = 16;
                    }
                    if arity.arg2_size == 8 {
                        arity.arg2_size = 16;
                    }
                }
                _ => {}
            }
            // Associate the instruction's arguments
            let args = match arity {
                // No args
                OpcodeArity { argc: 0, .. } => OpcodeArgs::None,
                // Single arg
                OpcodeArity {
                    argc: 1,
                    arg1_size: 8,
                    arg1_signed: false,
                    ..
                } => {
                    if let Some(arg1) = self.next_8_bits() {
                        OpcodeArgs::U8(arg1)
                    } else {
                        return self.decode_error(opcode, arity);
                    }
                }
                OpcodeArity {
                    argc: 1,
                    arg1_size: 16,
                    arg1_signed: false,
                    ..
                } => {
                    if let Some(arg1) = self.next_16_bits() {
                        OpcodeArgs::U16(arg1)
                    } else {
                        return self.decode_error(opcode, arity);
                    }
                }
                OpcodeArity {
                    argc: 1,
                    arg1_size: 16,
                    arg1_signed: true,
                    ..
                } => {
                    if let Some(arg1) = self.next_16_bits() {
                        OpcodeArgs::I16(arg1 as i16)
                    } else {
                        return self.decode_error(opcode, arity);
                    }
                }
                OpcodeArity {
                    argc: 1,
                    arg1_size: 24,
                    arg1_signed: false,
                    ..
                } => {
                    if let Some(arg1) = self.next_24_bits() {
                        OpcodeArgs::U24(arg1)
                    } else {
                        return self.decode_error(opcode, arity);
                    }
                }
                // Two args
                OpcodeArity {
                    argc: 2,
                    arg1_size: 8,
                    arg1_signed: false,
                    arg2_size: 8,
                    arg2_signed: false,
                    ..
                } => {
                    if let (Some(arg1), Some(arg2)) = (self.next_8_bits(), self.next_8_bits()) {
                        OpcodeArgs::U8U8(arg1, arg2)
                    } else {
                        return self.decode_error(opcode, arity);
                    }
                }
                OpcodeArity {
                    argc: 2,
                    arg1_size: 8,
                    arg1_signed: false,
                    arg2_size: 8,
                    arg2_signed: true,
                    ..
                } => {
                    if let (Some(arg1), Some(arg2)) = (self.next_8_bits(), self.next_8_bits()) {
                        OpcodeArgs::U8I8(arg1, arg2 as i8)
                    } else {
                        return self.decode_error(opcode, arity);
                    }
                }
                OpcodeArity {
                    argc: 2,
                    arg1_size: 8,
                    arg1_signed: false,
                    arg2_size: 16,
                    arg2_signed: false,
                    ..
                } => {
                    if let (Some(arg1), Some(arg2)) = (self.next_8_bits(), self.next_16_bits()) {
                        OpcodeArgs::U8U16(arg1, arg2)
                    } else {
                        return self.decode_error(opcode, arity);
                    }
                }
                OpcodeArity {
                    argc: 2,
                    arg1_size: 8,
                    arg1_signed: false,
                    arg2_size: 16,
                    arg2_signed: true,
                    ..
                } => {
                    if let (Some(arg1), Some(arg2)) = (self.next_8_bits(), self.next_16_bits()) {
                        OpcodeArgs::U8I16(arg1, arg2 as i16)
                    } else {
                        return self.decode_error(opcode, arity);
                    }
                }
                OpcodeArity {
                    argc: 2,
                    arg1_size: 16,
                    arg1_signed: false,
                    arg2_size: 16,
                    arg2_signed: false,
                    ..
                } => {
                    if let (Some(arg1), Some(arg2)) = (self.next_16_bits(), self.next_16_bits()) {
                        OpcodeArgs::U16U16(arg1, arg2)
                    } else {
                        return self.decode_error(opcode, arity);
                    }
                }
                // Three args
                OpcodeArity {
                    argc: 3,
                    arg1_size: 8,
                    arg1_signed: false,
                    arg2_size: 8,
                    arg2_signed: false,
                    arg3_size: 8,
                    arg3_signed: false,
                } => {
                    if let (Some(arg1), Some(arg2), Some(arg3)) =
                        (self.next_8_bits(), self.next_8_bits(), self.next_8_bits())
                    {
                        OpcodeArgs::U8U8U8(arg1, arg2, arg3)
                    } else {
                        return self.decode_error(opcode, arity);
                    }
                }
                // Invalid args
                _ => return Err(format!(
                    "Could not decode arguments for opcode {:?} - arity could not be determined.",
                    opcode
                )),
            };
            Ok(Instruction {
                opcode: opcode,
                args: args,
            })
        } else {
            Err(
                "Could not decode the next instruction. End of program has been reached."
                    .to_owned(),
            )
        }
    }
    fn decode_opcode(&mut self) -> Option<Opcode> {
        if let Some(opcode) = self.next_8_bits() {
            Some(Opcode::from(opcode))
        } else {
            None
        }
    }
    fn next_8_bits(&mut self) -> Option<u8> {
        if self.eof() {
            return None;
        }
        let result = self.program[self.pc];
        self.pc += 1;
        Some(result)
    }
    fn next_16_bits(&mut self) -> Option<u16> {
        if self.eof_with_offset(1) {
            return None;
        }
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        Some(result)
    }
    fn next_24_bits(&mut self) -> Option<U24> {
        if self.eof_with_offset(2) {
            return None;
        }
        let result = (
            self.program[self.pc] as u8,
            self.program[self.pc + 1] as u8,
            self.program[self.pc + 2] as u8,
        );
        self.pc += 3;
        Some(result)
    }
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }
    pub fn run(&mut self) {
        while self.execute_instruction() {}
    }
    pub fn execute_instruction(&mut self) -> bool {
        if let Ok(instruction) = self.decode_instruction() {
            match instruction.opcode {
                Opcode::STOP => {
                    println!("Halt encountered.");
                    return false;
                }
                op @ Opcode::LOADI => match instruction.args {
                    OpcodeArgs::U8I16(a, b) => {
                        println!("{}, {}", a, b);
                        self.registers[a as usize] = b as i32;
                    }
                    _ => {
                        println!(
                            "Unrecognized arguments {:?} for opcode {:?} found. Terminating.",
                            instruction.args, op
                        );
                        return false;
                    }
                },
                op @ _ => {
                    println!("Unrecognized opcode {:?} found. Terminating.", op);
                    return false;
                }
            }
        } else {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }
    #[test]
    fn test_empty_program() {
        let mut test_vm = VM::new();
        test_vm.program = vec![];
        test_vm.run();
    }
    #[test]
    fn test_opcode_stop() {
        let mut test_vm = VM::new();
        test_vm.program = vec![Opcode::STOP as u8, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![Opcode::EXT2 as u8, Opcode::LOADI as u8, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }
    #[test]
    fn test_opcode_invalid() {
        let mut test_vm = VM::new();
        test_vm.program = vec![200, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_eof() {
        let mut test_vm = VM::new();
        test_vm.program = vec![];
        assert_eq!(test_vm.eof(), true);
        test_vm.program = vec![0];
        assert_eq!(test_vm.eof(), false);
        test_vm.program = vec![0, 0, 0, 0, 0, 0];
        for v in 0..10 {
            test_vm.pc = v;
            assert_eq!(test_vm.eof(), v > 5);
        }
    }
    #[test]
    fn test_of_with_offset() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 0, 0, 0, 0];
        test_vm.pc = 2;
        assert_eq!(test_vm.eof_with_offset(0), false);
        assert_eq!(test_vm.eof_with_offset(3), false);
        assert_eq!(test_vm.eof_with_offset(4), true);
        assert_eq!(test_vm.eof_with_offset(8), true);
        test_vm.pc = 10;
        assert_eq!(test_vm.eof_with_offset(0), true);
        assert_eq!(test_vm.eof_with_offset(-5), false);
        assert_eq!(test_vm.eof_with_offset(-10), false);
    }
}
