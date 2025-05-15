use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Config {
    pub seed: [u8; 8],
    pub authority: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: [u8; 2],
    pub config_bump: u8,
}

impl Config {
    pub const LEN: usize = core::mem::size_of::<Config>();

    pub fn load(config_account: &AccountInfo) -> Result<&mut Self, ProgramError> {
        let data = unsafe { config_account.borrow_mut_data_unchecked() };
        let config_state = bytemuck::try_from_bytes_mut::<Config>(data)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(config_state)
    }
}
