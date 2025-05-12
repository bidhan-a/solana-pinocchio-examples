use crate::instruction::{self, CounterInstruction};
use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// Define the program entrypoint.
program_entrypoint!(process_instruction);
// Do not allocate memory.
no_allocator!();
// Use the nostd panic handler.
nostd_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match CounterInstruction::try_from(ix_disc)? {
        CounterInstruction::Create => instruction::process_create(accounts, instruction_data),
        CounterInstruction::Increment => instruction::process_increment(accounts),
        CounterInstruction::Decrement => instruction::process_decrement(accounts),
        CounterInstruction::Delete => instruction::process_delete(accounts),
    }
}
