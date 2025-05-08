extern crate alloc;
use alloc::vec;

use mollusk_svm::result::{Check, ProgramResult};
use mollusk_svm::Mollusk;
use pinocchio_hello_world::ID;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/pinocchio_hello_world");
    mollusk
}

#[test]
fn test_hello_world() {
    let mollusk = mollusk();

    // We can ignore accounts and instructions for this program.
    let ix_accounts = vec![];
    let ix_data = vec![];
    let instruction = Instruction::new_with_bytes(PROGRAM, &ix_data, ix_accounts);
    let tx_accounts = &vec![];

    let hello_world_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);
    assert!(hello_world_res.program_result == ProgramResult::Success);
}
