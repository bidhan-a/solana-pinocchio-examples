use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_token::state::{Mint, TokenAccount};

use crate::{constants::CONFIG_SEED, state::Config};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitializeInstructionData {
    pub seed: [u8; 8],
    pub fee: [u8; 2],
    pub config_bump: u8,
}

impl InitializeInstructionData {
    pub const LEN: usize = core::mem::size_of::<InitializeInstructionData>();
}

pub fn process_initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [authority, mint_x, mint_y, vault_x, vault_y, mint_lp, config, _system_program, _token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let instruction_data = bytemuck::try_from_bytes::<InitializeInstructionData>(&data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Validate config account.
    let config_pda = pubkey::create_program_address(
        &[
            CONFIG_SEED.as_bytes(),
            instruction_data.seed.as_ref(),
            &[instruction_data.config_bump as u8],
        ],
        &crate::ID,
    )?;
    if config.key() != &config_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate vault accounts.
    {
        let vault_x_account = TokenAccount::from_account_info(vault_x)?;
        let vault_y_account = TokenAccount::from_account_info(vault_y)?;
        if vault_x_account.owner() != config.key() {
            return Err(ProgramError::InvalidAccountData);
        }
        if vault_y_account.owner() != config.key() {
            return Err(ProgramError::InvalidAccountData);
        }
    }

    // Validate mint_lp account.
    {
        let mint_lp_account = Mint::from_account_info(mint_lp)?;
        if mint_lp_account.mint_authority() != Some(config.key()) {
            return Err(ProgramError::InvalidAccountData);
        }
    }

    // Create config account.
    pinocchio_system::instructions::CreateAccount {
        from: authority,
        to: config,
        space: Config::LEN as u64,
        lamports: Rent::get()?.minimum_balance(Config::LEN),
        owner: &crate::ID,
    }
    .invoke()?;

    // Initialize config account.
    let config_state = Config::load(config)?;
    config_state.seed = instruction_data.seed;
    config_state.authority = *authority.key();
    config_state.mint_x = *mint_x.key();
    config_state.mint_y = *mint_y.key();
    config_state.fee = instruction_data.fee;
    config_state.config_bump = instruction_data.config_bump;

    Ok(())
}
