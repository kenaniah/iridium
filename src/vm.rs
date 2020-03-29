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
    pub fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() { break; }
            match self.decode_opcode() {
                Opcode::HLT => {
                    println!("HLT encountered");
                    return;
                },
                op @ _ => {
                    println!("Unrecognized opcode {:?} found. Terminating.", op);
                    return;
                }
            }
        }
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
    fn test_opcode_hlt(){
        let mut test_vm = VM::new();
        test_vm.program = vec![0,0,0,0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_opcode_igl(){
        let mut test_vm = VM::new();
        test_vm.program = vec![200,0,0,0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
}
