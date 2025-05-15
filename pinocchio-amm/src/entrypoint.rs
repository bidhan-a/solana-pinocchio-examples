use crate::instruction::{self, AMMInstruction};
use pinocchio::{
    account_info::AccountInfo, default_panic_handler, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// Define the program entrypoint.
program_entrypoint!(process_instruction);
// Do not allocate memory.
no_allocator!();
// Use the nostd panic handler.
default_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match AMMInstruction::try_from(ix_disc)? {
        AMMInstruction::Initialize => instruction::process_initialize(accounts, &instruction_data),
        AMMInstruction::Deposit => instruction::process_deposit(accounts, &instruction_data),
        AMMInstruction::Swap => instruction::process_swap(accounts, &instruction_data),
        AMMInstruction::Withdraw => instruction::process_withdraw(accounts, &instruction_data),
    }
}
