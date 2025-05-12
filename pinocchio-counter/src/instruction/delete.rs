use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::state::Counter;

pub fn process_delete(accounts: &[AccountInfo]) -> ProgramResult {
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

    // Close counter account.
    unsafe {
        *maker.borrow_mut_lamports_unchecked() += *counter.borrow_lamports_unchecked();
        *counter.borrow_mut_lamports_unchecked() = 0
    };

    counter.close()?;

    Ok(())
}
