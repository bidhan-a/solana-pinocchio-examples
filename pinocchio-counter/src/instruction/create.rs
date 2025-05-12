use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::{constants::COUNTER_SEED, state::Counter};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CreateInstructionData {
    pub count: [u8; 8],
    pub bump: u8,
}

impl CreateInstructionData {
    pub const LEN: usize = core::mem::size_of::<CreateInstructionData>();
}

pub fn process_create(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, counter, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let instruction_data = bytemuck::try_from_bytes::<CreateInstructionData>(&data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Validate counter account.
    let counter_pda = pubkey::create_program_address(
        &[
            COUNTER_SEED.as_bytes(),
            maker.key().as_ref(),
            &[instruction_data.bump as u8],
        ],
        &crate::ID,
    )?;
    if counter.key() != &counter_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // Create counter account.
    pinocchio_system::instructions::CreateAccount {
        from: maker,
        to: counter,
        space: Counter::LEN as u64,
        lamports: Rent::get()?.minimum_balance(Counter::LEN),
        owner: &crate::ID,
    }
    .invoke()?;

    // Initialize counter account.
    let counter_state = Counter::load(counter)?;
    counter_state.maker = *maker.key();
    counter_state.count = instruction_data.count;
    counter_state.bump = instruction_data.bump;

    Ok(())
}
