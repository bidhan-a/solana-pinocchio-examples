use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_token::state::TokenAccount;

use crate::{constants::ESCROW_SEED, state::Escrow};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct MakeInstructionData {
    pub deposit_amount: [u8; 8],
    pub receive_amount: [u8; 8],
    pub bump: u8,
}

impl MakeInstructionData {
    pub const LEN: usize = core::mem::size_of::<MakeInstructionData>();
}

pub fn process_make(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, mint_a, mint_b, maker_ata_a, vault, escrow, _system_program, _token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let instruction_data = bytemuck::try_from_bytes::<MakeInstructionData>(&data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Validate escrow account.
    let escrow_pda = pubkey::create_program_address(
        &[
            ESCROW_SEED.as_bytes(),
            maker.key().as_ref(),
            &[instruction_data.bump as u8],
        ],
        &crate::ID,
    )?;
    if escrow.key() != &escrow_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate vault owner.
    assert!(TokenAccount::from_account_info(vault).unwrap().owner() == escrow.key());

    // Create escrow account.
    pinocchio_system::instructions::CreateAccount {
        from: maker,
        to: escrow,
        space: Escrow::LEN as u64,
        lamports: Rent::get()?.minimum_balance(Escrow::LEN),
        owner: &crate::ID,
    }
    .invoke()?;

    // Initialize escrow account.
    Escrow::initialize(
        escrow,
        *maker.key(),
        *mint_a.key(),
        *mint_b.key(),
        instruction_data.receive_amount,
        instruction_data.bump as u8,
    )?;

    // Transfer tokens to vault.
    pinocchio_token::instructions::Transfer {
        from: maker_ata_a,
        to: vault,
        authority: maker,
        amount: u64::from_le_bytes(instruction_data.deposit_amount),
    }
    .invoke()?;

    Ok(())
}
