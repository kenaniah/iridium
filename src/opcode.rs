#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// Defines the operations that are allowed within a virtual machine
///
/// # Opcode arguments
/// Opcodes within the Ruby virtual machine can take 0 - 3 arguments. Arguments can be either
/// 8-bit, 16-bit, or 24-bit. They can be both signed or unsigned (with the exception of 24-bit
/// arguments, which are always unsigned).
///
/// Ruby has 3 modifier opcodes that can adjust the size of the arguments to the next opcode
/// encountered. These modifier opcodes are:
/// * `EXT1` - modifies the next opcode's first argument to be 16-bit
/// * `EXT2` - modifies the next opcode's second argument to be 16-bit
/// * `EXT3` - modifies both of the next opcode's first _and_ second arguments to be 16-bit
///
/// The number and nature of the arguments for an opcode can be determined by calling its
/// [arity()](#method.arity) method.
///
/// # Opcode casting
/// For convenience, opcodes can be safely cast to and from `u8`, but the
/// [INVALID](#variant.INVALID) opcode will be returned when casting a `u8` value that either
/// does not have a corresponding valid opcode or resolves to the [MAX](#variant.MAX) opcode,
/// which is simply a marker to detect the end of valid opcodes within the enum.
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

#[derive(Default)]
pub struct OpcodeArity {
    pub argc: u8,
    pub arg1_size: u8,
    pub arg1_signed: bool,
    pub arg2_size: u8,
    pub arg2_signed: bool,
    pub arg3_size: u8,
    pub arg3_signed: bool,
}

impl Opcode {
    pub fn arity(&self) -> OpcodeArity {
        match self {
            // No args
            Self::NOP
            | Self::EXT1
            | Self::EXT2
            | Self::EXT3
            | Self::STOP
            | Self::MAX
            | Self::INVALID => OpcodeArity {
                argc: 0,
                ..OpcodeArity::default()
            },
            // u8
            Self::LOADI_0
            | Self::LOADI_1
            | Self::LOADI_2
            | Self::LOADI_3
            | Self::LOADNIL
            | Self::LOADSELF
            | Self::LOADT
            | Self::LOADF
            | Self::EXCEPT
            | Self::POPERR
            | Self::RAISE
            | Self::EPUSH
            | Self::EPOP
            | Self::CALL
            | Self::RETURN
            | Self::RETURN_BLK
            | Self::BREAK
            | Self::ARYCAT
            | Self::ARYPUSH
            | Self::STRCAT
            | Self::RANGE_INC
            | Self::RANGE_EXC
            | Self::OCLASS
            | Self::ALIAS
            | Self::SCLASS
            | Self::TCLASS
            | Self::ERR => OpcodeArity {
                argc: 1,
                arg1_size: 8,
                ..OpcodeArity::default()
            },
            // u16
            Self::JMP => OpcodeArity {
                argc: 1,
                arg1_size: 16,
                ..OpcodeArity::default()
            },
            // i16
            Self::ONERR => OpcodeArity {
                argc: 1,
                arg1_size: 16,
                arg1_signed: true,
                ..OpcodeArity::default()
            },
            // u24
            Self::ENTER => OpcodeArity {
                argc: 1,
                arg1_size: 24,
                ..OpcodeArity::default()
            },
            // u8, u8
            Self::MOVE
            | Self::LOADL
            | Self::LOADSYM
            | Self::GETGV
            | Self::SETGV
            | Self::GETSV
            | Self::SETSV
            | Self::GETIV
            | Self::SETIV
            | Self::GETCV
            | Self::SETCV
            | Self::GETCONST
            | Self::SETCONST
            | Self::GETMCNST
            | Self::SETMCNST
            | Self::RESCUE
            | Self::SENDV
            | Self::SENDVB
            | Self::SUPER
            | Self::KARG
            | Self::KARG2
            | Self::ADD
            | Self::SUB
            | Self::SUBI
            | Self::MUL
            | Self::DIV
            | Self::EQ
            | Self::LT
            | Self::LE
            | Self::GT
            | Self::GE
            | Self::ARRAY
            | Self::ARRAY2
            | Self::AREF
            | Self::ASET
            | Self::APOST
            | Self::STRING
            | Self::HASH
            | Self::HASHADD
            | Self::LAMBDA
            | Self::BLOCK
            | Self::METHOD
            | Self::CLASS
            | Self::MODULE
            | Self::EXEC
            | Self::DEF
            | Self::UNDEF => OpcodeArity {
                argc: 2,
                arg1_size: 8,
                arg2_size: 8,
                ..OpcodeArity::default()
            },
            // u8, i8
            Self::LOADI => OpcodeArity {
                argc: 2,
                arg1_size: 8,
                arg2_size: 8,
                arg2_signed: true,
                ..OpcodeArity::default()
            },
            // u8, u16
            Self::JMPIF | Self::JMPNOT | Self::ARGARY | Self::BLKPUSH => OpcodeArity {
                argc: 2,
                arg1_size: 8,
                arg2_size: 16,
                ..OpcodeArity::default()
            },
            // u8, u8, u8
            Self::GETUPVAR | Self::SETUPVAR | Self::SEND | Self::SENDB | Self::ADDI => {
                OpcodeArity {
                    argc: 3,
                    arg1_size: 8,
                    arg2_size: 8,
                    arg3_size: 8,
                    ..OpcodeArity::default()
                }
            }
        }
    }
}

