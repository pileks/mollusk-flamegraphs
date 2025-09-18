use bytemuck::{from_bytes_mut, Pod, Zeroable};
use shank::ShankType;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg};

use crate::{instruction::accounts::HelloWorldAccounts, state::HelloWorldAccount};

#[repr(C)]
#[derive(Pod, Zeroable, PartialEq, Eq, Debug, Clone, Copy, ShankType)]
pub struct HelloWorldArgs {
    #[skip]
    pub discriminator: u8,
}

pub fn hello_world<'a>(accounts: &'a [AccountInfo<'a>], _args: &HelloWorldArgs) -> ProgramResult {
    let ctx = HelloWorldAccounts::context(accounts)?;

    let mut hello_world_account_data = ctx.accounts.hello_world_account.try_borrow_mut_data()?;
    let hello_world_account: &mut HelloWorldAccount = from_bytes_mut(&mut hello_world_account_data);

    let c = hello_world_account.a + hello_world_account.b;

    msg!("Hello, World! {}", c);

    Ok(())
}

#[cfg(test)]
mod tests {
    fn system_account_with_lamports(lamports: u64) -> Account {
        Account::new(lamports, 0, &solana_sdk_ids::system_program::id())
    }

    fn hello_world_account(pubkey: Pubkey, a: u64, b: u64) -> Account {
        let rent = rent::Rent::default();
        let minimum_balance = rent::Rent::minimum_balance(&rent, HelloWorldAccount::BASE_LEN);

        let mut account = Account::new(minimum_balance, HelloWorldAccount::BASE_LEN, &pubkey);

        let hello_world_account = HelloWorldAccount {
            key: Key::HelloWorldAccount as u8,
            a,
            b,
            padding: [0; 7],
        };

        account.data = bytemuck::bytes_of(&hello_world_account).to_vec();

        account
    }

    use crate::state::Key;

    use super::*;
    use bytemuck::bytes_of;
    use mollusk_svm::{result::Check, Mollusk};
    use solana_account::Account;
    use solana_program::instruction::{AccountMeta, Instruction};

    use solana_program::pubkey::Pubkey;
    use solana_program::rent;
    use solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
    #[test]
    fn test_hello_world() {
        let mollusk = Mollusk::new(&crate::ID, "hello_world_program");
        let hello_world_account_pubkey = Pubkey::new_unique();

        let accounts = vec![
            (SYSTEM_PROGRAM_ID, system_account_with_lamports(0)),
            (
                hello_world_account_pubkey,
                hello_world_account(hello_world_account_pubkey, 1, 2),
            ),
        ];

        let args = HelloWorldArgs { discriminator: 0 };
        let data = bytes_of(&args);

        let instruction = Instruction::new_with_bytes(
            crate::ID,
            &data,
            vec![
                AccountMeta::new(SYSTEM_PROGRAM_ID, false),
                AccountMeta::new(hello_world_account_pubkey, false),
            ],
        );

        let checks = vec![Check::success()];

        mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
    }

    #[test]
    fn test_hello_world_multiple() {
        let mollusk = Mollusk::new(&crate::ID, "hello_world_program");
        let hello_world_account_pubkey = Pubkey::new_unique();

        let accounts = vec![
            (SYSTEM_PROGRAM_ID, system_account_with_lamports(0)),
            (
                hello_world_account_pubkey,
                hello_world_account(hello_world_account_pubkey, 1, 2),
            ),
        ];

        let args = HelloWorldArgs { discriminator: 0 };
        let data = bytes_of(&args);

        let instruction = Instruction::new_with_bytes(
            crate::ID,
            &data,
            vec![
                AccountMeta::new(SYSTEM_PROGRAM_ID, false),
                AccountMeta::new(hello_world_account_pubkey, false),
            ],
        );

        let checks = vec![Check::success()];

        mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
        mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
        mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
    }
}
