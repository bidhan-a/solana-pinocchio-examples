use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Counter {
    pub maker: Pubkey,
    pub count: [u8; 8],
    pub bump: u8,
}

impl Counter {
    pub const LEN: usize = core::mem::size_of::<Counter>();

    pub fn load(counter_account: &AccountInfo) -> Result<&mut Self, ProgramError> {
        let data = unsafe { counter_account.borrow_mut_data_unchecked() };
        let counter_state = bytemuck::try_from_bytes_mut::<Counter>(data)
            .map_err(|_| ProgramError::AccountBorrowFailed)?;
        Ok(counter_state)
    }
}
