use shank::{ShankContext, ShankInstruction};

use crate::processor::HelloWorldArgs;

#[derive(Clone, Debug, PartialEq, Eq, ShankContext, ShankInstruction)]
pub enum HelloWorldInstruction {
    #[account(0, name = "system_program", desc = "The system program")]
    HelloWorld(HelloWorldArgs),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HelloWorldInstructionDiscriminants {
    HelloWorld = 0,
}

impl From<u8> for HelloWorldInstructionDiscriminants {
    fn from(value: u8) -> Self {
        match value {
            0 => HelloWorldInstructionDiscriminants::HelloWorld,
            _ => panic!("Invalid instruction discriminant"),
        }
    }
}
