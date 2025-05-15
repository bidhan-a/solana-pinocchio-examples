use pinocchio::program_error::ProgramError;

#[derive(Clone, PartialEq)]
pub enum CustomError {
    InvalidDeposit,
    InvalidSwap,
    InvalidWithdrawal,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        Self::Custom(e as u32)
    }
}
