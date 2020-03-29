#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    NOP,
    MOVE,
    LOADL,
    LOADI,
    LOADI_0,
    LOADI_1,
    LOADI_2,
    LOADI_3,
    LOADSYM,
    LOADNIL,
    LOADSELF,
    LOADT,
    LOADF,
    GETGV,
    SETGV,
    GETSV,
    SETSV,
    GETIV,
    SETIV,
    GETCV,
    SETCV,
    GETCONST,
    SETCONST,
    GETMCNST,
    SETMCNST,
    GETUPVAR,
    SETUPVAR,
    JMP,
    JMPIF,
    JMPNOT,
    ONERR,
    EXCEPT,
    RESCUE,
    POPERR,
    RAISE,
    EPUSH,
    EPOP,
    SENDV,
    SENDVB,
    SEND,
    SENDB,
    CALL,
    SUPER,
    ARGARY,
    ENTER,
    KARG,
    KARG2,
    RETURN,
    RETURN_BLK,
    BREAK,
    BLKPUSH,
    ADD,
    ADDI,
    SUB,
    SUBI,
    MUL,
    DIV,
    EQ,
    LT,
    LE,
    GT,
    GE,
    ARRAY,
    ARRAY2,
    ARYCAT,
    ARYPUSH,
    AREF,
    ASET,
    APOST,
    STRING,
    STRCAT,
    HASH,
    HASHADD,
    LAMBDA,
    BLOCK,
    METHOD,
    RANGE_INC,
    RANGE_EXC,
    OCLASS,
    CLASS,
    MODULE,
    EXEC,
    DEF,
    ALIAS,
    UNDEF,
    SCLASS,
    TCLASS,
    ERR,
    EXT1,
    EXT2,
    EXT3,
    STOP,
    MAX,
    INVALID = 255,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        if v < Opcode::MAX as u8 {
            unsafe { std::mem::transmute(v) }
        } else {
            Opcode::INVALID
        }
    }
}
impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::STOP);
        assert_eq!(instruction.opcode, Opcode::STOP);
    }
    #[test]
    fn test_u8_conversion() {
        // Basic tests
        assert_eq!(Opcode::NOP, Opcode::from(0));
        assert_eq!(Opcode::STOP, Opcode::from(91));
        assert_eq!(Opcode::INVALID, Opcode::from(92));
        assert_eq!(Opcode::INVALID, Opcode::from(200));
        // Roundtrip tests
        for v in 0..Opcode::MAX as u8 {
            assert_eq!(v, Opcode::from(v) as u8);
            assert_eq!(Opcode::from(v), Opcode::from(Opcode::from(v) as u8));
        }
    }
}