pub type U24 = (u8, u8, u8);

#[derive(Debug, PartialEq)]
pub enum OpcodeArgs {
    // Zero arguments
    None,
    // One argument
    U8(u8),
    U16(u16),
    I16(i16),
    U24(U24),
    // Two arguments
    U8U8(u8, u8),
    U8I8(u8, i8),
    U8U16(u8, u16),
    U8I16(u8, i16),
    U16U16(u16, u16),
    // Three arguments
    U8U8U8(u8, u8, u8),
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

#[cfg(test)]
mod tests {
    use super::*;
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
    #[test]
    fn test_arity() {
        // Basic tests
        assert_eq!(0, Opcode::NOP.arity().argc);
        assert_eq!(1, Opcode::LOADNIL.arity().argc);
        assert_eq!(8, Opcode::LOADNIL.arity().arg1_size);
        assert_eq!(2, Opcode::JMPIF.arity().argc);
        assert_eq!(8, Opcode::JMPIF.arity().arg1_size);
        assert_eq!(16, Opcode::JMPIF.arity().arg2_size);
        // Ensures argc matches argument sizes
        for v in 0..Opcode::MAX as u8 {
            match Opcode::from(v).arity() {
                OpcodeArity {
                    argc: 0,
                    arg1_size: 0,
                    arg2_size: 0,
                    arg3_size: 0,
                    ..
                } => {}
                OpcodeArity {
                    argc: 1,
                    arg1_size: 8,
                    arg2_size: 0,
                    arg3_size: 0,
                    ..
                } => {}
                OpcodeArity {
                    argc: 1,
                    arg1_size: 16,
                    arg2_size: 0,
                    arg3_size: 0,
                    ..
                } => {}
                OpcodeArity {
                    argc: 1,
                    arg1_size: 24,
                    arg2_size: 0,
                    arg3_size: 0,
                    ..
                } => {}
                OpcodeArity {
                    argc: 2,
                    arg1_size: 8,
                    arg2_size: 8,
                    arg3_size: 0,
                    ..
                } => {}
                OpcodeArity {
                    argc: 2,
                    arg1_size: 8,
                    arg2_size: 16,
                    arg3_size: 0,
                    ..
                } => {}
                OpcodeArity {
                    argc: 3,
                    arg1_size: 8,
                    arg2_size: 8,
                    arg3_size: 8,
                    ..
                } => {}
                _ => panic!(
                    "Opcode {:?} - arguments do not match allowed sizes",
                    Opcode::from(v)
                ),
            }
        }
    }
}
