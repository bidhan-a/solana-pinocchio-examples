#![no_std]

use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    pubkey::Pubkey, ProgramResult,
};
use pinocchio_log::log;

pinocchio_pubkey::declare_id!("CipundEMT29zcHH1YbH2KsHLRVkEUV55PaJ9baKnLmUr");

// Define the program entrypoint.
program_entrypoint!(process_instruction);
// Do not allocate memory.
no_allocator!();
// Use the nostd panic handler.
nostd_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    log!("Hello, Solana!");
    Ok(())
}
