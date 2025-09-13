use bytemuck::{Pod, Zeroable};
use shank::ShankType;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg};

#[repr(C)]
#[derive(Pod, Zeroable, PartialEq, Eq, Debug, Clone, Copy, ShankType)]
pub struct HelloWorldArgs {
    #[skip]
    pub discriminator: u8,
}

pub fn hello_world<'a>(_accounts: &'a [AccountInfo<'a>], _args: &HelloWorldArgs) -> ProgramResult {
    msg!("Hello, World!");
    Ok(())
}

#[cfg(test)]
mod tests {
    fn system_account_with_lamports(lamports: u64) -> Account {
        Account::new(lamports, 0, &solana_sdk_ids::system_program::id())
    }

    use super::*;
    use bytemuck::bytes_of;
    use mollusk_svm::{result::Check, Mollusk};
    use solana_account::Account;
    use solana_program::instruction::{AccountMeta, Instruction};

    use solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
    #[test]
    fn test_hello_world() {
        let mollusk = Mollusk::new(
            &crate::ID,
            "hello_world_program",
        );

        let accounts = vec![(SYSTEM_PROGRAM_ID, system_account_with_lamports(0))];
        let args = HelloWorldArgs { discriminator: 0 };
        let data = bytes_of(&args);

        let instruction =
            Instruction::new_with_bytes(crate::ID, &data, vec![AccountMeta::new(SYSTEM_PROGRAM_ID, false)]);

        let checks = vec![Check::success()];

        mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
    }

    #[test]
    fn test_hello_world_multiple() {
        let mollusk = Mollusk::new(
            &crate::ID,
            "hello_world_program",
        );

        let accounts = vec![(SYSTEM_PROGRAM_ID, system_account_with_lamports(0))];
        let args = HelloWorldArgs { discriminator: 0 };
        let data = bytes_of(&args);

        let instruction =
            Instruction::new_with_bytes(crate::ID, &data, vec![AccountMeta::new(SYSTEM_PROGRAM_ID, false)]);

        let checks = vec![Check::success()];

        mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
        mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
        mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
    }
}
