mod hello_world;

pub use hello_world::{hello_world, HelloWorldArgs};

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey,
};

use bytemuck::try_from_bytes;

use crate::instruction::HelloWorldInstructionDiscriminants;

#[inline]
pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    match HelloWorldInstructionDiscriminants::from(instruction_data[0]) {
        HelloWorldInstructionDiscriminants::HelloWorld => {
            msg!("HelloWorld");
            hello_world(
                accounts,
                try_from_bytes(instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?,
            )
        }
    }
}
