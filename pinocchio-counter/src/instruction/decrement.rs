use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::state::Counter;

pub fn process_decrement(accounts: &[AccountInfo]) -> ProgramResult {
    let [maker, counter, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate PDA is owned by the program.
    if !counter.is_owned_by(&crate::ID) {
        return Err(ProgramError::IllegalOwner);
    }

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let counter_state = Counter::load(counter)?;

    // Validate counter maker.
    if counter_state.maker.ne(maker.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Update counter state.
    let new_count = u64::from_le_bytes(counter_state.count)
        .checked_sub(1)
        .ok_or(ProgramError::InvalidInstructionData)?;
    counter_state.count = new_count.to_le_bytes();

    Ok(())
}
