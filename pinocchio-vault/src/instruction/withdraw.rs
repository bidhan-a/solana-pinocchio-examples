use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey, ProgramResult,
};
use pinocchio_system::instructions::Transfer;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct WithdrawInstructionData {
    pub bump: u8,
}

pub fn process_withdraw(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [withdraw_account, vault_account, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !withdraw_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vault_account.lamports() == 0 {
        return Err(ProgramError::InvalidAccountData);
    }

    let instruction_data = bytemuck::try_from_bytes::<WithdrawInstructionData>(&data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let bump = instruction_data.bump;

    let vault_pda =
        pubkey::create_program_address(&[b"p-vault", withdraw_account.key(), &[bump]], &crate::ID)?;
    if vault_account.key() != &vault_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // Create signers.
    let binding = [bump];
    let signer_seeds = [
        Seed::from(b"p-vault"),
        Seed::from(withdraw_account.key()),
        Seed::from(&binding),
    ];
    let signers = [Signer::from(&signer_seeds)];

    Transfer {
        from: vault_account,
        to: withdraw_account,
        lamports: vault_account.lamports(),
    }
    .invoke_signed(&signers)?;

    Ok(())
}
