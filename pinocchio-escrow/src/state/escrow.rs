use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

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

    pub fn load(escrow_account: &AccountInfo) -> Result<Self, ProgramError> {
        let mut data = escrow_account
            .try_borrow_mut_data()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        let escrow_state = *bytemuck::try_from_bytes_mut::<Escrow>(&mut data)
            .map_err(|_| ProgramError::AccountBorrowFailed)?;
        Ok(escrow_state)
    }

    pub fn initialize(
        escrow_account: &AccountInfo,
        maker: Pubkey,
        mint_a: Pubkey,
        mint_b: Pubkey,
        receive_amount: [u8; 8],
        bump: u8,
    ) -> ProgramResult {
        let mut escrow_state = Self::load(escrow_account)?;

        escrow_state.maker = maker;
        escrow_state.mint_a = mint_a;
        escrow_state.mint_b = mint_b;
        escrow_state.receive_amount = receive_amount;
        escrow_state.bump = bump;

        Ok(())
    }
}
