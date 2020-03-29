use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
        }
    }
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }
    pub fn run(&mut self) {
        while self.execute_instruction() {}
    }
    pub fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::STOP => {
                println!("HLT encountered");
                return false;
            }
            Opcode::LOADI => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            }
            op @ _ => {
                println!("Unrecognized opcode {:?} found. Terminating.", op);
                return false;
            }
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
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![Opcode::STOP as u8, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![Opcode::LOADI as u8, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }
    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        test_vm.program = vec![200, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
}
