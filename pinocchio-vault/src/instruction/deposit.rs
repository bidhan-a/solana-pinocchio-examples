use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey, ProgramResult};
use pinocchio_system::instructions::Transfer;

use crate::constants::LAMPORTS_PER_SOL;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct DepositInstructionData {
    pub amount: [u8; 8],
    pub bump: u8,
}

impl DepositInstructionData {
    pub const LEN: usize = core::mem::size_of::<DepositInstructionData>();
}

pub fn process_deposit(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [deposit_account, vault_account, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !deposit_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let instruction_data = bytemuck::try_from_bytes::<DepositInstructionData>(&data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let bump = instruction_data.bump;
    let amount = u64::from_le_bytes(instruction_data.amount);

    let vault_pda =
        pubkey::create_program_address(&[b"p-vault", deposit_account.key(), &[bump]], &crate::ID)?;
    if vault_account.key() != &vault_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    Transfer {
        from: deposit_account,
        to: vault_account,
        lamports: amount * LAMPORTS_PER_SOL,
    }
    .invoke()?;

    Ok(())
}
