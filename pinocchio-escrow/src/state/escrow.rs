use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amount: [u8; 8],
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = core::mem::size_of::<Escrow>();

    pub fn load(escrow_account: &AccountInfo) -> Result<&mut Self, ProgramError> {
        let data = unsafe { escrow_account.borrow_mut_data_unchecked() };
        let escrow_state = bytemuck::try_from_bytes_mut::<Escrow>(data)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(escrow_state)
    }
}
