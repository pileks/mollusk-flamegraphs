use num_derive::FromPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum HelloWorldError {
    #[error("Invalid instruction")]
    InvalidInstruction,
}

impl From<HelloWorldError> for ProgramError {
    fn from(e: HelloWorldError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
